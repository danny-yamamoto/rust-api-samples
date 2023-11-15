## `/storage`
- [Crate cloud_storage](https://docs.rs/cloud-storage/latest/cloud_storage/)

```bash
curl "localhost:8080/storage?bucket=sanbox-334000_bucket&object=test.txt" -i
curl "localhost:8080/storage?bucket=sanbox-334000_bucket&object=test.html" -i
```

```bash
vscode ➜ /workspaces/rust-api-samples (main) $ curl "localhost:8080/storage?bucket=sanbox-334000_bucket&object=test.txt" -i
HTTP/1.1 500 Internal Server Error
content-type: application/json
content-length: 95
date: Sun, 05 Nov 2023 03:21:47 GMT

{"content":"Failed to read object: Other(\"No such object: sanbox-334000_bucket/test.txt\")\n"}
vscode ➜ /workspaces/rust-api-samples (main) $ curl "localhost:8080/storage?bucket=sanbox-334000_bucket&object=test.html" -i
HTTP/1.1 200 OK
content-type: application/json
content-length: 304
date: Sun, 05 Nov 2023 03:21:55 GMT

{"content":"<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"UTF-8\">\n    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n    <title>Document</title>\n</head>\n<body>\n    test\n</body>\n</html>"}
vscode ➜ /workspaces/rust-api-samples (main) $ 
```

## note
- `impl` の用法
- `toko` の用法
- `Serialize` / `Deserialize`
- `async` の用法
- response での struct の用法
- `Router` の `get` は HTTP Method
- response の JsonRejection
