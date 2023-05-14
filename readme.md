# (WIP) Rudis
A mini version of redis server that provides http interface implemented in Rust. The in-memorry kv-storage is sharded and concurrent safe. Inspired by [Tokio's tutorial](https://tokio.rs/tokio/tutorial) and [Webdis](https://github.com/nicolasff/webdis)

This is a still work-in-progress project and is not meant to be used in production(yet). Only basic commands like GET and SET are supported. More commands will be added in the future.

## Installation
```
cargo install rudis-http
```
## Usage
To run the server, simply run
```
$ rudis-http
```
or you can optionally specify the address, num shards (more to be added later)
```
$ rudis-http -n <num_shards> -a <address_to_listen_on>

// to view all commands, do
$ rudis-http --help
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

// or if SET's arguments are not correct
$ curl 'localhost:6379/set/hello'
{"SET": "Invalid"}
```
and for get, you will be getting the key value pair if there's a match, or an empty json object if there's no match
```
$ curl 'localhost:6379/get/hello'     
{"hello":"world"}

// if key does not exist
$ curl 'localhost:6379/GET/123'
{}

// if GET's arguments are not correct
$ curl 'localhost:6379/GET'
{"GET": "Invalid"}
```
Any other types of commands will be responded with an empty json object

## Todo
- support post request with json as arguments
- more cli options
- set argument's TTL
- general purpose client, and build http interface on top of that
- .rdb file generation and backup
- authentication
### Finsihed Todos
- cachepadded for mutex
