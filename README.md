update # libsql-orm

[![Crates.io](https://img.shields.io/crates/v/libsql-orm.svg)](https://crates.io/crates/libsql-orm)
[![Documentation](https://docs.rs/libsql-orm/badge.svg)](https://docs.rs/libsql-orm)
[![License](https://img.shields.io/crates/l/libsql-orm.svg)](LICENSE)
[![Build Status](https://github.com/ayonsaha2011/libsql-orm/workflows/CI/badge.svg)](https://github.com/ayonsaha2011/libsql-orm/actions)

A powerful, async-first ORM for [Turso Database](https://github.com/tursodatabase) with first-class support for **Cloudflare Workers** and WebAssembly environments.

> ⚠️ **Disclaimer**: This library is in early development and not fully tested in production environments. Use at your own risk. Please report any issues you encounter and feel free to contribute via pull requests - we're happy to address them and welcome community contributions!

## ✨ Features

- 🚀 **Cloudflare Workers Ready** - Built specifically for edge computing environments
- 🔄 **Async/Await Support** - Fully async API with excellent performance
- 🎯 **Type-Safe** - Leverages Rust's type system for compile-time safety
- 📊 **Rich Query Builder** - Fluent API for complex queries
- 🔍 **Advanced Filtering** - Search, pagination, sorting, and aggregations
- 🎨 **Derive Macros** - Automatic model generation with `#[derive(Model)]`
- 📦 **Bulk Operations** - Efficient batch inserts, updates, and deletes
- 🌐 **WASM Compatible** - Optimized for WebAssembly targets
- 🔧 **Custom Table Names** - `#[table_name("custom")]` attribute support
- ✅ **Boolean Type Safety** - Automatic SQLite integer ↔ Rust boolean conversion
- 🏷️ **Column Attributes** - `#[orm_column(...)]` for column customization
- 🔄 **Upsert Operations** - Smart create_or_update and upsert methods
- 🔌 **MCP Support** - Model Context Protocol integration for AI-powered database interactions

## 🚀 Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
libsql-orm = { version = "0.2.4", features = ["cloudflare"] }
serde = { version = "1.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }

# Required for Cloudflare Workers support - use git version of libsql with newer worker dependency
[patch.crates-io]
libsql = { git = "https://github.com/ayonsaha2011/libsql", features = ["cloudflare"] }
```

### Why the Git Patch?

For Cloudflare Workers compatibility, you need to use a patched version of libsql that includes:
- Updated `worker` dependency compatibility
- Enhanced WASM support for edge environments
- Cloudflare-specific optimizations

The patch ensures seamless integration with Cloudflare Workers' runtime environment.

### Basic Usage

```rust
use libsql_orm::{Model, Database, FilterOperator, Filter, Value};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Model, Debug, Clone, Serialize, Deserialize)]
#[table_name("users")]  // Custom table name (optional)
struct User {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    pub age: Option<i32>,
    pub is_active: bool,        // ✅ Automatic boolean conversion
    pub is_verified: bool,      // ✅ Works with any boolean field
    pub created_at: DateTime<Utc>,
}

// In your async function
async fn example() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to database
    let db = Database::new_connect("turso://your-db.turso.io", "your-auth-token").await?;
    
    // Create a user
    let user = User {
        id: None,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        age: Some(30),
        is_active: true,
        created_at: Utc::now(),
    };
    
    // Save to database
    let saved_user = user.create(&db).await?;
    
    // Find users
    let users = User::find_all(&db).await?;
    
    // Query with conditions
    let active_users = User::find_where(
        FilterOperator::Single(Filter::eq("is_active", true)),
        &db
    ).await?;
    
    Ok(())
}
```

### Cloudflare Workers Integration

First, ensure your `Cargo.toml` includes the necessary features and patches:

```toml
[dependencies]
libsql-orm = { version = "0.2.4", features = ["cloudflare"] }
worker = ">=0.7.0"
serde = { version = "1.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }

# Use git version of libsql with newer worker dependency
[patch.crates-io]
libsql = { git = "https://github.com/ayonsaha2011/libsql", features = ["cloudflare"] }
```

Then in your worker code:

```rust
use worker::*;
use libsql_orm::{Model, Database};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Model, Debug, Clone, Serialize, Deserialize)]
#[table_name("blog_posts")]  // Custom table name
struct Post {
    pub id: Option<i64>,
    pub title: String,
    pub content: String,
    pub published: bool,       // ✅ Boolean automatically converted from SQLite
    pub featured: bool,        // ✅ Multiple boolean fields supported
    pub created_at: DateTime<Utc>,
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    
    // Get database credentials from environment
    let database_url = env.var("TURSO_DATABASE_URL")?.to_string();
    let auth_token = env.var("TURSO_AUTH_TOKEN")?.to_string();
    
    // Connect to database
    let db = Database::new_connect(&database_url, &auth_token).await
        .map_err(|e| format!("Database connection failed: {}", e))?;
    
    // Handle the request
    match req.method() {
        Method::Get => {
            let posts = Post::find_all(&db).await
                .map_err(|e| format!("Query failed: {}", e)))?;
            Response::from_json(&posts)
        }
        Method::Post => {
            let post: Post = req.json().await?;
            let saved_post = post.create(&db).await
                .map_err(|e| format!("Create failed: {}", e)))?;
            Response::from_json(&saved_post)
        }
        _ => Response::error("Method not allowed", 405)
    }
}
```

### Advanced Cloudflare Workers with Axum Integration

For more complex applications, you can integrate libsql-orm with Axum for better routing and state management.

**Key Requirements:**
- 🏗️ **Library crate**: Use `crate-type = ["cdylib"]` for Cloudflare Workers
- 🔧 **Worker features**: Enable `http` and `axum` features for the worker crate
- 🎯 **Axum config**: Use `default-features = false` for WASM compatibility
- 🔗 **Tower service**: Required for Axum routing integration

**Setup:**

```toml
[package]
name = "my-cloudflare-app"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
worker = { version = "0.7", features = ['http', 'axum'] }
worker-macros = { version = "0.7", features = ['http'] }
axum = { version = "0.8", default-features = false, features = ["json", "macros"] }
tower-service = "0.3.3"
libsql-orm = { version = "0.2.4", features = ["cloudflare"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
console_error_panic_hook = "0.1"

# Use git version of libsql with newer worker dependency
[patch.crates-io]
libsql = { git = "https://github.com/ayonsaha2011/libsql", features = ["cloudflare"] }
```

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use tower_service::Service;
use worker::*;
use std::result::Result;
use libsql_orm::{Model, Database, FilterOperator, Filter};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::sync::Arc;

// Application state
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
}

impl AppState {
    pub async fn new(env: &Env) -> worker::Result<Self> {
        // Get database credentials from environment
        let database_url = env.var("TURSO_DATABASE_URL")?.to_string();
        let auth_token = env.var("TURSO_AUTH_TOKEN")?.to_string();

        // Connect to database
        let db = Database::new_connect(&database_url, &auth_token).await
            .map_err(|e| format!("Database connection failed: {}", e))?;

        Ok(Self {
            db: Arc::new(db),
        })
    }
}

// User model
#[derive(Model, Debug, Clone, Serialize, Deserialize)]
#[table_name("users")]
struct User {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

// Request/Response DTOs
#[derive(Deserialize)]
struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self { success: true, data: Some(data), error: None }
    }

    fn error(error: String) -> Self {
        Self { success: false, data: None, error: Some(error) }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    pub error: String,
    pub message: String,
}

// Route handlers
#[worker::send]
async fn get_users(State(state): State<AppState>) -> Result<Json<ApiResponse<Vec<User>>>, (StatusCode, Json<ErrorResponse>)> {
    match User::find_all(&state.db).await {
        Ok(users) => Ok(Json(ApiResponse::success(users))),
        Err(e) => {
            console_log!("Error fetching users: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                error: "internal_server_error".to_string(),
                message: "Internal server error".to_string()
            })))
        }
    }
}

#[worker::send]
async fn get_user_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>
) -> Result<Json<ApiResponse<User>>, (StatusCode, Json<ErrorResponse>)> {
    match User::find_by_id(id, &state.db).await {
        Ok(Some(user)) => Ok(Json(ApiResponse::success(user))),
        Ok(None) => Err((StatusCode::NOT_FOUND, Json(ErrorResponse {
            error: "user_not_found".to_string(),
            message: "User not found".to_string()
        }))),
        Err(e) => {
            console_log!("Error fetching user {}: {}", id, e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                error: "internal_server_error".to_string(),
                message: "Internal server error".to_string()
            })))
        }
    }
}

#[worker::send]
async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>
) -> Result<(StatusCode, Json<ApiResponse<User>>), (StatusCode, Json<ErrorResponse>)> {
    let user = User {
        id: None,
        name: req.name,
        email: req.email,
        is_active: true,
        created_at: Utc::now(),
    };

    match user.create(&state.db).await {
        Ok(created_user) => Ok((
            StatusCode::CREATED,
            Json(ApiResponse::success(created_user))
        )),
        Err(e) => {
            console_log!("Error creating user: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                error: "internal_server_error".to_string(),
                message: "Internal server error".to_string()
            })))
        }
    }
}

#[worker::send]
async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(mut user): Json<User>
) -> Result<Json<ApiResponse<User>>, (StatusCode, Json<ErrorResponse>)> {
    user.id = Some(id);

    match user.update(&state.db).await {
        Ok(updated_user) => Ok(Json(ApiResponse::success(updated_user))),
        Err(e) => {
            console_log!("Error updating user {}: {}", id, e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                error: "internal_server_error".to_string(),
                message: "Internal server error".to_string()
            })))
        }
    }
}

#[worker::send]
async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i64>
) -> Result<Json<ApiResponse<String>>, (StatusCode, Json<ErrorResponse>)> {
    let filter = FilterOperator::Single(Filter::eq("id", id));

    match User::delete_where(filter, &state.db).await {
        Ok(_) => Ok(Json(ApiResponse::success("User deleted successfully".to_string()))),
        Err(e) => {
            console_log!("Error deleting user {}: {}", id, e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                error: "internal_server_error".to_string(),
                message: "Internal server error".to_string()
            })))
        }
    }
}

#[worker::send]
async fn get_active_users(State(state): State<AppState>) -> Result<Json<ApiResponse<Vec<User>>>, (StatusCode, Json<ErrorResponse>)> {
    let filter = FilterOperator::Single(Filter::eq("is_active", true));

    match User::find_where(filter, &state.db).await {
        Ok(users) => Ok(Json(ApiResponse::success(users))),
        Err(e) => {
            console_log!("Error fetching active users: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                error: "internal_server_error".to_string(),
                message: "Internal server error".to_string()
            })))
        }
    }
}

// Create router with all routes
fn create_router() -> Router<AppState> {
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route("/users/active", get(get_active_users))
        .route("/users/:id", get(get_user_by_id).put(update_user).delete(delete_user))
}

// Main Cloudflare Workers handler
#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> worker::Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    // Initialize application state
    let app_state = match AppState::new(&env).await {
        Ok(state) => state,
        Err(e) => {
            console_log!("Failed to initialize application state: {}", e);
            return Ok(axum::http::Response::builder()
                .status(500)
                .header("content-type", "application/json")
                .body(axum::body::Body::from(
                    "{\"error\":\"initialization_failed\",\"message\":\"Failed to initialize application\"}"
                ))?
            );
        }
    };

    // Create router
    let mut router = create_router().with_state(app_state);

    // Handle the request
    Ok(router.call(req).await?)
}
```

This example demonstrates:

- **🏗️ Clean Architecture**: Separating models, DTOs, and handlers
- **🔄 State Management**: Using Axum's state system for database sharing
- **🛣️ RESTful Routing**: Complete CRUD operations with proper HTTP methods
- **📊 Error Handling**: Comprehensive error handling with tuple returns
- **🎯 Type Safety**: Strong typing with request/response DTOs
- **🚀 Performance**: Efficient database connection sharing with `AppState::new()`
- **📝 Logging**: Built-in `console_log!` macro for debugging
- **🔍 Advanced Queries**: Filtering and conditional operations
- **⚡ Worker Integration**: `#[worker::send]` attributes for optimal performance

### API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/users` | Get all users |
| POST | `/users` | Create a new user |
| GET | `/users/:id` | Get user by ID |
| PUT | `/users/:id` | Update user |
| DELETE | `/users/:id` | Delete user |
| GET | `/users/active` | Get all active users |

### 🚀 Deployment

To deploy this Cloudflare Workers application, you'll need a `wrangler.toml` configuration:

```toml
name = "my-cloudflare-app"
main = "build/worker/shim.mjs"
compatibility_date = "2023-05-18"

[env.production.vars]
TURSO_DATABASE_URL = "your-database-url"
TURSO_AUTH_TOKEN = "your-auth-token"

[[env.production.rules]]
type = "CompiledWasm"
globs = ["**/*.wasm"]
fallthrough = true
```

**Deploy commands:**
```bash
# Install dependencies
npm install -g wrangler

# Deploy to Cloudflare Workers
wrangler deploy
```

## 📚 Advanced Features

### Custom Table Names

Use the `#[table_name("custom_name")]` attribute to specify custom table names:

```rust
#[derive(Model, Serialize, Deserialize)]
#[table_name("user_accounts")]  // Custom table name
struct User {
    pub id: Option<i64>,
    pub username: String,
    pub email: String,
}

// Default table name would be "user" (struct name lowercase)
// With attribute, table name is "user_accounts"
assert_eq!(User::table_name(), "user_accounts");
```

**Benefits:**
- 🏷️ **Legacy Integration** - Map to existing database tables
- 🎯 **Naming Control** - Override default naming conventions  
- 📁 **Multi-tenant** - Use prefixes like `tenant_users`
- 🔄 **Migration Friendly** - Rename tables without changing structs

### Boolean Type Safety

libsql-orm automatically handles boolean conversion between SQLite and Rust:

```rust
use libsql_orm::{Model, FilterOperator, Filter, Value};
use serde::{Serialize, Deserialize};

#[derive(Model, Serialize, Deserialize)]
struct User {
    pub id: Option<i64>,
    pub is_active: bool,      // ✅ SQLite INTEGER(0/1) ↔ Rust bool
    pub is_verified: bool,    // ✅ Automatic conversion
    pub has_premium: bool,    // ✅ Works with any boolean field name
    pub can_edit: bool,       // ✅ No configuration needed
    pub enabled: bool,        // ✅ Type-safe operations
}

// All boolean operations work seamlessly
let user = User::find_where(
    FilterOperator::Single(Filter::eq("is_active", true)),
    &db
).await?;

// JSON serialization works correctly
let json = serde_json::to_string(&user)?;  // ✅ Booleans as true/false
let deserialized: User = serde_json::from_str(&json)?;  // ✅ No errors
```

**Key Features:**
- ✅ **Automatic Detection** - Boolean fields identified at compile time
- ✅ **Zero Configuration** - Works with any boolean field name
- ✅ **Type Safety** - No runtime errors or invalid conversions
- ✅ **Performance** - Conversion logic generated at compile time
- ✅ **JSON Compatible** - Seamless serialization/deserialization

### Column Attributes

Customize column properties with `#[orm_column(...)]`:

```rust
use libsql_orm::Model;
use serde::{Serialize, Deserialize};

#[derive(Model, Serialize, Deserialize)]
struct Product {
    #[orm_column(type = "INTEGER PRIMARY KEY AUTOINCREMENT")]
    pub id: Option<i64>,
    
    #[orm_column(not_null, unique)]
    pub sku: String,
    
    #[orm_column(type = "REAL CHECK(price >= 0)")]
    pub price: f64,
    
    #[orm_column(type = "BOOLEAN DEFAULT TRUE")]
    pub is_available: bool,     // ✅ Boolean with DEFAULT constraint
}
```

### Query Builder

```rust
use libsql_orm::{QueryBuilder, FilterOperator, Filter, Sort, SortOrder, Pagination};

// Complex query with filtering and pagination
let query = QueryBuilder::new("users")
    .select(&["id", "name", "email"])
    .r#where(FilterOperator::Single(Filter::ge("age", 18i64)))
    .order_by(Sort::new("created_at", SortOrder::Desc))
    .limit(10)
    .offset(20);

let (sql, params) = query.build()?;
```

### Pagination

```rust
use libsql_orm::{Pagination, PaginatedResult};

let pagination = Pagination::new(1, 10); // page 1, 10 items per page
let result: PaginatedResult<User> = User::find_paginated(&pagination, &db).await?;

// Access pagination info
// Page: result.pagination.page
// Total pages: result.pagination.total_pages.unwrap_or(0)
// Total items: result.pagination.total.unwrap_or(0)
for user in result.data {
    // Process user: user.name
}
```

### Bulk Operations

```rust
// Bulk insert
let users = vec![
    User { /* ... */ },
    User { /* ... */ },
    User { /* ... */ },
];
let saved_users = User::bulk_create(&users, &db).await?;

// Bulk delete
let ids_to_delete = vec![1, 2, 3, 4, 5];
let deleted_count = User::bulk_delete(&ids_to_delete, &db).await?;
```

### Aggregations

```rust
use libsql_orm::Aggregate;

// Count users
let total_users = User::count(&db).await?;

// Average age
let avg_age = User::aggregate(
    Aggregate::Avg,
    "age",
    None,
    &db
).await?;

// Count with filter
let active_users_count = User::count_where(
    FilterOperator::Single(Filter::eq("is_active", true)),
    &db
).await?;
```

### Search

```rust
use libsql_orm::{SearchFilter, Pagination};

let search = SearchFilter::new(
    "john",
    vec!["name", "email"]
);

// Optional pagination
let pagination = Pagination::new(1, 10);
let results = User::search(&search, Some(&pagination), &db).await?;
```

### Upsert Operations

libsql-orm provides intelligent create-or-update operations:

```rust
use libsql_orm::{Model, Database};
use chrono::{DateTime, Utc};

// Create or update based on primary key
let mut user = User {
    id: Some(123),  // If record exists, it will be updated
    name: "John Doe".to_string(),
    email: "john@example.com".to_string(),
    is_active: true,
    created_at: Utc::now(),
};

// Automatically decides whether to create or update
let saved_user = user.create_or_update(&db).await?;

// Upsert based on unique constraints (e.g., email)
let user = User {
    id: None,  // Primary key not set
    name: "Jane Smith".to_string(),
    email: "jane@example.com".to_string(),  // Unique field
    is_active: true,
    created_at: Utc::now(),
};

// Will update existing record with this email, or create new if not found
let saved_user = user.upsert(&["email"], &db).await?;

// Multiple unique constraints
let saved_user = user.upsert(&["email", "username"], &db).await?;
```

## 🔌 MCP (Model Context Protocol) Support

libsql-orm provides comprehensive MCP server integration for AI-powered database interactions. The MCP protocol enables seamless communication between AI assistants and your Turso database.

### MCP Features

- **40+ Tools**: Complete CRUD operations, queries, migrations, and more
- **Type-Safe Queries**: Automatic validation and type checking
- **Advanced Filtering**: Complex AND/OR/NOT filter combinations
- **Pagination Support**: Both offset-based and cursor-based pagination
- **Migration Management**: Create, execute, and track database migrations
- **Bulk Operations**: Efficient batch processing
- **Search Capabilities**: Multi-column text search
- **Aggregations**: COUNT, SUM, AVG, MIN, MAX operations

### MCP Resources

The MCP server exposes:
- Database connection resources
- Model resources (table-based)
- Query builder resources
- Migration tracking resources

### Quick MCP Example

```json
{
  "tool": "model_find_where",
  "arguments": {
    "model": "User",
    "filter": {
      "type": "And",
      "filters": [
        {
          "type": "Single",
          "filter": {"column": "is_active", "operator": "Eq", "value": true}
        },
        {
          "type": "Single",
          "filter": {"column": "age", "operator": "Gt", "value": 18}
        }
      ]
    }
  }
}
```

📖 **[Complete MCP Documentation](MCP_DOCUMENTATION.md)** - Comprehensive guide with all 40+ tools, examples, and best practices

## 🏗️ Architecture

### WASM Compatibility

libsql-orm is built from the ground up for WebAssembly environments:

- Uses `libsql` WASM bindings for database connectivity
- Optimized async runtime for edge computing
- Minimal binary size with selective feature compilation
- Compatible with Cloudflare Workers, Deno Deploy, and other edge platforms


## 🔗 Ecosystem

libsql-orm works great with:

- **[Turso Database](https://github.com/tursodatabase)** - The database platform
- **[Turso](https://turso.tech/)** - Managed Turso hosting
- **[Cloudflare Workers](https://workers.cloudflare.com/)** - Edge computing platform
- **[worker-rs](https://github.com/cloudflare/workers-rs)** - Cloudflare Workers Rust SDK

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## ☕ Support the Project

If you find this library helpful and would like to support its development, consider making a donation:

### 💰 Donation Options

- **GitHub Sponsors**: [Sponsor on GitHub](https://github.com/sponsors/ayonsaha2011)
- **Buy Me a Coffee**: [Buy me a coffee](https://coff.ee/ayonsaha2011)
- **PayPal**: [PayPal Donation](https://paypal.me/ayonsaha)

### 🎯 What Your Support Helps With

- 🚀 **Feature Development** - Building new capabilities and improvements
- 🐛 **Bug Fixes** - Maintaining stability and reliability  
- 📚 **Documentation** - Creating better guides and examples
- 🔧 **Maintenance** - Keeping the library up-to-date with dependencies
- ☁️ **Infrastructure** - Hosting costs for CI/CD and testing

Every contribution, no matter the size, helps make this library better for everyone! 🙏

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Turso team](https://github.com/tursodatabase) for the excellent database platform
- [Cloudflare](https://cloudflare.com) for the Workers platform
- Rust community for the amazing ecosystem

---
**Need help?**
- 📚 [Documentation](https://docs.rs/libsql-orm)
- 🔌 [MCP Documentation](MCP_DOCUMENTATION.md)
- 💬 [Discussions](https://github.com/ayonsaha2011/libsql-orm/discussions)
- 🐛 [Issues](https://github.com/ayonsaha2011/libsql-orm/issues)

