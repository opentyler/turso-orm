# libsql-orm MCP Server Documentation

## Overview

`libsql-orm` is a powerful, async-first ORM for Turso Database (libsql) with first-class support for Cloudflare Workers and WebAssembly environments. This MCP (Model Context Protocol) server provides tools and resources for interacting with Turso databases using a type-safe, Rust-based ORM.

**Version**: 0.2.4
**License**: MIT
**Repository**: https://github.com/ayonsaha2011/libsql-orm

## Key Features

- 🚀 **Cloudflare Workers Ready** - Built specifically for edge computing environments
- 🔄 **Async/Await Support** - Fully async API with excellent performance
- 🎯 **Type-Safe** - Leverages Rust's type system for compile-time safety
- 📊 **Rich Query Builder** - Fluent API for complex queries
- 🔍 **Advanced Filtering** - Search, pagination, sorting, and aggregations
- 🎨 **Derive Macros** - Automatic model generation with `#[derive(Model)]`
- 📦 **Bulk Operations** - Efficient batch inserts, updates, and deletes
- 🌐 **WASM Compatible** - Optimized for WebAssembly targets
- 🔄 **Upsert Operations** - Smart create_or_update and upsert methods
- 📝 **Built-in Logging** - Comprehensive logging for debugging

## MCP Resources

### Resource Types

The MCP server exposes the following resource types that can be queried and manipulated:

#### 1. **Database Connection Resource**
- **Type**: `database://connection`
- **Description**: Manages connections to Turso databases
- **Operations**:
  - `new_connect(url, token)` - Create new database connection
  - `query(sql, params)` - Execute SQL queries
  - `execute(sql, params)` - Execute SQL statements

#### 2. **Model Resources**
- **Type**: `model://{table_name}`
- **Description**: Represents database tables as Rust structs with ORM capabilities
- **Operations**:
  - CRUD operations (Create, Read, Update, Delete)
  - Bulk operations
  - Search and filtering
  - Pagination

#### 3. **Query Resources**
- **Type**: `query://builder`
- **Description**: Fluent query builder for complex SQL queries
- **Operations**:
  - SELECT with column selection
  - JOIN operations (INNER, LEFT, RIGHT, FULL)
  - WHERE clauses with filters
  - GROUP BY and HAVING
  - ORDER BY with sorting
  - LIMIT and OFFSET

#### 4. **Migration Resources**
- **Type**: `migration://{migration_id}`
- **Description**: Database schema migrations
- **Operations**:
  - Create migrations
  - Execute migrations
  - Rollback migrations
  - Track migration history

---

## MCP Tools

### Database Tools

#### `database_connect`
Connect to a Turso database.

**Parameters**:
- `url` (string, required): Database URL (e.g., "turso://your-db.turso.io")
- `token` (string, required): Authentication token

**Returns**: Database connection object

**Example**:
```json
{
  "tool": "database_connect",
  "arguments": {
    "url": "turso://my-db.turso.io",
    "token": "your-auth-token"
  }
}
```

#### `database_query`
Execute a SQL query.

**Parameters**:
- `sql` (string, required): SQL query string
- `params` (array, optional): Query parameters

**Returns**: Query results as rows

**Example**:
```json
{
  "tool": "database_query",
  "arguments": {
    "sql": "SELECT * FROM users WHERE age > ?",
    "params": [18]
  }
}
```

---

### Model CRUD Tools

#### `model_create`
Create a new record in the database.

**Parameters**:
- `model` (string, required): Model name
- `data` (object, required): Record data

**Returns**: Created record with ID

**Example**:
```json
{
  "tool": "model_create",
  "arguments": {
    "model": "User",
    "data": {
      "name": "John Doe",
      "email": "john@example.com",
      "is_active": true
    }
  }
}
```

#### `model_find_by_id`
Find a record by primary key.

**Parameters**:
- `model` (string, required): Model name
- `id` (integer, required): Primary key value

**Returns**: Record or null

