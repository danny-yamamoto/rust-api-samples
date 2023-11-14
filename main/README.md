### setup
```bash
cargo install sqlx-cli --no-default-features --features sqlite
```

```sql
vscode ➜ /workspaces/rust-api-samples/users (main) $ sqlite3 local.db
```

```sql
CREATE TABLE IF NOT EXISTS users ( user_id INTEGER PRIMARY KEY, email_address TEXT, created_at INTEGER, deleted INTEGER, settings TEXT);
INSERT INTO users (user_id, email_address, created_at, deleted, settings) VALUES (1, 'maria@example.com', 0, 0, '');
INSERT INTO users (user_id, email_address, created_at, deleted, settings) VALUES (999, 'admin@example.com', 0, 0, '');
```

```bash
export DATABASE_URL="sqlite:./local.db"
echo $DATABASE_URL
```

```bash
vscode ➜ /workspaces/rust-api-samples (main) $ curl "localhost:8080/users?user_id=1" -i
HTTP/1.1 200 OK
content-type: application/json
content-length: 90
date: Tue, 14 Nov 2023 09:53:56 GMT

{"user_id":1,"email_address":"maria@example.com","created_at":0,"deleted":0,"settings":""}
vscode ➜ /workspaces/rust-api-samples (main) $
```
