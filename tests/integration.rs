use libsql_orm::{
    Database, Filter, FilterOperator, MigrationBuilder, MigrationManager, Model, Pagination,
    QueryBuilder, SearchFilter, Sort, SortOrder,
};
use serde::{Deserialize, Serialize};
use std::sync::Once;

static LOGGER: Once = Once::new();

#[derive(Model, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[table_name("users")]
struct User {
    #[orm_column(type = "INTEGER PRIMARY KEY AUTOINCREMENT")]
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    #[orm_column(type = "INTEGER")]
    pub age: Option<i64>,
    #[orm_column(type = "REAL")]
    pub score: Option<f64>,
    #[orm_column(type = "INTEGER")]
    pub is_active: bool,
}

fn init_logger() {
    LOGGER.call_once(|| {
        let _ = env_logger::builder().is_test(true).try_init();
    });
}

async fn setup_db() -> libsql_orm::Result<Database> {
    init_logger();
    let db = Database::new_local(":memory:").await?;
    db.execute(&User::migration_sql(), vec![]).await?;
    Ok(db)
}

fn user(name: &str, email: &str, age: Option<i64>, score: Option<f64>, is_active: bool) -> User {
    User {
        id: None,
        name: name.to_string(),
        email: email.to_string(),
        age,
        score,
        is_active,
    }
}

async fn insert_and_get_real(db: &Database, user: &User) -> libsql_orm::Result<User> {
    let _ = user.create(db).await?;
    let rows = User::find_where(
        FilterOperator::Single(Filter::eq("email", user.email.clone())),
        db,
    )
    .await?;
    rows.into_iter()
        .max_by_key(|u| u.id.unwrap_or_default())
        .ok_or_else(|| libsql_orm::Error::NotFound("inserted row not found".to_string()))
}

#[tokio::test(flavor = "current_thread")]
async fn database_new_local_in_memory_works() {
    let db = Database::new_local(":memory:").await.unwrap();
    let mut rows = db.query("SELECT 1", vec![]).await.unwrap();
    assert!(rows.next().await.unwrap().is_some());
}

#[tokio::test(flavor = "current_thread")]
async fn migration_sql_creates_users_table() {
    let db = Database::new_local(":memory:").await.unwrap();
    db.execute(&User::migration_sql(), vec![]).await.unwrap();

    let mut rows = db
        .query(
            "SELECT name FROM sqlite_master WHERE type = 'table' AND name = 'users'",
            vec![],
        )
        .await
        .unwrap();
    assert!(rows.next().await.unwrap().is_some());
}

#[tokio::test(flavor = "current_thread")]
async fn create_single_insert_query_back_real_row() {
    let db = setup_db().await.unwrap();
    let inserted = insert_and_get_real(
        &db,
        &user("Alice", "alice@example.com", Some(30), Some(98.5), true),
    )
    .await
    .unwrap();
    assert_eq!(inserted.name, "Alice");
    assert_eq!(inserted.email, "alice@example.com");
    assert!(inserted.id.is_some());
}

#[tokio::test(flavor = "current_thread")]
async fn create_multiple_inserts_get_distinct_real_ids() {
    let db = setup_db().await.unwrap();
    let a = insert_and_get_real(&db, &user("A", "a@example.com", Some(20), None, true))
        .await
        .unwrap();
    let b = insert_and_get_real(&db, &user("B", "b@example.com", Some(21), None, true))
        .await
        .unwrap();

    assert_ne!(a.id, b.id);
}

#[tokio::test(flavor = "current_thread")]
async fn find_by_id_returns_found() {
    let db = setup_db().await.unwrap();
    let inserted = insert_and_get_real(&db, &user("Find", "find@example.com", None, None, true))
        .await
        .unwrap();
    let found = User::find_by_id(inserted.id.unwrap(), &db).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().email, "find@example.com");
}

#[tokio::test(flavor = "current_thread")]
async fn find_by_id_returns_none_when_missing() {
    let db = setup_db().await.unwrap();
    let found = User::find_by_id(999_999, &db).await.unwrap();
    assert!(found.is_none());
}