**Example**:
```json
{
  "tool": "model_find_by_id",
  "arguments": {
    "model": "User",
    "id": 123
  }
}
```

#### `model_find_all`
Find all records.

**Parameters**:
- `model` (string, required): Model name

**Returns**: Array of records

**Example**:
```json
{
  "tool": "model_find_all",
  "arguments": {
    "model": "User"
  }
}
```

#### `model_find_where`
Find records matching filter criteria.

**Parameters**:
- `model` (string, required): Model name
- `filter` (object, required): Filter operator

**Returns**: Array of matching records

**Example**:
```json
{
  "tool": "model_find_where",
  "arguments": {
    "model": "User",
    "filter": {
      "type": "Single",
      "filter": {
        "column": "is_active",
        "operator": "Eq",
        "value": true
      }
    }
  }
}
```

#### `model_update`
Update an existing record.

**Parameters**:
- `model` (string, required): Model name
- `id` (integer, required): Primary key value
- `data` (object, required): Updated fields

**Returns**: Updated record

**Example**:
```json
{
  "tool": "model_update",
  "arguments": {
    "model": "User",
    "id": 123,
    "data": {
      "name": "Jane Doe",
      "email": "jane@example.com"
    }
  }
}
```

#### `model_delete`
Delete a record by ID.

**Parameters**:
- `model` (string, required): Model name
- `id` (integer, required): Primary key value

**Returns**: Success boolean

**Example**:
```json
{
  "tool": "model_delete",
  "arguments": {
    "model": "User",
    "id": 123
  }
}
```

---

### Upsert Tools

#### `model_create_or_update`
Create or update based on primary key existence.

**Parameters**:
- `model` (string, required): Model name
- `data` (object, required): Record data (must include ID for update)

**Returns**: Created or updated record

**Example**:
```json
{
  "tool": "model_create_or_update",
  "arguments": {
    "model": "User",
    "data": {
      "id": 123,
      "name": "Updated Name",
      "email": "updated@example.com"
    }
  }
}
```

#### `model_upsert`
Upsert based on unique constraint columns.

**Parameters**:
- `model` (string, required): Model name
- `unique_columns` (array, required): Column names to check for uniqueness
- `data` (object, required): Record data

**Returns**: Created or updated record

**Example**:
```json
{
  "tool": "model_upsert",
  "arguments": {
    "model": "User",
    "unique_columns": ["email"],
    "data": {
      "name": "John Doe",
      "email": "john@example.com"
    }
  }
}
```

---

### Bulk Operation Tools

#### `model_bulk_create`
Create multiple records in a transaction.

**Parameters**:
- `model` (string, required): Model name
- `records` (array, required): Array of record data

**Returns**: Array of created records

**Example**:
```json
{
  "tool": "model_bulk_create",
  "arguments": {
    "model": "User",
    "records": [
      {"name": "User 1", "email": "user1@example.com"},
      {"name": "User 2", "email": "user2@example.com"},
      {"name": "User 3", "email": "user3@example.com"}
    ]
  }
}
```

#### `model_bulk_update`
Update multiple records in a transaction.

**Parameters**:
- `model` (string, required): Model name
- `records` (array, required): Array of records with IDs

**Returns**: Array of updated records

**Example**:
```json
{
  "tool": "model_bulk_update",
  "arguments": {
    "model": "User",
    "records": [
      {"id": 1, "is_active": false},
      {"id": 2, "is_active": false},
      {"id": 3, "is_active": false}
    ]
  }
}
```

#### `model_bulk_delete`
Delete multiple records by IDs.

**Parameters**:
- `model` (string, required): Model name
- `ids` (array, required): Array of primary key values

**Returns**: Number of deleted records

**Example**:
```json
{
  "tool": "model_bulk_delete",
  "arguments": {
    "model": "User",
    "ids": [1, 2, 3, 4, 5]
  }
}
```

---

### Pagination Tools

#### `model_find_paginated`
Find records with offset-based pagination.

