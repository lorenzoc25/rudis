# Rudis
A mini version of redis server that provides http interface implemented in Rust. The in-memorry kv-storage is sharded and concurrent safe. Inspired by [Tokio's tutorial](https://tokio.rs/tokio/tutorial) and [Webdis](https://github.com/nicolasff/webdis)

This is a still work-in-progress project and is not meant to be used in production(yet). Only basic commands like GET and SET are supported. More commands will be added in the future.

There is also the [Experimental Version](https://github.com/lorenzoc25/rudis/tree/experimental) which adapatively uses `io_uring` on supported kernels. If kernel does not support `io_uring`, it will fall back to using `epoll/kqueue`. This version is not published to the crates and therefore needs to be cloned and built locally. It requires **nightly rust compiler** to be built.

## Installation
```sh
cargo install rudis-http
```
## Usage
To run the server, simply run
```sh
$ rudis-http
```
or you can optionally specify the address, num shards (more to be added later)
```sh
$ rudis-http -n <num_shards> -a <address_to_listen_on>

# to view all commands, do
$ rudis-http --help
```
And the server will be listening on the port you specified or `127.0.0.1:6379` by default.

Once the server is up and running, its service can be accessed via http GET request. The following are the supported requests:
```
GET: <your-url>/GET/<key> 
SET: <your-url>/SET/<key>/<value>
```
The response will be in json format. For a SET, you will be getting the status of this command like
```sh
$ curl 'localhost:6379/set/hello/world'
{"SET": "OK"}

# or if SET's arguments are not correct
$ curl 'localhost:6379/set/hello'
{"SET": "Invalid"}
```
You can also send a POST request with json as key value pair to support a SET command. 
```sh
$ curl -X POST 'localhost:6379/set' -d '{"hello":"world"}'
{"SET": "OK"}

$ curl -X POST 'localhost:6379/set' -d '{}'
{"SET": "Invalid"}
# for multiple kv pairs
$ curl -X POST 'localhost:6379/set' -d '{"hello":"world", "foo":"bar"}'
{"SET": "OK}
```
and for get, you will be getting the key value pair if there's a match, or an empty json object if there's no match
```sh
$ curl 'localhost:6379/get/hello'     
{"hello":"world"}

# if key does not exist
$ curl 'localhost:6379/GET/123'
{}

# if GET's arguments are not correct
$ curl 'localhost:6379/GET'
{"GET": "Invalid"}
```
Any other types of commands will be responded with an empty json object

## Todo
- more cli options
- set argument's TTL
- general purpose client, and build http interface on top of that
- .rdb file generation and backup
- authentication
### Finished Todos
- cachepadded for mutex
- support post request with json as arguments 
