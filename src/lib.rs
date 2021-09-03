//! funes is a server to mock API responses. You might use it to:
//! - test applications without hitting production resources;
//! - create integrations tests for your applications;
//!
//! # Installation
//!
//! Install funes in your `Cargo.toml` alongside your preferable [async
//! runtime](https://rust-lang.github.io/async-book/08_ecosystem/00_chapter.html):
//!
//! ```toml
//! [dependencies]
//! actix-web = "4.0.0-beta.8"
//! funes = "0.1.9"
//! ```
//!
//! Create a new instance of a funes server in your `main.rs`:
//!
//! ```ignore
//! use funes;
//!
//! #[actix_web::main]
//! async fn main() -> std::io::Result<()> {
//!     Ok(funes::server::new().await?)
//! }
//! ```
//!
//! # Mocking Requests
//!
//! Now your application is ready to receive requests and to store the
//! responses. To test it, run your funes server in one terminal and make
//! requests from another one:
//!
//! ```sh
//! cargo run
//!
//! [2021-08-31T23:21:33Z INFO  funes::server] Server running in 0.0.0.0:8080
//! [2021-08-31T23:21:33Z INFO  funes::server] Mocks directory is ./mocks
//! [2021-08-31T23:21:33Z INFO  funes::server] Calling externals apis is allowed? true
//! ```
//!
//! To mock the requests of an API, call the endpoint `http://localhost:8080/{API}`.
//! The first request will hit the API and then store the response:
//!
//! ```sh
//! curl http://localhost:8080/pokeapi.co/api/v2/pokemon/1
//!
//! [2021-08-30T23:16:17Z DEBUG funes::handlers] File not found! For api: pokeapi.co/api/v2/pokemon/1, resource: ./mocks/16709024112015907760.json
//! [2021-08-30T23:16:17Z DEBUG funes::fetch] External get to: http://pokeapi.co/api/v2/pokemon/1
//! [2021-08-30T23:16:17Z DEBUG funes::utils] Write filename: ./mocks/16709024112015907760.json
//! [2021-08-30T23:16:17Z INFO  actix_web::middleware::logger] 201 0.355466 GET /pokeapi.co/api/v2/pokemon/1 HTTP/1.1 curl/7.64.1 bytes:199394
//! ```
//!
//! The second request and all the subsequent ones will be served from
//! the stored response:
//!
//! ```sh
//! curl http://localhost:8080/pokeapi.co/api/v2/pokemon/1
//!
//! [2021-08-30T23:16:50Z INFO  actix_web::middleware::logger] 200 0.000206 GET /pokeapi.co/api/v2/pokemon/1 HTTP/1.1 curl/7.64.1 bytes:199394
//! ```
//!
//! Posts are supported:
//!
//! ```sh
//! curl \
//!   -d '{ "userId": 1, "id": 1, "title": "title", "body": "body" }' \
//!   -H "Content-Type: application/json" \
//!   -X POST 0.0.0.0:8080/jsonplaceholder.typicode.com/posts
//!
//! [2021-09-01T04:25:01Z DEBUG funes::handlers] File not found! For api: jsonplaceholder.typicode.com/posts, resource: ./mocks/768531861528487606.json, payload_post: {"body":"body","id":1,"title":"title","userId":1}
//! [2021-09-01T04:25:01Z DEBUG funes::fetch] External post to: http://jsonplaceholder.typicode.com/posts
//! [2021-09-01T04:25:02Z DEBUG funes::utils] Write filename: ./mocks/768531861528487606.json
//! [2021-09-01T04:25:02Z INFO  actix_web::middleware::logger] 201 0.293888 POST /jsonplaceholder.typicode.com/posts HTTP/1.1 curl/7.64.1 bytes:51
//! ```
//!
//! # Routes
//!
//! These are the default routes of a funes app:
//!
//! - `localhost:8080/{api_to_mock}` - Like
//!   `localhost:8080/pokeapi.co/api/v2/pokemon/1`
//! - `localhost:8080/mocks` - List of all saved mocks
//! - `localhost:8080/health`
//! - `localhost:8080/resource-status`
//!
//! # Default Envs
//!
//! ```sh
//! RUST_ALLOW_EXTERNALS=true
//! RUST_APP=funes
//! RUST_HOST=0.0.0.0:8080
//! RUST_LOG=funes,actix_web=info
//! RUST_MOCK_DIR="./mocks"
//! ```
//!
//! # Examples
//!
//! All examples can be found in the
//! [examples](https://github.com/rodmoioliveira/funes/tree/main/examples) folder:
//!
//! - [minimal](https://github.com/rodmoioliveira/funes/tree/main/examples/minimal)
//! - [docker](https://github.com/rodmoioliveira/funes/tree/main/examples/docker)
//!
//! # Benchmarks
//!
//! Funes is really fast! Take a look:
//!
//! ```sh
//! wrk -t2 -c10 -d60s http://localhost:8080/pokeapi.co/api/v2/pokemon/1
//!
//! Running 1m test @ http://localhost:8080/pokeapi.co/api/v2/pokemon/1
//!   2 threads and 10 connections
//!   Thread Stats   Avg      Stdev     Max   +/- Stdev
//!     Latency    14.63ms  106.45ms   1.29s    97.81%
//!     Req/Sec    11.12k     1.69k   13.18k    86.18%
//!   1298872 requests in 1.00m, 241.39GB read
//! Requests/sec:  21630.87
//! Transfer/sec:      4.02GB
//! ```

mod config;
mod error;
mod fetch;
mod format;
mod handlers;
mod io;
mod statics;
mod utils;

pub mod server;
