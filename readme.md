# (WIP) Rudis
A mini version of redis that support http interface implemented in Rust. The in-memorry kv-storage is sharded and concurrent safe. Inspired by [Tokio's tutorial](https://tokio.rs/tokio/tutorial) and [Webdis](https://github.com/nicolasff/webdis)

This is a still work-in-progress project. The goal is to learn Rust and Tokio and is not meant to be used in production.
## Usage
To run the server, simply run
```
$ rudis 
```
or you can optionally specify the port
```
$ rudis <port>
```
And the server will be listening on the port you specified or `127.0.0.1:6379` by default.

Once the server isup and running, its service can be accessed via http request. The following are the supported requests:
```
Get: <your-url>/GET/<key>
Set: <your-url>/SET/<key>/<value>
```
The response will be in json format. For a SET, you will be getting the status of this command like
```
$ curl 'localhost:6379/set/hello/world'
{"SET": "OK"}
```
and for get, you will be getting the key value pair if there's a match, or an empty json object if there's no match
```
$ curl 'localhost:6379/get/hello'     
{"hello":"world"}

$ curl 'localhost:6379/GET/123'
{}
```

## Todo
- support post request with json as arguments
- set argument's TTL
- general purpose client, and build http interface on top of that
- .rdb file generation and backup
- authentication

