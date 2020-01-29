# Service with ingress nginx

1. Install ingress nginx controller

```
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/master/deploy/static/mandatory.yaml
```

https://kubernetes.github.io/ingress-nginx/deploy/

2. Verify installation

```
kubectl get pods --all-namespaces -l app.kubernetes.io/name=ingress-nginx
```


