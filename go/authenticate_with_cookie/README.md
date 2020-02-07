# authenticate_with_cookie

This is simple api that contains 2 APIs. It's used to test login and authenticate with cookie

## Quickstart

Run server with docker
```
docker run -p 8080:8080 ledongthuc/authenticate_with_cookie
```

## Login fail and access forbid resource

Login:

```
$ curl -c ./cookie.txt -v http://localhost:8080/login?username=wrong&password=incorrect

*   Trying ::1...
* TCP_NODELAY set
* Connected to localhost (::1) port 8080 (#0)
> GET /login?username=wrong&password=incorrect HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/7.64.1
> Accept: */*
>
< HTTP/1.1 401 Unauthorized
< Date: Fri, 07 Feb 2020 21:40:44 GMT
< Content-Length: 10
< Content-Type: text/plain; charset=utf-8
<
* Connection #0 to host localhost left intact
Login fail* Closing connection 0
```

Access forbid resource

```
$ curl -b ./cookie.txt -v http://localhost:8080/authenticate

*   Trying ::1...
* TCP_NODELAY set
* Connected to localhost (::1) port 8080 (#0)
> GET /authenticate HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/7.64.1
> Accept: */*
>
< HTTP/1.1 401 Unauthorized
< Date: Fri, 07 Feb 2020 21:45:44 GMT
< Content-Length: 30
< Content-Type: text/plain; charset=utf-8
<
* Connection #0 to host localhost left intact
http: named cookie not present* Closing connection 0

```

Logout:

```
$ curl -c cookie.txt -v http://localhost:8080/logout; rm cookie.txt

*   Trying ::1...
* TCP_NODELAY set
* Connected to localhost (::1) port 8080 (#0)
> GET /authenticate HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/7.64.1
> Accept: */*
>
< HTTP/1.1 401 Unauthorized
< Date: Fri, 07 Feb 2020 21:45:44 GMT
< Content-Length: 30
< Content-Type: text/plain; charset=utf-8
<
* Connection #0 to host localhost left intact
http: named cookie not present* Closing connection 0

```

## Login success and access forbid resource

Login:

```
$ curl -c ./cookie.txt -v http://localhost:8080/login?username=username&password=password

*   Trying ::1...
* TCP_NODELAY set
* Connected to localhost (::1) port 8080 (#0)
> GET /login?username=username&password=password HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/7.64.1
> Accept: */*
>
< HTTP/1.1 200 OK
* Added cookie AUTHENTICATED="true" for domain localhost, path /, expire 0
< Set-Cookie: AUTHENTICATED=true
< Date: Fri, 07 Feb 2020 21:47:54 GMT
< Content-Length: 13
< Content-Type: text/plain; charset=utf-8
<
* Connection #0 to host localhost left intact
Login success* Closing connection 0
```

Access forbid resource:

```
$ curl -b ./cookie.txt -v http://localhost:8080/authenticate

*   Trying ::1...
* TCP_NODELAY set
* Connected to localhost (::1) port 8080 (#0)
> GET /authenticate HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/7.64.1
> Accept: */*
> Cookie: AUTHENTICATED=true
>
< HTTP/1.1 200 OK
< Date: Fri, 07 Feb 2020 21:48:36 GMT
< Content-Length: 13
< Content-Type: text/plain; charset=utf-8
<
* Connection #0 to host localhost left intact
Authenticated* Closing connection 0

```

Logout:

```
$ curl -c cookie.txt -v http://localhost:8080/logout; rm cookie.txt

*   Trying ::1...
* TCP_NODELAY set
* Connected to localhost (::1) port 8080 (#0)
> GET /authenticate HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/7.64.1
> Accept: */*
>
< HTTP/1.1 401 Unauthorized
< Date: Fri, 07 Feb 2020 21:45:44 GMT
< Content-Length: 30
< Content-Type: text/plain; charset=utf-8
<
* Connection #0 to host localhost left intact
http: named cookie not present* Closing connection 0

```
