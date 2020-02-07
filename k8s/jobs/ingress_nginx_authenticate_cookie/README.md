# Service with ingress nginx and authentication

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

notes: will take litle time to assign IP address

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

5. Login success

```
curl -c ./cookie.txt -v http://192.168.99.101:80/login?username=username&password=password
```

Output:
```
*   Trying 192.168.99.101...
* TCP_NODELAY set
* Connected to 192.168.99.101 (192.168.99.101) port 80 (#0)
> GET /login?username=username&password=password HTTP/1.1
> Host: 192.168.99.101
> User-Agent: curl/7.64.1
> Accept: */*
>
< HTTP/1.1 200 OK
< Server: openresty/1.15.8.2
< Date: Fri, 07 Feb 2020 22:16:12 GMT
< Content-Type: text/plain; charset=utf-8
< Content-Length: 13
< Connection: keep-alive
* Added cookie AUTHENTICATED="true" for domain 192.168.99.101, path /, expire 0
< Set-Cookie: AUTHENTICATED=true
<
* Connection #0 to host 192.168.99.101 left intact
Login success* Closing connection 0
```

6. Acess forbidden resource (with cookie)

```
curl -v http://192.168.99.101:80/docs/
```

Output:

```
*   Trying 192.168.99.101...
* TCP_NODELAY set
* Connected to 192.168.99.101 (192.168.99.101) port 80 (#0)
> GET /docs/ HTTP/1.1
> Host: 192.168.99.101
> User-Agent: curl/7.64.1
> Accept: */*
>
< HTTP/1.1 302 Moved Temporarily
< Server: openresty/1.15.8.2
< Date: Fri, 07 Feb 2020 22:18:28 GMT
< Content-Type: text/html
< Content-Length: 151
< Connection: keep-alive
< Location: http://192.168.99.101/login?rd=http://192.168.99.101%2Fdocs%2F
<
<html>
<head><title>302 Found</title></head>
<body>
<center><h1>302 Found</h1></center>
<hr><center>openresty/1.15.8.2</center>
</body>
</html>
* Connection #0 to host 192.168.99.101 left intact
* Closing connection 0
```

7. Logout and clean cookie file

```
$ curl -c cookie.txt -v http://192.168.99.101:80/logout; rm cookie.txt
```

Output:

```
*   Trying 192.168.99.101...
* TCP_NODELAY set
* Connected to 192.168.99.101 (192.168.99.101) port 80 (#0)
> GET /logout HTTP/1.1
> Host: 192.168.99.101
> User-Agent: curl/7.64.1
> Accept: */*
>
< HTTP/1.1 200 OK
< Server: openresty/1.15.8.2
< Date: Fri, 07 Feb 2020 22:20:33 GMT
< Content-Type: text/plain; charset=utf-8
< Content-Length: 14
< Connection: keep-alive
* Added cookie AUTHENTICATED="false" for domain 192.168.99.101, path /, expire 0
< Set-Cookie: AUTHENTICATED=false
<
* Connection #0 to host 192.168.99.101 left intact
Logout success* Closing connection 0
```
