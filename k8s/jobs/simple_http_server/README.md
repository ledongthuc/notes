# Simple http server

WORKDIR: current folder
service_name: http-app-service

1. Apply deployment and service

```
kubectl apply ./
```

2. Get URL

Check how to get URL of service `http-app-service`: [Get k8s service URL](/k8s/setup/get_k8s_url.md)
Example output:

```
|----------------------|---------------------------|-----------------------------|-----|
|      NAMESPACE       |           NAME            |         TARGET PORT         | URL |
|----------------------|---------------------------|-----------------------------|-----|
| default              | http-app-service          | http://192.168.99.101:31055 |
|----------------------|---------------------------|-----------------------------|-----|

```

3. Test url

Request:
```
curl http://192.168.99.101:31055
```

Response:
```
Hello world
```
