# Simple http server

WORKDIR: current folder
service_name: http-app-service

1. Apply deployment and service

```
kubectl apply ./
```

2. Get URL

Check exposed URL `http-app-service`

check again install notes of K8s: [Get k8s service URL](/k8s/setup/)

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
