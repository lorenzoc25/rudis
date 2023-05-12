# (WIP) Rudis
A mini version of redis that support http interface implemented in Rust. The in-memorry kv-storage is sharded and concurrent safe. Inspired by [Tokio's tutorial](https://tokio.rs/tokio/tutorial) and [Webdis](https://github.com/nicolasff/webdis)

This is a still work-in-progress project. The goal is to learn Rust and Tokio and is not meant to be used in production.
## Endpoints
```
Get: <your-url>/GET/<key>
Setr: <your-url>/SET/<key>/<value>
```

## Todo
- support post request with json as arguments
- set argument's TTL
- general purpose client, and build http interface on top of that
- .rdb file generation and backup
- authentication