#[tokio::test(flavor = "current_thread")]
async fn find_all_returns_all_rows() {
    let db = setup_db().await.unwrap();
    insert_and_get_real(&db, &user("U1", "u1@example.com", None, None, true))
        .await
        .unwrap();
    insert_and_get_real(&db, &user("U2", "u2@example.com", None, None, false))
        .await
        .unwrap();

    let all = User::find_all(&db).await.unwrap();
    assert_eq!(all.len(), 2);
}

#[tokio::test(flavor = "current_thread")]
async fn find_where_eq_filters_rows() {
    let db = setup_db().await.unwrap();
    insert_and_get_real(&db, &user("EqA", "eqa@example.com", Some(20), None, true))
        .await
        .unwrap();
    insert_and_get_real(&db, &user("EqB", "eqb@example.com", Some(40), None, true))
        .await
        .unwrap();

    let rows = User::find_where(FilterOperator::Single(Filter::eq("age", 40i64)), &db)
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].name, "EqB");
}

#[tokio::test(flavor = "current_thread")]
async fn find_where_gt_filters_rows() {
    let db = setup_db().await.unwrap();
    insert_and_get_real(&db, &user("GtA", "gta@example.com", Some(25), None, true))
        .await
        .unwrap();
    insert_and_get_real(&db, &user("GtB", "gtb@example.com", Some(45), None, true))
        .await
        .unwrap();

    let rows = User::find_where(FilterOperator::Single(Filter::gt("age", 30i64)), &db)
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].name, "GtB");
}

#[tokio::test(flavor = "current_thread")]
async fn find_where_like_filters_rows() {
    let db = setup_db().await.unwrap();
    insert_and_get_real(&db, &user("LikeA", "like_a@example.com", None, None, true))
        .await
        .unwrap();
    insert_and_get_real(&db, &user("LikeB", "other@example.com", None, None, true))
        .await
        .unwrap();

    let rows = User::find_where(
        FilterOperator::Single(Filter::like("email", "%like_%")),
        &db,
    )
    .await
    .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].name, "LikeA");
}

#[tokio::test(flavor = "current_thread")]
async fn find_where_and_filters_rows() {
    let db = setup_db().await.unwrap();
    insert_and_get_real(&db, &user("AndA", "anda@example.com", Some(29), None, true))
        .await
        .unwrap();
    insert_and_get_real(&db, &user("AndB", "andb@example.com", Some(35), None, false))
        .await
        .unwrap();

    let filter = FilterOperator::And(vec![
        FilterOperator::Single(Filter::gt("age", 30i64)),
        FilterOperator::Single(Filter::eq("is_active", false)),
    ]);
    let rows = User::find_where(filter, &db).await.unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].name, "AndB");
}

#[tokio::test(flavor = "current_thread")]
async fn find_where_or_filters_rows() {
    let db = setup_db().await.unwrap();
    insert_and_get_real(&db, &user("OrA", "ora@example.com", Some(22), None, true))
        .await
        .unwrap();
    insert_and_get_real(&db, &user("OrB", "orb@example.com", Some(33), None, false))
        .await
        .unwrap();

    let filter = FilterOperator::Or(vec![
        FilterOperator::Single(Filter::eq("name", "OrA")),
        FilterOperator::Single(Filter::eq("name", "OrB")),
    ]);
    let rows = User::find_where(filter, &db).await.unwrap();
    assert_eq!(rows.len(), 2);
}

#[tokio::test(flavor = "current_thread")]
async fn update_changes_fields() {
    let db = setup_db().await.unwrap();
    let mut row = insert_and_get_real(
        &db,
        &user("Before", "before@example.com", Some(18), Some(1.0), true),
    )
    .await
    .unwrap();

    row.name = "After".to_string();
    row.age = Some(19);
    row.score = Some(77.25);
    row.update(&db).await.unwrap();

    let fetched = User::find_by_id(row.id.unwrap(), &db).await.unwrap().unwrap();
    assert_eq!(fetched.name, "After");
    assert_eq!(fetched.age, Some(19));
    assert_eq!(fetched.score, Some(77.25));
}

