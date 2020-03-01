# Setup Grafana with k8s

## Quickstart

```
mkdir -p /tmp/k8s/grafana/;
kubectl apply -f ./grafana.yaml;
kubectl get service --watch;
```

Wait and get external IP of grafana-service.

This example, I demo with minikube + service type=NodePort.

It create /tmp/k8s/grafana/ for storage. So please remove when you don't want it anymore

If you run minikube on VM, try to use following command to mount host directory to VM

```
sudo chown 472:472 /tmp/k8s/grafana/
sudo chmod 777 /tmp/k8s/grafana/
minikube mount --gid=472 --uid=472 /tmp/k8s/grafana:/tmp/k8s/grafana
```

If you run minikube on VM, check external IP/port through following command:

```
minikube service list
```
