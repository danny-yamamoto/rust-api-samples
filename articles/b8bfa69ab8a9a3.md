---
title: "Rust vs. Go: Building & Comparing REST APIs for Cloud Storage"
emoji: "🐷"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["rust", "go", "googlecloudstorage"]
published: false
---
Rust の Production での実装について、他社の利用状況を見ると、Web App に導入していけそう。

あとの問題は、team や会社の skill set をどうするか。Ops できるか。

Cloud Functions など、runtime に依存するものはまだ動かせない。Cloud Run など container service では動かせる。

microservices 関連で考えると、Otel への対応が気になる。dependencies に追加することで実装は可能かもしれない。この辺りは別途検証したい。

まだまだ、ecosystem は不足している部分はある。今後の Rust ecosystem に期待。

Rust の Code を流用する場合は、dependencies のチェックが必要。変なものが紛れていないか。

コードはこちら。

https://github.com/danny-yamamoto/rust-api-samples