#[tokio::test(flavor = "current_thread")]
async fn update_changes_boolean_field() {
    let db = setup_db().await.unwrap();
    let mut row = insert_and_get_real(&db, &user("Bool", "bool@example.com", None, None, true))
        .await
        .unwrap();
    row.is_active = false;
    row.update(&db).await.unwrap();

    let fetched = User::find_by_id(row.id.unwrap(), &db).await.unwrap().unwrap();
    assert!(!fetched.is_active);
}

#[tokio::test(flavor = "current_thread")]
async fn delete_single_row() {
    let db = setup_db().await.unwrap();
    let row = insert_and_get_real(&db, &user("Del", "del@example.com", None, None, true))
        .await
        .unwrap();
    let deleted = row.delete(&db).await.unwrap();
    assert!(deleted);
    assert!(User::find_by_id(row.id.unwrap(), &db).await.unwrap().is_none());
}

#[tokio::test(flavor = "current_thread")]
async fn bulk_delete_rows() {
    let db = setup_db().await.unwrap();
    let a = insert_and_get_real(&db, &user("B1", "b1@example.com", None, None, true))
        .await
        .unwrap();
    let b = insert_and_get_real(&db, &user("B2", "b2@example.com", None, None, true))
        .await
        .unwrap();
    let c = insert_and_get_real(&db, &user("B3", "b3@example.com", None, None, true))
        .await
        .unwrap();

    let deleted_count = User::bulk_delete(&[a.id.unwrap(), b.id.unwrap()], &db)
        .await
        .unwrap();
    assert_eq!(deleted_count, 2);
    let all = User::find_all(&db).await.unwrap();
    assert_eq!(all.len(), 1);
    assert_eq!(all[0].id, c.id);
}

#[tokio::test(flavor = "current_thread")]
async fn delete_where_removes_matching_rows() {
    let db = setup_db().await.unwrap();
    insert_and_get_real(&db, &user("DW1", "dw1@example.com", Some(20), None, true))
        .await
        .unwrap();
    insert_and_get_real(&db, &user("DW2", "dw2@example.com", Some(40), None, true))
        .await
        .unwrap();

    let _ = User::delete_where(FilterOperator::Single(Filter::gt("age", 30i64)), &db)
        .await
        .unwrap();
    let all = User::find_all(&db).await.unwrap();
    assert_eq!(all.len(), 1);
    assert_eq!(all[0].name, "DW1");
}

#[tokio::test(flavor = "current_thread")]
async fn count_empty_table() {
    let db = setup_db().await.unwrap();
    assert_eq!(User::count(&db).await.unwrap(), 0);
}

#[tokio::test(flavor = "current_thread")]
async fn count_after_inserts() {
    let db = setup_db().await.unwrap();
    insert_and_get_real(&db, &user("C1", "c1@example.com", None, None, true))
        .await
        .unwrap();
    insert_and_get_real(&db, &user("C2", "c2@example.com", None, None, true))
        .await
        .unwrap();
    assert_eq!(User::count(&db).await.unwrap(), 2);
}

#[tokio::test(flavor = "current_thread")]
async fn count_where_matches() {
    let db = setup_db().await.unwrap();
    insert_and_get_real(&db, &user("CW1", "cw1@example.com", Some(19), None, false))
        .await
        .unwrap();
    insert_and_get_real(&db, &user("CW2", "cw2@example.com", Some(31), None, true))
        .await
        .unwrap();
    insert_and_get_real(&db, &user("CW3", "cw3@example.com", Some(55), None, true))
        .await
        .unwrap();

    let count = User::count_where(FilterOperator::Single(Filter::eq("is_active", true)), &db)
        .await
        .unwrap();
    assert_eq!(count, 2);
}

#[tokio::test(flavor = "current_thread")]
async fn find_paginated_first_page() {
    let db = setup_db().await.unwrap();
    for i in 0..5 {
        let email = format!("p1_{i}@example.com");
        insert_and_get_real(&db, &user("Pg", &email, Some(i), None, true))
            .await
            .unwrap();
    }

    let page = Pagination::new(1, 2);
    let result = User::find_paginated(&page, &db).await.unwrap();
    assert_eq!(result.data.len(), 2);
    assert_eq!(result.pagination.total, Some(5));
    assert_eq!(result.pagination.total_pages, Some(3));
}

