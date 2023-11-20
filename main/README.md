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
export SERVICE_ACCOUNT="/workspaces/rust-api-samples/main/key.json"
echo $DATABASE_URL
echo $SERVICE_ACCOUNT
```

```bash
cargo sqlx prepare --database-url "sqlite:./local.db"
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

```bash
vscode ➜ /workspaces/rust-api-samples (main) $ curl "localhost:8080/storage?bucket=sanbox-334000_bucket&object=test.html" -i
HTTP/1.1 200 OK
content-type: application/json
content-length: 304
date: Wed, 15 Nov 2023 04:52:22 GMT

{"content":"<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"UTF-8\">\n    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n    <title>Document</title>\n</head>\n<body>\n    test\n</body>\n</html>"}
```

```bash
vscode ➜ /workspaces/rust-api-samples (main) $ curl "localhost:8080/storage?bucket=sanbox-334000_bucket&object=test.htm" -i
HTTP/1.1 500 Internal Server Error
content-type: application/json
content-length: 60
date: Wed, 15 Nov 2023 04:52:32 GMT

"Other(\"No such object: sanbox-334000_bucket/test.htm\")\n"
vscode ➜ /workspaces/rust-api-samples (main) $
```

```sql
CREATE TABLE IF NOT EXISTS users ( user_id INTEGER PRIMARY KEY, email_address TEXT, created_at INTEGER, deleted INTEGER, settings TEXT);
INSERT INTO users (user_id, email_address, created_at, deleted, settings) VALUES (1, 'maria@example.com', 0, 0, '');
INSERT INTO users (user_id, email_address, created_at, deleted, settings) VALUES (100, 'alex@example.com', 1, 0, '');
INSERT INTO users (user_id, email_address, created_at, deleted, settings) VALUES (10000, 'marc@example.com', 0, 1, '');
```
