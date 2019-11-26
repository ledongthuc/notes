# Simple http server

1. Apply deployment and service

```
kubectl apply ./
```

2. Expose the deployment to outside

It's based on K8S environment. If we use minikube, please use:

```
minikube service http-app-service
```

3. Browse the cluster with exposed service IP

```
Hello world
```
