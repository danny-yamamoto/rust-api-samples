## `/users`

### setup
```bash
cargo install sqlx-cli --no-default-features --features sqlite
cargo add sqlx --features "sqlite runtime-tokio-native-tls chrono"
cargo add tokio --features=full
cargo add dotenv
sqlx database create --database-url "sqlite:./local.db"
sqlx migrate add -r create_table
sqlx migrate run --database-url sqlite:./local.db
```

```bash
export DATABASE_URL="sqlite:./local.db"
echo $DATABASE_URL
```


```sql
vscode ➜ /workspaces/rust-api-samples/users (main) $ sqlite3 local.db
SQLite version 3.34.1 2021-01-20 14:10:07
Enter ".help" for usage hints.
sqlite> .tables
_sqlx_migrations  users           
sqlite> pragma table_info(users);
0|user_id|INTEGER|0||1
1|email_address|TEXT|0||0
2|created_at|INTEGER|0||0
3|deleted|INTEGER|0||0
4|settings|TEXT|0||0
sqlite> .exit
vscode ➜ /workspaces/rust-api-samples/users (main) $
```

- database up
```sql
CREATE TABLE IF NOT EXISTS users ( user_id INTEGER PRIMARY KEY, email_address TEXT, created_at INTEGER, deleted INTEGER, settings TEXT);
INSERT INTO users (user_id, email_address, created_at, deleted, settings) VALUES (1, 'maria@example.com', 0, 0, '');
INSERT INTO users (user_id, email_address, created_at, deleted, settings) VALUES (999, 'admin@example.com', 0, 0, '');
```

- database down
```sql
DROP TABLE IF EXISTS users;
```

## test
```bash
curl "localhost:8080/users?user_id=1" -i
```

# note
- `i64` の理由
- `IntoResponse` の使い方
- `cargo sqlx prepare --database-url "sqlite:./local.db"` の意味
