# Install Vault with k8s locally

## Prerequires

 - Docker
 - K8s (e.g. Minikube)
 - Helm

## 1. Install Consul

This example use consul as default datasource

```
helm repo add hashicorp https://helm.releases.hashicorp.com;
helm install consul hashicorp/consul --values helm-consul-values.yml;
kubectl get pods;
```

## 2. Install Vault

```
helm install vault hashicorp/vault --values helm-vault-values.yml
kubectl get pods
```

## 3. Initialize and unseal Vault

```
kubectl exec vault-0 -- vault operator init -key-shares=1 -key-threshold=1 -format=json > cluster-keys.json;
VAULT_UNSEAL_KEY=$(cat cluster-keys.json | jq -r ".unseal_keys_b64[]");
kubectl exec vault-0 -- vault operator unseal $VAULT_UNSEAL_KEY;
kubectl exec vault-1 -- vault operator unseal $VAULT_UNSEAL_KEY
kubectl exec vault-2 -- vault operator unseal $VAULT_UNSEAL_KEY
kubectl get pods
```

## 5. Add an example secret

```
cat cluster-keys.json | jq -r ".root_token";
kubectl exec -it vault-0 -- /bin/sh;
vault login;
vault secrets enable -path=secret kv-v2;
vault kv put secret/webapp/config username="static-user" password="static-password";
vault kv get secret/webapp/config;
```

## 4. Connect Vault to k8s

```
vault auth enable kubernetes;
vault write auth/kubernetes/config token_reviewer_jwt="$(cat /var/run/secrets/kubernetes.io/serviceaccount/token)" kubernetes_host="https://$KUBERNETES_PORT_444_TCP_ADDR:443" kubernetes_ca_cert=@/var/run/secrets/kubernetes.io/serviceaccount/ca.crt;
vault policy write webapp - <<EOF
path "secret/data/webapp/config" {
  capabilities = ["read"]
}
EOF;
vault write auth/kubernetes/role/webapp bound_service_account_names=vault bound_service_account_namespaces=default policies=webapp ttl=24h;
```

## 5. Deploy example k8s pod

```
kubectl apply --filename deployment-01-webapp.yml;
kubectl get pods;
kubectl port-forward $(kubectl get pod -l app=webapp -o jsonpath="{.items[0].metadata.name}") 8080:8080;
```
