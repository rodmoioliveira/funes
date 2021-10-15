//! > Lo recuerdo (yo no tengo derecho a pronunciar ese verbo sagrado, sólo un
//! > hombre en la tierra tuvo derecho y ese hombre ha muerto) con una oscura
//! > pasionaria en la mano, viéndola como nadie la ha visto, aunque la mirara
//! > desde el crepúsculo del día hasta el de la noche, toda una vida entera.
//!
//! *Jorge Luis Borges - Funes, el memorioso*
//!
//! ---
//!
//! # Funes
//!
//! ![pipeline workflow](https://github.com/rodmoioliveira/funes/actions/workflows/ci.yml/badge.svg)
//! [![crates.io](https://img.shields.io/crates/v/funes.svg)](https://crates.io/crates/funes)
//! [![docs.rs](https://docs.rs/funes/badge.svg)](https://docs.rs/funes)
//! [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/rodmoioliveira/funes/blob/main/LICENSE)
//! [![Changelog](https://camo.githubusercontent.com/4d89fc2186d69bdbb2c6ea6cb54ab16915be5e5e0b63a393e87a75741f1baa8c/68747470733a2f2f696d672e736869656c64732e696f2f62616467652f6368616e67656c6f672d4348414e47454c4f472e6d642d253233453035373335)](https://github.com/rodmoioliveira/funes/blob/main/CHANGELOG.md)
//! [![Code of Conduct](https://img.shields.io/badge/code-of%20conduct-blue.svg)](https://github.com/rodmoioliveira/funes/blob/main/CODE_OF_CONDUCT.md)
//!
//! funes is a server to mock HTTP responses. You might use it to:
//!
//! - test applications without hitting production resources;
//! - create integrations tests for your applications;
//!
//! # Installation
//!
//! To install funes, you must have [rust and
//! cargo](https://www.rust-lang.org/tools/install) installed. Then you can run:
//!
//! ```sh
//! cargo install funes
//! ```
//!
//! # Usage
//!
//! Now you can run your funes server in one terminal and make requests from
//! another one:
//!
//! ```sh
//! # terminal 1
//! funes
//!
//! [2021-09-09T04:45:50Z INFO  funes::server] ENVS: Envs { allow_externals: true, api_regex: ".+", h_server: "funes", h_user_agent: "funes", latency_collection: "none", latency_enable: false, localhost: "0.0.0.0:8080", log: "funes,actix_web=info", mock_dir: "/Users/rodolfo.moi/.funes" }, LATENCY_COLLECTION: {}
//! ```
//!
//! To mock the requests of an `{api}`, call the endpoint
//! `http://localhost:8080/{api}`. The first request will hit the `{api}` and then
//! store the response:
//!
//! ```sh
//! # terminal 2
//! curl http://localhost:8080/jsonplaceholder.typicode.com/todos/1
//!
//! # terminal 1
//! [2021-09-08T23:42:27Z INFO  actix_web::middleware::logger] 201 0.125973 GET /jsonplaceholder.typicode.com/todos/1 HTTP/1.1 curl/7.64.1 bytes:66
//! ```
//!
//! The second request and all the subsequent ones will be served from the
//! stored response:
//!
//! ```sh
//! # terminal 2
//! curl http://localhost:8080/jsonplaceholder.typicode.com/todos/1
//!
//! # terminal 1
//! [2021-09-08T23:43:06Z INFO  actix_web::middleware::logger] 200 0.000330 GET /jsonplaceholder.typicode.com/todos/1 HTTP/1.1 curl/7.64.1 bytes:66
//! ```
//!
//! Posts are supported:
//!
//! ```sh
//! # terminal 2
//! curl \
//!   -d '{ "userId": 1, "id": 1, "title": "title", "body": "body" }' \
//!   -H "Content-Type: application/json" \
//!   -X POST 0.0.0.0:8080/jsonplaceholder.typicode.com/posts
//!
//! # terminal 1
//! [2021-09-01T04:25:02Z INFO  actix_web::middleware::logger] 201 0.293888 POST /jsonplaceholder.typicode.com/posts HTTP/1.1 curl/7.64.1 bytes:51
//! ```
//!
//! # Endpoints
//!
//! These are the endpoints of a funes app:
//!
//! - `localhost:8080/{api}` - Like `localhost:8080/pokeapi.co/api/v2/pokemon/1`
//! - `localhost:8080/mocks` - List of all saved mocks
//! - `localhost:8080/health`
//! - `localhost:8080/resource-status`
//!
//! # Default Envs
//!
//! ```sh
//! FUNES_ALLOW_EXTERNALS=true
//! FUNES_APP=funes
//! FUNES_HOST=0.0.0.0:8080
//! FUNES_LOG=funes,actix_web=info
//! FUNES_MOCK_DIR=$HOME/.funes
//! ```
//!
//! # Examples
//!
//! All examples can be found in the
//! [examples](https://github.com/rodmoioliveira/funes/tree/main/examples) folder:
//!
//! - [docker](https://github.com/rodmoioliveira/funes/tree/main/examples/docker)
//! - [latency-collection](https://github.com/rodmoioliveira/funes/tree/main/examples/latency-collection)
//!
//! # Benchmarks
//!
//! Funes is built on top of [actix](https://actix.rs/) and is quite fast! Take a
//! look:
//!
//! ```sh
//! # https://github.com/giltene/wrk2
//! wrk2 -t2 -c10 -d60s -R2000 --latency http://localhost:8080/pokeapi.co/api/v2/pokemon/1
//!
//! Running 1m test @ http://localhost:8080/pokeapi.co/api/v2/pokemon/1
//!   2 threads and 10 connections
//!   Thread calibration: mean lat.: 1.449ms, rate sampling interval: 10ms
//!   Thread calibration: mean lat.: 1.469ms, rate sampling interval: 10ms
//!   Thread Stats   Avg      Stdev     Max   +/- Stdev
//!     Latency     1.43ms  588.92us   6.26ms   66.00%
//!     Req/Sec     1.05k   146.60     1.67k    73.83%
//!   Latency Distribution (HdrHistogram - Recorded Latency)
//!  50.000%    1.39ms
//!  75.000%    1.83ms
//!  90.000%    2.20ms
//!  99.000%    2.94ms
//!  99.900%    3.58ms
//!  99.990%    4.31ms
//!  99.999%    6.23ms
//! 100.000%    6.26ms
//!
//!   Detailed Percentile spectrum:
//!        Value   Percentile   TotalCount 1/(1-Percentile)
//!
//!        0.209     0.000000            1         1.00
//!        0.676     0.100000        10025         1.11
//!        0.877     0.200000        20009         1.25
//!        1.065     0.300000        30007         1.43
//!        1.233     0.400000        39993         1.67
//!        1.393     0.500000        50000         2.00
//!        1.474     0.550000        54997         2.22
//!        1.557     0.600000        59988         2.50
//!        1.645     0.650000        64995         2.86
//!        1.735     0.700000        69983         3.33
//!        1.829     0.750000        74984         4.00
//!        1.878     0.775000        77482         4.44
//!        1.929     0.800000        80006         5.00
//!        1.982     0.825000        82477         5.71
//!        2.042     0.850000        84989         6.67
//!        2.113     0.875000        87505         8.00
//!        2.151     0.887500        88721         8.89
//!        2.197     0.900000        89961        10.00
//!        2.249     0.912500        91232        11.43
//!        2.307     0.925000        92483        13.33
//!        2.371     0.937500        93726        16.00
//!        2.407     0.943750        94329        17.78
//!        2.447     0.950000        94972        20.00
//!        2.491     0.956250        95597        22.86
//!        2.537     0.962500        96213        26.67
//!        2.595     0.968750        96846        32.00
//!        2.629     0.971875        97146        35.56
//!        2.665     0.975000        97453        40.00
//!        2.703     0.978125        97764        45.71
//!        2.753     0.981250        98085        53.33
//!        2.809     0.984375        98397        64.00
//!        2.841     0.985938        98556        71.11
//!        2.875     0.987500        98703        80.00
//!        2.919     0.989062        98861        91.43
//!        2.967     0.990625        99017       106.67
//!        3.027     0.992188        99175       128.00
//!        3.057     0.992969        99252       142.22
//!        3.083     0.993750        99326       160.00
//!        3.117     0.994531        99406       182.86
//!        3.157     0.995313        99482       213.33
//!        3.209     0.996094        99561       256.00
//!        3.239     0.996484        99601       284.44
//!        3.271     0.996875        99639       320.00
//!        3.297     0.997266        99677       365.71
//!        3.335     0.997656        99719       426.67
//!        3.375     0.998047        99755       512.00
//!        3.415     0.998242        99775       568.89
//!        3.455     0.998437        99794       640.00
//!        3.499     0.998633        99815       731.43
//!        3.533     0.998828        99833       853.33
//!        3.587     0.999023        99853      1024.00
//!        3.617     0.999121        99863      1137.78
//!        3.645     0.999219        99873      1280.00
//!        3.711     0.999316        99882      1462.86
//!        3.781     0.999414        99892      1706.67
//!        3.835     0.999512        99902      2048.00
//!        3.875     0.999561        99907      2275.56
//!        3.915     0.999609        99911      2560.00
//!        3.933     0.999658        99916      2925.71
//!        3.963     0.999707        99921      3413.33
//!        4.025     0.999756        99926      4096.00
//!        4.107     0.999780        99929      4551.11
//!        4.115     0.999805        99931      5120.00
//!        4.143     0.999829        99933      5851.43
//!        4.191     0.999854        99936      6826.67
//!        4.247     0.999878        99938      8192.00
//!        4.311     0.999890        99940      9102.22
//!        5.243     0.999902        99941     10240.00
//!        5.371     0.999915        99942     11702.86
//!        5.499     0.999927        99943     13653.33
//!        5.551     0.999939        99944     16384.00
//!        5.639     0.999945        99945     18204.44
//!        5.855     0.999951        99946     20480.00
//!        5.855     0.999957        99946     23405.71
//!        5.935     0.999963        99947     27306.67
//!        5.935     0.999969        99947     32768.00
//!        6.107     0.999973        99948     36408.89
//!        6.107     0.999976        99948     40960.00
//!        6.107     0.999979        99948     46811.43
//!        6.227     0.999982        99949     54613.33
//!        6.227     0.999985        99949     65536.00
//!        6.227     0.999986        99949     72817.78
//!        6.227     0.999988        99949     81920.00
//!        6.227     0.999989        99949     93622.86
//!        6.263     0.999991        99950    109226.67
//!        6.263     1.000000        99950          inf
//! #[Mean    =        1.428, StdDeviation   =        0.589]
//! #[Max     =        6.260, Total count    =        99950]
//! #[Buckets =           27, SubBuckets     =         2048]
//! ----------------------------------------------------------
//!   119982 requests in 1.00m, 22.30GB read
//! Requests/sec:   1999.67
//! Transfer/sec:    380.54MB
//! ```

mod config;
mod error;
mod fetch;
mod format;
mod handlers;
mod io;
mod latency;
pub mod server;
mod statics;
mod utils;
