# liveness-probe-api
API server supports routes for liveness, readness, startup probes

Docker images at: https://hub.docker.com/repository/docker/ledongthuc/liveness-probe-api
Source at: https://github.com/ledongthuc/snippet/tree/master/go/liveness-probe-api

## Quickstart

Run server with docker
```
docker run -p 8080:8080 ledongthuc/liveness-probe-api
```

Make requests to service 6 times
```
curl -I http://localhost:8081/liveness_probe_status && \
curl -I http://localhost:8080/liveness_probe_status && \
curl -I http://localhost:8080/liveness_probe_status && \
curl -I http://localhost:8080/liveness_probe_status && \
curl -I http://localhost:8080/liveness_probe_status && \
curl -I http://localhost:8080/liveness_probe_status
```
## Notes

If you want change the number of first successful requests, use environment `NO_SUCCESS`

```
docker run -e NO_SUCCESS=10 -p 8080:8080 ledongthuc/liveness-probe-api
```

Want to run/build with Golang, check Makefile

```
make build
make run
```

Feel free to PR
