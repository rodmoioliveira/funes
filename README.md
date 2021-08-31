> I remember him with a dark passion flower in his hand, seeing it as no one has ever seen it, though he might look at it from the twilight of dawn till that of evening, a whole lifetime.

*Jorge Luis Borges - Funes, The Memorious*

# Funes

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Changelog](https://camo.githubusercontent.com/4d89fc2186d69bdbb2c6ea6cb54ab16915be5e5e0b63a393e87a75741f1baa8c/68747470733a2f2f696d672e736869656c64732e696f2f62616467652f6368616e67656c6f672d4348414e47454c4f472e6d642d253233453035373335)](CHANGELOG.md)
[![Code of Conduct](https://img.shields.io/badge/code-of%20conduct-blue.svg)](CODE_OF_CONDUCT.md)

funes is a server to mock API responses. You might use it to:
- test applications without hitting production resources;
- create integrations tests for your applications;

## Installation

Install funes in your `Cargo.toml` alongside your preferable [async
runtime](https://rust-lang.github.io/async-book/08_ecosystem/00_chapter.html):

```rs
[dependencies]
actix-web = "4.0.0-beta.8"
funes = { git = "https://github.com/rodmoioliveira/funes", branch = "main" }
```

Create a new instance of a funes server in your `main.rs`:

```rs
use funes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Ok(funes::server::new().await?)
}
```

## Mocking Requests

Now your application is ready to receive requests and to store the responses. To
test it, run your funes server in one terminal and make requests from another
one:

```sh
# terminal 1
cargo run

    Finished dev [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/funes`
[2021-08-30T22:59:45Z INFO  funes::server] Server running in 0.0.0.0:8080
[2021-08-30T22:59:45Z WARN  funes::server] Calling externals apis is allowed? true
```

To mock the requests of an API, call the endpoint `http://localhost:8080/{API}`.
The first request will hit the API and then store the response:

```sh
# terminal 2
curl http://localhost:8080/pokeapi.co/api/v2/pokemon/1

# terminal 1
[2021-08-30T23:16:17Z DEBUG funes::handlers] File not found! For api: pokeapi.co/api/v2/pokemon/1, resource: ./mocks/16709024112015907760.json
[2021-08-30T23:16:17Z DEBUG funes::fetch] External get to: http://pokeapi.co/api/v2/pokemon/1
[2021-08-30T23:16:17Z DEBUG funes::utils] Write filename: ./mocks/16709024112015907760.json
[2021-08-30T23:16:17Z INFO  actix_web::middleware::logger] 201 0.355466 GET /pokeapi.co/api/v2/pokemon/1 HTTP/1.1 curl/7.64.1 bytes:199394
```

The second request and all the subsequent ones will be served from
the stored response:

```sh
# terminal 2
curl http://localhost:8080/pokeapi.co/api/v2/pokemon/1

# terminal 1
[2021-08-30T23:16:50Z INFO  actix_web::middleware::logger] 200 0.000206 GET /pokeapi.co/api/v2/pokemon/1 HTTP/1.1 curl/7.64.1 bytes:199394
```

## Routes

These are the default routes of a funes app:

- `localhost:8080/{api_to_mock}` - Like `localhost:8080/pokeapi.co/api/v2/pokemon/1`
- `localhost:8080/mocks` - List of all saved mocks
- `localhost:8080/health`
- `localhost:8080/resource-status`

## Examples

All examples can be found in the [examples](examples/) folder:

- [minimal](examples/minimal/)
- [docker](examples/docker/)

## Benchmarks

Funes is really fast! Take a look:

```sh
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