**Parameters**:
- `model` (string, required): Model name
- `page` (integer, required): Page number (1-based)
- `per_page` (integer, required): Items per page

**Returns**: Paginated result with data and metadata

**Example**:
```json
{
  "tool": "model_find_paginated",
  "arguments": {
    "model": "User",
    "page": 1,
    "per_page": 10
  }
}
```

**Response**:
```json
{
  "data": [...],
  "pagination": {
    "page": 1,
    "per_page": 10,
    "total": 100,
    "total_pages": 10
  }
}
```

#### `model_find_where_paginated`
Find filtered records with pagination.

**Parameters**:
- `model` (string, required): Model name
- `filter` (object, required): Filter operator
- `page` (integer, required): Page number
- `per_page` (integer, required): Items per page

**Returns**: Paginated result

**Example**:
```json
{
  "tool": "model_find_where_paginated",
  "arguments": {
    "model": "User",
    "filter": {
      "type": "Single",
      "filter": {"column": "is_active", "operator": "Eq", "value": true}
    },
    "page": 1,
    "per_page": 20
  }
}
```

---

### Search Tools

#### `model_search`
Search records across multiple columns.

**Parameters**:
- `model` (string, required): Model name
- `query` (string, required): Search query
- `columns` (array, required): Columns to search in
- `case_sensitive` (boolean, optional): Case sensitivity flag
- `exact_match` (boolean, optional): Exact match flag
- `page` (integer, optional): Page number
- `per_page` (integer, optional): Items per page

**Returns**: Paginated search results

**Example**:
```json
{
  "tool": "model_search",
  "arguments": {
    "model": "User",
    "query": "john",
    "columns": ["name", "email"],
    "case_sensitive": false,
    "exact_match": false,
    "page": 1,
    "per_page": 10
  }
}
```

---

### Aggregation Tools

#### `model_count`
Count all records.

**Parameters**:
- `model` (string, required): Model name

**Returns**: Total count

**Example**:
```json
{
  "tool": "model_count",
  "arguments": {
    "model": "User"
  }
}
```

#### `model_count_where`
Count records matching filter.

**Parameters**:
- `model` (string, required): Model name
- `filter` (object, required): Filter operator

**Returns**: Filtered count

**Example**:
```json
{
  "tool": "model_count_where",
  "arguments": {
    "model": "User",
    "filter": {
      "type": "Single",
      "filter": {"column": "is_active", "operator": "Eq", "value": true}
    }
  }
}
```

#### `model_aggregate`
Perform aggregate operations.

**Parameters**:
- `model` (string, required): Model name
- `function` (string, required): Aggregate function (Count, Sum, Avg, Min, Max)
- `column` (string, required): Column name
- `filter` (object, optional): Filter operator

**Returns**: Aggregate value

**Example**:
```json
{
  "tool": "model_aggregate",
  "arguments": {
    "model": "Order",
    "function": "Sum",
    "column": "total_amount",
    "filter": {
      "type": "Single",
      "filter": {"column": "status", "operator": "Eq", "value": "completed"}
    }
  }
}
```

---

### Query Builder Tools

#### `query_build`
Build and execute a custom query.

**Parameters**:
- `table` (string, required): Table name
- `select` (array, optional): Column names
- `joins` (array, optional): Join clauses
- `where` (object, optional): Filter operator
- `group_by` (array, optional): Group by columns
- `having` (object, optional): Having clause
- `order_by` (array, optional): Sort specifications
- `limit` (integer, optional): Result limit
- `offset` (integer, optional): Result offset
- `distinct` (boolean, optional): Distinct flag

**Returns**: Query results

**Example**:
```json
{
  "tool": "query_build",
  "arguments": {
    "table": "orders",
    "select": ["orders.id", "users.name", "orders.total"],
    "joins": [
      {
        "type": "Inner",
        "table": "users",
        "condition": "users.id = orders.user_id"
      }
    ],
    "where": {
      "type": "Single",
      "filter": {"column": "orders.status", "operator": "Eq", "value": "completed"}
    },
    "order_by": [
      {"column": "orders.created_at", "order": "Desc"}
    ],
    "limit": 10
  }
}
```

