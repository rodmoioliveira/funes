# Docker

You can use funes within a docker image:

```sh
# Build Imange
docker build -t funes-docker -f Dockerfile . --no-cache=true

# Run inside docker
docker run --rm \
  -p 8080:8080 \
  --name funes-docker funes-docker

# Make a request
curl http://localhost:8080/pokeapi.co/api/v2/pokemon/1
```
