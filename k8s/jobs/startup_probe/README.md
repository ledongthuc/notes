# Liveness probe

We use a testing service `ledongthuc/liveness-probe-api` to simulate the case liveness route's down or return error status.
 - 1st -> 5th requests to `/liveness_probe_status`, they return status code 200
 - 6th and 7th request, `/liveness_probe_status` returns status code 500, it makes the pod's restarted
 - Request counter will be reset after restarting pod
 - After 5 times restarting, we will got error CrashLoopBackOff

1. Apply deployment and service

```
kubectl apply  -f./
```

2. Watch pod status

```
$ kubectl get pod --watch

http-app-xxx   0/1     ContainerCreating   0          5s
http-app-xxx   1/1     Running             0          5s
http-app-xxx   1/1     Running             1          30s
http-app-xxx   1/1     Running             2          54s
http-app-xxx   1/1     Running             3          80s
http-app-xxx   1/1     Running             4          104s
http-app-xxx   0/1     CrashLoopBackOff    4          2m7s
```

3. To check detail, use describe

```
$ kubectl describe pod http-app-xxx
... 
Events:
  Type     Reason     Age               From               Message
  ----     ------     ----              ----               -------
  Normal   Scheduled  <unknown>         default-scheduler  Successfully assigned default/http-app-xxx to minikube
  Normal   Pulling    4s (x3 over 82s)  kubelet, minikube  Pulling image "ledongthuc/liveness-probe-api:latest"
  Warning  Unhealthy  4s (x4 over 49s)  kubelet, minikube  Liveness probe failed: HTTP probe failed with statuscode: 500
  Normal   Killing    4s (x2 over 44s)  kubelet, minikube  Container http-server failed liveness probe, will be restarted
  Normal   Pulled     2s (x3 over 79s)  kubelet, minikube  Successfully pulled image "ledongthuc/liveness-probe-api:latest"
  Normal   Created    2s (x3 over 79s)  kubelet, minikube  Created container http-server
  Normal   Started    2s (x3 over 79s)  kubelet, minikube  Started container http-server
```

4. Check application logs:

```
$ kubectl logs http-app-xxx
NUMBER_OF_SUCCESS:  5
NUMBER_OF_READINESS_FAIL:  5
NUMBER_OF_STARTUP_FAIL:  5
Start with port 8080
/liveness_probe_status, Counter:  0 , Status: 200
/liveness_probe_status, Counter:  1 , Status: 200
/liveness_probe_status, Counter:  2 , Status: 200
/liveness_probe_status, Counter:  3 , Status: 200
/liveness_probe_status, Counter:  4 , Status: 200
/liveness_probe_status, Counter:  5 , Status: 500
/liveness_probe_status, Counter:  6 , Status: 500

