# rustbook-log-collector

[『実践Rust入門』](https://gihyo.jp/book/2019/978-4-297-10559-4)の第11章におけるlog-collectorを

- Actix Web 4
- Diesel 2
- SQLite

で書き直したものです。server、api、cliのうちcliは実装していません。

SQLiteとDiesel CLIをインストールしたうえで次の手順で動かします。

```
$ echo DATABASE_URL=server/log-collector.db > .env
$ (cd server; diesel setup --database-url=log-collector.db)
$ cargo r
```
