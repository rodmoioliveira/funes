> Lo recuerdo (yo no tengo derecho a pronunciar ese verbo sagrado, sólo un
> hombre en la tierra tuvo derecho y ese hombre ha muerto) con una oscura
> pasionaria en la mano, viéndola como nadie la ha visto, aunque la mirara desde
> el crepúsculo del día hasta el de la noche, toda una vida entera.

*Jorge Luis Borges - Funes, el memorioso*

# Funes

![pipeline workflow](https://github.com/rodmoioliveira/funes/actions/workflows/rust.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/funes.svg)](https://crates.io/crates/funes)
[![docs.rs](https://docs.rs/funes/badge.svg)](https://docs.rs/funes)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/rodmoioliveira/funes/blob/main/LICENSE)
[![Changelog](https://camo.githubusercontent.com/4d89fc2186d69bdbb2c6ea6cb54ab16915be5e5e0b63a393e87a75741f1baa8c/68747470733a2f2f696d672e736869656c64732e696f2f62616467652f6368616e67656c6f672d4348414e47454c4f472e6d642d253233453035373335)](https://github.com/rodmoioliveira/funes/blob/main/CHANGELOG.md)
[![Code of Conduct](https://img.shields.io/badge/code-of%20conduct-blue.svg)](https://github.com/rodmoioliveira/funes/blob/main/CODE_OF_CONDUCT.md)

funes is a server to mock API responses. You might use it to:

- test applications without hitting production resources;
- create integrations tests for your applications;

# Installation

To install funes, you must have [rust and
cargo](https://www.rust-lang.org/tools/install) installed. Then you can run:

```sh
cargo install funes
```

# Usage

Now you can run your funes server in one terminal and make requests from another
one:

```sh
# terminal 1
funes

[2021-09-09T04:45:50Z INFO  funes::server] ENVS: Envs { allow_externals: true, api_regex: ".+", h_server: "funes", h_user_agent: "funes", latency_collection: "none", latency_enable: false, localhost: "0.0.0.0:8080", log: "funes,actix_web=info", mock_dir: "/Users/rodolfo.moi/.mocks" }, LATENCY_COLLECTION: {}
```

To mock the requests of an `{api}`, call the endpoint
`http://localhost:8080/{api}`. The first request will hit the `{api}` and then
store the response:

```sh
# terminal 2
curl http://localhost:8080/jsonplaceholder.typicode.com/todos/1

# terminal 1
[2021-09-08T23:42:27Z INFO  actix_web::middleware::logger] 201 0.125973 GET /jsonplaceholder.typicode.com/todos/1 HTTP/1.1 curl/7.64.1 bytes:66
```

The second request and all the subsequent ones will be served from the stored
response:

```sh
# terminal 2
curl http://localhost:8080/jsonplaceholder.typicode.com/todos/1

# terminal 1
[2021-09-08T23:43:06Z INFO  actix_web::middleware::logger] 200 0.000330 GET /jsonplaceholder.typicode.com/todos/1 HTTP/1.1 curl/7.64.1 bytes:66
```

Posts are supported:

```sh
# terminal 2
curl \
  -d '{ "userId": 1, "id": 1, "title": "title", "body": "body" }' \
  -H "Content-Type: application/json" \
  -X POST 0.0.0.0:8080/jsonplaceholder.typicode.com/posts

# terminal 1
[2021-09-01T04:25:02Z INFO  actix_web::middleware::logger] 201 0.293888 POST /jsonplaceholder.typicode.com/posts HTTP/1.1 curl/7.64.1 bytes:51
```

# Endpoints

These are the endpoints of a funes app:

- `localhost:8080/{api}` - Like `localhost:8080/pokeapi.co/api/v2/pokemon/1`
- `localhost:8080/mocks` - List of all saved mocks
- `localhost:8080/health`
- `localhost:8080/resource-status`

# Default Envs

```sh
FUNES_ALLOW_EXTERNALS=true
FUNES_API_REGEX=".+"
FUNES_APP=funes
FUNES_HOST=0.0.0.0:8080
FUNES_LATENCY_COLLECTION=""
FUNES_LOG=funes,actix_web=info
FUNES_MOCK_DIR=$HOME/.mocks
```

# Examples

All examples can be found in the
[examples](https://github.com/rodmoioliveira/funes/tree/main/examples) folder:

- [docker](https://github.com/rodmoioliveira/funes/tree/main/examples/docker)
- [latency-collection](https://github.com/rodmoioliveira/funes/tree/main/examples/latency-collection)

# Benchmarks

Funes is built on top of [actix](https://actix.rs/) and is blazing fast! Take a
look:

```sh
# https://github.com/wg/wrk
wrk -t2 -c10 -d60s http://localhost:8080/pokeapi.co/api/v2/pokemon/1

Running 1m test @ http://localhost:8080/pokeapi.co/api/v2/pokemon/1
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    14.63ms  106.45ms   1.29s    97.81%
    Req/Sec    11.12k     1.69k   13.18k    86.18%
  1298872 requests in 1.00m, 241.39GB read
Requests/sec:  21630.87
Transfer/sec:      4.02GB
```
