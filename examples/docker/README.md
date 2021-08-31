# Docker

You can use funes inside a docker image:

```sh
# Build Imange
docker build -t funes-docker -f Dockerfile . --no-cache=true

# Run inside docker
docker run --rm \
	--env RUST_APP=funes \
	--env RUST_HOST=0.0.0.0:8080 \
	--env RUST_LOG=funes,actix_web=info \
	--env RUST_ALLOW_EXTERNALS=true \
	-p 8080:8080 \
	--name funes-docker funes-docker

# Make a request
curl http://localhost:8080/pokeapi.co/api/v2/pokemon/1
```
