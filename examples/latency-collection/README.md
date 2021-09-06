# Latency Collection

You can provide funes with a latency collection and a regex to simulate more
realistic test scenarios. You will need a `latency_collection.json` file and a
`regex` that should match each of the keys in the file:

#### latency_collection.json

```json
{
  "jsonplaceholder.typicode.com/posts": {
    "min": 0.00317,
    "p50": 0.0833,
    "p75": 0.14,
    "p90": 0.199,
    "p95": 0.217,
    "p99": 0.217,
    "max": 3.75
  },
  "jsonplaceholder.typicode.com/comments": {
    "min": 0.00317,
    "p50": 0.0833,
    "p75": 0.14,
    "p90": 0.199,
    "p95": 0.217,
    "p99": 0.217,
    "max": 3.75
  },
  "jsonplaceholder.typicode.com/albums": {
    "min": 0.00317,
    "p50": 0.0833,
    "p75": 0.14,
    "p90": 0.199,
    "p95": 0.217,
    "p99": 0.217,
    "max": 3.75
  },
  "jsonplaceholder.typicode.com/todos": {
    "min": 0.00317,
    "p50": 0.0833,
    "p75": 0.14,
    "p90": 0.199,
    "p95": 0.217,
    "p99": 0.217,
    "max": 3.75
  },
  "jsonplaceholder.typicode.com/users": {
    "min": 0.00317,
    "p50": 0.0833,
    "p75": 0.14,
    "p90": 0.199,
    "p95": 0.217,
    "p99": 0.217,
    "max": 3.75
  }
}
```

The schema for this file is:

```txt
{
  "regex_match_key": {
    "min": float_number,
    "p50": float_number,
    "p75": float_number,
    "p90": float_number,
    "p95": float_number,
    "p99": float_number,
    "max": float_number
  },
  "regex_match_key": {...},
  ...
}
```

#### regex for matching keys

```sh
jsonplaceholder\.typicode\.com/\w+
```

#### Running with latency simulation

```sh
FUNES_LATENCY_COLLECTION="latency_collection.json" \
FUNES_API_REGEX='jsonplaceholder\.typicode\.com/\w+' \
funes

# Test mocked api with latency
wrk -t2 -c10 -d60s --timeout 70s http://localhost:8080/jsonplaceholder.typicode.com/todos/1
Running 1m test @ http://localhost:8080/jsonplaceholder.typicode.com/todos/1
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   413.07ms  829.91ms   3.76s    88.18%
    Req/Sec    41.07     19.18   115.00     67.27%
  4908 requests in 1.00m, 0.99MB read
Requests/sec:     81.71
Transfer/sec:     16.92KB
```
