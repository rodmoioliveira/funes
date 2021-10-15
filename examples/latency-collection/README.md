# Latency Collection

You can provide a latency_collection to `funes` for more realistic test
simulations. Just create a `latency_collection.json` like so:

```json
{
  "regex": "jsonplaceholder\\.typicode\\.com/\\w+",
  "latencies": {
    "jsonplaceholder.typicode.com/posts": {
      "min": 13,
      "p50": 199,
      "p75": 248,
      "p90": 343,
      "p95": 591,
      "p99": 2384,
      "max": 3000
    },
    "jsonplaceholder.typicode.com/comments": {
      "min": 13,
      "p50": 199,
      "p75": 248,
      "p90": 343,
      "p95": 591,
      "p99": 2384,
      "max": 3000
    },
    "jsonplaceholder.typicode.com/albums": {
      "min": 13,
      "p50": 199,
      "p75": 248,
      "p90": 343,
      "p95": 591,
      "p99": 2384,
      "max": 3000
    },
    "jsonplaceholder.typicode.com/todos": {
      "min": 13,
      "p50": 199,
      "p75": 248,
      "p90": 343,
      "p95": 591,
      "p99": 2384,
      "max": 3000
    },
    "jsonplaceholder.typicode.com/users": {
      "min": 13,
      "p50": 199,
      "p75": 248,
      "p90": 343,
      "p95": 591,
      "p99": 2384,
      "max": 3000
    }
  }
}
```

The schema for this file should be:

```txt
{
  "regex": "regex_to_match_latencies_keys",
  "latencies": {
    "key_to_be_matched_by_regex": {
      "min": u64_milliseconds,
      "p50": u64_milliseconds,
      "p75": u64_milliseconds,
      "p90": u64_milliseconds,
      "p95": u64_milliseconds,
      "p99": u64_milliseconds,
      "max": u64_milliseconds
    },
    "regex_match_key": {...},
    ...
  }
}
```

Then run with:

```sh
FUNES_LATENCY_COLLECTION=latency_collection.json funes
```