---

### Filter Tools

#### Filter Operators

**Single Filter**:
```json
{
  "type": "Single",
  "filter": {
    "column": "age",
    "operator": "Gt",
    "value": 18
  }
}
```

**AND Combination**:
```json
{
  "type": "And",
  "filters": [
    {
      "type": "Single",
      "filter": {"column": "is_active", "operator": "Eq", "value": true}
    },
    {
      "type": "Single",
      "filter": {"column": "age", "operator": "Ge", "value": 18}
    }
  ]
}
```

**OR Combination**:
```json
{
  "type": "Or",
  "filters": [
    {
      "type": "Single",
      "filter": {"column": "role", "operator": "Eq", "value": "admin"}
    },
    {
      "type": "Single",
      "filter": {"column": "role", "operator": "Eq", "value": "moderator"}
    }
  ]
}
```

**NOT Filter**:
```json
{
  "type": "Not",
  "filter": {
    "type": "Single",
    "filter": {"column": "status", "operator": "Eq", "value": "deleted"}
  }
}
```

#### Supported Operators

- `Eq` - Equal to
- `Ne` - Not equal to
- `Lt` - Less than
- `Le` - Less than or equal
- `Gt` - Greater than
- `Ge` - Greater than or equal
- `Like` - Pattern matching
- `NotLike` - Negative pattern matching
- `In` - In list
- `NotIn` - Not in list
- `IsNull` - Is NULL
- `IsNotNull` - Is NOT NULL
- `Between` - Between range
- `NotBetween` - Not between range

---

### Migration Tools

#### `migration_create`
Create a new migration.

**Parameters**:
- `name` (string, required): Migration name
- `sql` (string, required): Migration SQL

**Returns**: Migration object

**Example**:
```json
{
  "tool": "migration_create",
  "arguments": {
    "name": "create_users_table",
    "sql": "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL)"
  }
}
```

#### `migration_execute`
Execute a migration.

**Parameters**:
- `migration_id` (string, required): Migration ID

**Returns**: Success status

**Example**:
```json
{
  "tool": "migration_execute",
  "arguments": {
    "migration_id": "550e8400-e29b-41d4-a716-446655440000"
  }
}
```

#### `migration_rollback`
Rollback a migration.

**Parameters**:
- `migration_id` (string, required): Migration ID

**Returns**: Success status

**Example**:
```json
{
  "tool": "migration_rollback",
  "arguments": {
    "migration_id": "550e8400-e29b-41d4-a716-446655440000"
  }
}
```

#### `migration_list_pending`
List pending migrations.

**Returns**: Array of pending migrations

**Example**:
```json
{
  "tool": "migration_list_pending",
  "arguments": {}
}
```

#### `migration_list_executed`
List executed migrations.

**Returns**: Array of executed migrations

**Example**:
```json
{
  "tool": "migration_list_executed",
  "arguments": {}
}
```

#### `migration_generate_from_model`
Generate migration from model definition.

**Parameters**:
- `model` (string, required): Model name

**Returns**: Generated migration

**Example**:
```json
{
  "tool": "migration_generate_from_model",
  "arguments": {
    "model": "User"
  }
}
```

---

### Migration Template Tools

#### `migration_template_create_table`
Create table migration template.

**Parameters**:
- `table_name` (string, required): Table name
- `columns` (array, required): Column definitions

**Returns**: Migration object

**Example**:
```json
{
  "tool": "migration_template_create_table",
  "arguments": {
    "table_name": "posts",
    "columns": [
      {"name": "id", "definition": "INTEGER PRIMARY KEY AUTOINCREMENT"},
      {"name": "title", "definition": "TEXT NOT NULL"},
      {"name": "content", "definition": "TEXT"},
      {"name": "created_at", "definition": "TEXT NOT NULL"}
    ]
  }
}
```

