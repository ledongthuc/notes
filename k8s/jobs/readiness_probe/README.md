# Liveness probe

We use a testing service `ledongthuc/liveness-probe-api` to simulate the case readiness isn't ready to serve request of service.
 - 1st -> 5th requests to `/readiness_probe_status`, they return status code 500, requests to service will be failed. So the pod's network isn't list to service
 - from 5th request, `/readiness_probe_status` returns status code 200, requests to service will be success and its network is attached, we can reach to this pod
 
Docker Image: https://hub.docker.com/r/ledongthuc/liveness-probe-api

1. Apply deployment and service

```
kubectl apply  -f./
```

2. Watch pod status

```
$ kubectl get pod --watch

http-app-xxx   0/1     Running   0          52s
http-app-xxx   1/1     Running   0          61s
```

4. Check application logs:
```
$ kubectl logs http-app-xxx

NUMBER_OF_SUCCESS:  5
NUMBER_OF_READINESS_FAIL:  5
NUMBER_OF_STARTUP_FAIL:  5
Start with port 8080
/readiness_probe_status, Counter:  0 , Status: 500
/readiness_probe_status, Counter:  1 , Status: 500
/readiness_probe_status, Counter:  2 , Status: 500
/readiness_probe_status, Counter:  3 , Status: 500
/readiness_probe_status, Counter:  4 , Status: 500
/readiness_probe_status, Counter:  5 , Status: 200
/readiness_probe_status, Counter:  6 , Status: 200
/readiness_probe_status, Counter:  7 , Status: 200
/readiness_probe_status, Counter:  8 , Status: 200

```

5. Describe pod
```
$ kubectl logs http-app-xxx

Events:
  Type     Reason     Age                 From               Message
  ----     ------     ----                ----               -------
  Normal   Scheduled  <unknown>           default-scheduler  Successfully assigned default/http-app-xxx to minikube
  Normal   Pulling    2m                  kubelet, minikube  Pulling image "ledongthuc/liveness-probe-api:latest"
  Normal   Pulled     119s                kubelet, minikube  Successfully pulled image "ledongthuc/liveness-probe-api:latest"
  Normal   Created    119s                kubelet, minikube  Created container http-server
  Normal   Started    119s                kubelet, minikube  Started container http-server
  Warning  Unhealthy  70s (x5 over 110s)  kubelet, minikube  Readiness probe failed: HTTP probe failed with statuscode: 500
```
