# HTTP hello world

 - Support hello-world message from HTTP request
 - Support custom route and message base on environment variable

Docker images at: https://hub.docker.com/r/ledongthuc/hello-api
Source at: https://github.com/ledongthuc/snippet/tree/master/go/hello-api

# Quick start

Start client with docker
```
docker run -p 8080:8080 ledongthuc/hello-api:latest env CUSTOM_ROUTES="/custom_url:this is custom;/random.8b3ec8ff2ac96774ff4e6965e9bba307.html:this is random" ./app
```

Runs:

```
$ curl localhost:8080
Hello, World!
```

```
curl localhost:8080/custom_url
this is random
```

```
curl localhost:8080/random.8b3ec8ff2ac96774ff4e6965e9bba307.html
this is random
```