#### `migration_template_add_column`
Add column migration template.

**Parameters**:
- `table_name` (string, required): Table name
- `column_name` (string, required): Column name
- `definition` (string, required): Column definition

**Returns**: Migration object

**Example**:
```json
{
  "tool": "migration_template_add_column",
  "arguments": {
    "table_name": "users",
    "column_name": "last_login",
    "definition": "TEXT"
  }
}
```

#### `migration_template_create_index`
Create index migration template.

**Parameters**:
- `index_name` (string, required): Index name
- `table_name` (string, required): Table name
- `columns` (array, required): Column names

**Returns**: Migration object

**Example**:
```json
{
  "tool": "migration_template_create_index",
  "arguments": {
    "index_name": "idx_users_email",
    "table_name": "users",
    "columns": ["email"]
  }
}
```

---

## Data Types

### Value Types

The ORM supports the following value types:

- **Null**: NULL value
- **Integer**: 64-bit signed integer
- **Real**: 64-bit floating point
- **Text**: UTF-8 string
- **Blob**: Binary data
- **Boolean**: Boolean (stored as INTEGER 0/1)

### Sort Order

- **Asc**: Ascending order
- **Desc**: Descending order

### Join Types

- **Inner**: INNER JOIN
- **Left**: LEFT JOIN
- **Right**: RIGHT JOIN
- **Full**: FULL JOIN

### Aggregate Functions

- **Count**: COUNT(column)
- **Sum**: SUM(column)
- **Avg**: AVG(column)
- **Min**: MIN(column)
- **Max**: MAX(column)

---

## Error Handling

The MCP server returns errors in the following format:

```json
{
  "error": {
    "type": "Error Type",
    "message": "Detailed error message"
  }
}
```

### Error Types

- **Connection**: Database connection errors
- **Sql**: SQL execution errors
- **Serialization**: Data serialization/deserialization errors
- **Validation**: Data validation errors
- **NotFound**: Resource not found errors
- **Pagination**: Pagination parameter errors
- **Query**: Query building errors
- **DatabaseError**: General database errors

---

## Usage Examples

### Example 1: Basic CRUD Operations

```json
// Create a user
{
  "tool": "model_create",
  "arguments": {
    "model": "User",
    "data": {
      "name": "Alice",
      "email": "alice@example.com",
      "age": 30,
      "is_active": true
    }
  }
}

// Find user by ID
{
  "tool": "model_find_by_id",
  "arguments": {
    "model": "User",
    "id": 1
  }
}

// Update user
{
  "tool": "model_update",
  "arguments": {
    "model": "User",
    "id": 1,
    "data": {
      "email": "alice.updated@example.com"
    }
  }
}

// Delete user
{
  "tool": "model_delete",
  "arguments": {
    "model": "User",
    "id": 1
  }
}
```

### Example 2: Complex Filtering

```json
// Find active users over 18
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

### Example 3: Pagination

```json
// Get page 2 with 20 items
{
  "tool": "model_find_paginated",
  "arguments": {
    "model": "User",
    "page": 2,
    "per_page": 20
  }
}
```

### Example 4: Search

```json
// Search for "john" in name and email fields
{
  "tool": "model_search",
  "arguments": {
    "model": "User",
    "query": "john",
    "columns": ["name", "email"],
    "page": 1,
    "per_page": 10
  }
}
```

### Example 5: Aggregation

```json
// Count active users
{
  "tool": "model_count_where",
  "arguments": {
    "model": "User",
    "filter": {
      "type": "Single",
      "filter": {"column": "is_active", "operator": "Eq", "value": true}
    }
  }
}