#[tokio::test(flavor = "current_thread")]
async fn find_paginated_last_page() {
    let db = setup_db().await.unwrap();
    for i in 0..5 {
        let email = format!("p2_{i}@example.com");
        insert_and_get_real(&db, &user("Pg", &email, Some(i), None, true))
            .await
            .unwrap();
    }

    let page = Pagination::new(3, 2);
    let result = User::find_paginated(&page, &db).await.unwrap();
    assert_eq!(result.data.len(), 1);
    assert_eq!(result.pagination.total_pages, Some(3));
}

#[tokio::test(flavor = "current_thread")]
async fn query_builder_select_all() {
    let db = setup_db().await.unwrap();
    insert_and_get_real(&db, &user("Q1", "q1@example.com", None, None, true))
        .await
        .unwrap();
    insert_and_get_real(&db, &user("Q2", "q2@example.com", None, None, true))
        .await
        .unwrap();

    let rows = QueryBuilder::new("users").execute::<User>(&db).await.unwrap();
    assert_eq!(rows.len(), 2);
}

#[tokio::test(flavor = "current_thread")]
async fn query_builder_with_filter() {
    let db = setup_db().await.unwrap();
    insert_and_get_real(&db, &user("QF1", "qf1@example.com", Some(21), None, true))
        .await
        .unwrap();
    insert_and_get_real(&db, &user("QF2", "qf2@example.com", Some(41), None, true))
        .await
        .unwrap();

    let rows = QueryBuilder::new("users")
        .r#where(FilterOperator::Single(Filter::gt("age", 30i64)))
        .execute::<User>(&db)
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].name, "QF2");
}

#[tokio::test(flavor = "current_thread")]
async fn query_builder_order_limit_offset() {
    let db = setup_db().await.unwrap();
    insert_and_get_real(&db, &user("QO1", "qo1@example.com", Some(25), None, true))
        .await
        .unwrap();
    insert_and_get_real(&db, &user("QO2", "qo2@example.com", Some(30), None, true))
        .await
        .unwrap();
    insert_and_get_real(&db, &user("QO3", "qo3@example.com", Some(35), None, true))
        .await
        .unwrap();

    let rows = QueryBuilder::new("users")
        .order_by(Sort::new("age", SortOrder::Asc))
        .limit(1)
        .offset(1)
        .execute::<User>(&db)
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].age, Some(30));
}

#[tokio::test(flavor = "current_thread")]
async fn query_builder_execute_count() {
    let db = setup_db().await.unwrap();
    insert_and_get_real(&db, &user("QC1", "qc1@example.com", Some(25), None, true))
        .await
        .unwrap();
    insert_and_get_real(&db, &user("QC2", "qc2@example.com", Some(50), None, true))
        .await
        .unwrap();

    let count = QueryBuilder::new("users")
        .r#where(FilterOperator::Single(Filter::gt("age", 30i64)))
        .execute_count(&db)
        .await
        .unwrap();
    assert_eq!(count, 1);
}

#[tokio::test(flavor = "current_thread")]
async fn search_filter_across_multiple_columns() {
    let db = setup_db().await.unwrap();
    insert_and_get_real(&db, &user("Search Me", "sm@example.com", None, None, true))
        .await
        .unwrap();
    insert_and_get_real(
        &db,
        &user("Another", "needle@example.com", None, None, true),
    )
    .await
    .unwrap();

    let search = SearchFilter::new("needle", vec!["name", "email"]);
    let result = User::search(&search, None, &db).await.unwrap();
    assert_eq!(result.data.len(), 1);
    assert_eq!(result.data[0].email, "needle@example.com");
}

#[tokio::test(flavor = "current_thread")]
async fn create_or_update_creates_when_no_pk() {
    let db = setup_db().await.unwrap();
    let u = user("COU", "cou@example.com", Some(44), None, true);
    let _ = u.create_or_update(&db).await.unwrap();
    assert_eq!(User::count(&db).await.unwrap(), 1);
}

