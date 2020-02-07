# Service with ingress nginx

1. Install ingress nginx controller

[Install ingress](/k8s/setup/minikube/install_ingress.md)

2. Verify installation

```
kubectl get pods --all-namespaces -l app.kubernetes.io/name=ingress-nginx
```

3. Apply k8s configuration

note: assume you are at root of repository

```
kubectl apply -f ./
```

4. Check exposed ip of ingress

```
kubectl get ingress http-app-ingress
```

Output example:

```
Output: 

NAME               HOSTS   ADDRESS          PORTS   AGE
http-app-ingress   *       192.168.99.101   80      8m18s
```

```
   Internet IP
        |
   [ Ingress ]
   --|-----|--
   [ Services ]
```

5. Try try try

```
curl -v 192.168.99.101:80/sub-path/
```

Output:
```
*   Trying 192.168.99.101...
* TCP_NODELAY set
* Connected to 192.168.99.101 (192.168.99.101) port 80 (#0)
> GET /sub-path/ HTTP/1.1
> Host: 192.168.99.101
> User-Agent: curl/7.64.1
> Accept: */*
>
< HTTP/1.1 200 OK
< Server: openresty/1.15.8.2
< Date: Thu, 23 Jan 2020 20:01:08 GMT
< Content-Type: text/plain; charset=utf-8
< Content-Length: 12
< Connection: keep-alive
< X-App-Name: http-echo
< X-App-Version: 0.2.3
<
Hello world
* Connection #0 to host 192.168.99.101 left intact
* Closing connection 0
```