// Average age of users
{
  "tool": "model_aggregate",
  "arguments": {
    "model": "User",
    "function": "Avg",
    "column": "age"
  }
}
```

### Example 6: Complex Query with Joins

```json
{
  "tool": "query_build",
  "arguments": {
    "table": "orders",
    "select": ["orders.*", "users.name as user_name", "products.title as product_title"],
    "joins": [
      {
        "type": "Inner",
        "table": "users",
        "condition": "users.id = orders.user_id"
      },
      {
        "type": "Inner",
        "table": "products",
        "condition": "products.id = orders.product_id"
      }
    ],
    "where": {
      "type": "And",
      "filters": [
        {
          "type": "Single",
          "filter": {"column": "orders.status", "operator": "Eq", "value": "completed"}
        },
        {
          "type": "Single",
          "filter": {"column": "orders.created_at", "operator": "Ge", "value": "2024-01-01"}
        }
      ]
    },
    "order_by": [
      {"column": "orders.created_at", "order": "Desc"}
    ],
    "limit": 50
  }
}
```

---

## Best Practices

### 1. Connection Management
- Reuse database connections when possible
- Close connections properly after use
- Use connection pooling for high-traffic applications

### 2. Query Optimization
- Use indexes for frequently queried columns
- Avoid N+1 queries by using joins
- Use pagination for large result sets
- Use specific column selection instead of SELECT *

### 3. Data Validation
- Validate data before database operations
- Use appropriate data types
- Implement constraints at the database level

### 4. Error Handling
- Always handle errors gracefully
- Log errors for debugging
- Provide meaningful error messages to users

### 5. Migrations
- Version control migration files
- Test migrations before production deployment
- Always provide rollback scripts
- Use descriptive migration names

### 6. Security
- Use parameterized queries to prevent SQL injection
- Validate and sanitize user input
- Implement proper authentication and authorization
- Use environment variables for sensitive data

---

## Cloudflare Workers Integration

For deploying with Cloudflare Workers:

### Dependencies
```toml
[dependencies]
libsql-orm = { version = "0.2.4", features = ["cloudflare"] }
worker = { version = ">=0.7.0", features = ['http', 'axum'] }
```

### Environment Variables
- `TURSO_DATABASE_URL` - Database URL
- `TURSO_AUTH_TOKEN` - Authentication token

### Sample Worker Request
```json
{
  "method": "POST",
  "path": "/api/users",
  "body": {
    "name": "John Doe",
    "email": "john@example.com"
  }
}
```

---

## Performance Considerations

### Batch Operations
Use bulk operations for multiple inserts/updates to reduce round trips:
- `model_bulk_create` for multiple inserts
- `model_bulk_update` for multiple updates
- `model_bulk_delete` for multiple deletes

### Pagination
- Use cursor-based pagination for large datasets
- Offset-based pagination is simpler but less efficient for large offsets

### Indexing
- Create indexes on frequently queried columns
- Use composite indexes for multi-column queries
- Monitor index usage and remove unused indexes

### Caching
- Cache frequently accessed data
- Invalidate cache on data updates
- Use edge caching with Cloudflare Workers

---

## Limitations

1. **WASM Environment**:
   - Limited to libsql/Turso databases
   - Some native database features may not be available

2. **Boolean Type**:
   - Stored as INTEGER (0/1) in SQLite
   - Automatic conversion provided

3. **Transaction Support**:
   - Manual transaction handling in WASM
   - Use BEGIN/COMMIT for atomic operations

4. **Last Insert ID**:
   - Not available in WASM environments
   - Placeholder value returned

---

## Additional Resources

- **Documentation**: https://docs.rs/libsql-orm
- **Repository**: https://github.com/ayonsaha2011/libsql-orm
- **Turso Database**: https://turso.tech/
- **Cloudflare Workers**: https://workers.cloudflare.com/

---

## Support

For issues, questions, or contributions:
- GitHub Issues: https://github.com/ayonsaha2011/libsql-orm/issues
- GitHub Discussions: https://github.com/ayonsaha2011/libsql-orm/discussions

---

**Last Updated**: 2025-12-29
**Documentation Version**: 1.0.0
