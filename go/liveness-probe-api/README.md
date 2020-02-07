# liveness-probe-api

API server supports routes for liveness, readiness, startup probes

Docker images at: https://hub.docker.com/r/ledongthuc/liveness-probe-api

Source at: https://github.com/ledongthuc/snippet/tree/master/go/liveness-probe-api

## Quickstart

Run server with docker
```
docker run -p 8080:8080 ledongthuc/liveness-probe-api
```

Make requests to service 10 times
```
for i in {1..10}; do curl -I http://localhost:8080/liveness_probe_status; done;
```

6 first requests are sucess, 4 next requests return 500 error

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
## APIs

`/liveness_probe_status`

Will return status 200 until counter > `NUMBER_OF_SUCCESS`. Otherwise, return status 500

`/liveness_probe_timeout`

Will return status 200 until counter > `NUMBER_OF_SUCCESS`. Otherwise, timeout

`/readiness_probe_status`

Will return status 200 until counter > `NUMBER_OF_READINESS_FAIL`. Otherwise, return status 200 

`/readiness_probe_timeout`

Will be timeout until counter > `NUMBER_OF_READINESS_FAIL`. Otherwise, return status 200 

`/startup_probe_status`

Will return status 200 until counter > `NUMBER_OF_STARTUP_FAIL`. Otherwise, return status 200 

`/startup_probe_timeout`

Will be timeout until counter > `NUMBER_OF_STARTUP_FAIL`. Otherwise, return status 200 

## Feel free to PR