#[tokio::test(flavor = "current_thread")]
async fn create_or_update_updates_when_pk_exists() {
    let db = setup_db().await.unwrap();
    let mut existing = insert_and_get_real(&db, &user("Old", "old@example.com", Some(20), None, true))
        .await
        .unwrap();
    existing.name = "New".to_string();
    existing.age = Some(21);

    let _ = existing.create_or_update(&db).await.unwrap();
    let fetched = User::find_by_id(existing.id.unwrap(), &db).await.unwrap().unwrap();
    assert_eq!(fetched.name, "New");
    assert_eq!(fetched.age, Some(21));
}

#[tokio::test(flavor = "current_thread")]
async fn data_round_trip_null_fields() {
    let db = setup_db().await.unwrap();
    let row = insert_and_get_real(
        &db,
        &user("Nulls", "nulls@example.com", None, None, true),
    )
    .await
    .unwrap();
    assert_eq!(row.age, None);
    assert_eq!(row.score, None);
}

#[tokio::test(flavor = "current_thread")]
async fn data_round_trip_float_values() {
    let db = setup_db().await.unwrap();
    let row = insert_and_get_real(
        &db,
        &user("Float", "float@example.com", Some(10), Some(42.125), true),
    )
    .await
    .unwrap();
    assert_eq!(row.score, Some(42.125));
}

#[tokio::test(flavor = "current_thread")]
async fn data_round_trip_boolean_false() {
    let db = setup_db().await.unwrap();
    let row = insert_and_get_real(
        &db,
        &user("False", "false@example.com", Some(10), None, false),
    )
    .await
    .unwrap();
    assert!(!row.is_active);
}

#[tokio::test(flavor = "current_thread")]
async fn data_round_trip_special_chars() {
    let db = setup_db().await.unwrap();
    let special_name = "O'Reilly & Sons (R&D)";
    let row = insert_and_get_real(
        &db,
        &user(special_name, "special@example.com", Some(10), None, true),
    )
    .await
    .unwrap();
    assert_eq!(row.name, special_name);
}

#[tokio::test(flavor = "current_thread")]
async fn data_round_trip_unicode_text() {
    let db = setup_db().await.unwrap();
    let unicode_name = "\u{4f60}\u{597d} \u{4e16}\u{754c}";
    let row = insert_and_get_real(
        &db,
        &user(unicode_name, "unicode@example.com", Some(10), None, true),
    )
    .await
    .unwrap();
    assert_eq!(row.name, unicode_name);
}

#[tokio::test(flavor = "current_thread")]
async fn migrations_init_execute_and_get_executed() {
    let db = Database::new_local(":memory:").await.unwrap();
    let manager = MigrationManager::new(db);
    manager.init().await.unwrap();

    let migration = MigrationBuilder::new("create_projects")
        .up("CREATE TABLE projects (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL)")
        .build();
    manager.execute_migration(&migration).await.unwrap();

    let mut rows = manager
        .database()
        .query(
            "SELECT name FROM sqlite_master WHERE type = 'table' AND name = 'projects'",
            vec![],
        )
        .await
        .unwrap();
    assert!(rows.next().await.unwrap().is_some());

    let executed = manager.get_executed_migrations().await.unwrap();
    assert_eq!(executed.len(), 1);
    assert_eq!(executed[0].name, "create_projects");
}

#[tokio::test(flavor = "current_thread")]
async fn edge_case_find_all_on_empty_table() {
    let db = setup_db().await.unwrap();
    let all = User::find_all(&db).await.unwrap();
    assert!(all.is_empty());
}

#[tokio::test(flavor = "current_thread")]
async fn edge_case_delete_nonexistent_row() {
    let db = setup_db().await.unwrap();
    let ghost = User {
        id: Some(123_456),
        name: "Ghost".to_string(),
        email: "ghost@example.com".to_string(),
        age: None,
        score: None,
        is_active: false,
    };

    let deleted = ghost.delete(&db).await.unwrap();
    assert!(deleted);
    assert_eq!(User::count(&db).await.unwrap(), 0);
}
