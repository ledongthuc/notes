# Liveness probe

We use a testing service `ledongthuc/liveness-probe-api` to simulate the case readiness isn't ready to serve request of service.
 - 1st and 2th requests to `/liveness_probe_status`, they return status code 500, requests to service will be failed
 - 3rd and 4nd request, `/liveness_probe_status` returns status code 200, requests to service will be success

1. Apply deployment and service

```
kubectl apply  -f./
```

2. Watch pod status

```
$ kubectl get pod --watch

http-app-xxx   0/1     Running             0          5s
http-app-xxx   1/1     Running             4          104s
```

4. Check application logs:

```
$ kubectl logs http-app-xxx

Env NUMBER_OF_LIVENESS_SUCCESS:  2
Env NUMBER_OF_SKIP_STARTUP:
Start with port 8080
/readiness_probe_status, Counter:  0 , Status: 500
/readiness_probe_status, Counter:  1 , Status: 500
/readiness_probe_status, Counter:  2 , Status: 200
/readiness_probe_status, Counter:  3 , Status: 200
