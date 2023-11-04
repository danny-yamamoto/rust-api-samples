---
title: ""
emoji: "🐷"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: []
published: false
---
他社の利用状況を見ると、Web App には導入していけそう。

あとの問題は、team や会社の skill set をどうするか。Ops できるか。

Cloud Functions など、runtime に依存するものはまだ動かせない。Cloud Run など container service では動かせる。

microservices 関連で考えると、Otel への対応が気になる。dependencies に追加することで実装は可能かもしれない。この辺りは別途検証したい。

まだまだ、ecosystem は不足している部分はある。

Code を流用する場合は、dependencies のチェックが必要。変なものが紛れていないか。

