# Minimal

Example of minimal setup:

```sh
# Build server
cargo build --release

# Run server
target/release/minimal

# Make a request
curl http://localhost:8080/pokeapi.co/api/v2/pokemon/1
```
