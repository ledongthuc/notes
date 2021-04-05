# Build image

1. Build & Push image

```
docker build -t 270431303334.dkr.ecr.ap-southeast-2.amazonaws.com/hello-world:latest .;
aws ecr get-login-password --region ap-southeast-2 | docker login --username AWS --password-stdin 270431303334.dkr.ecr.ap-southeast-2.amazonaws.com;
docker push 270431303334.dkr.ecr.ap-southeast-2.amazonaws.com/hello-world:latest;
```

2. Run terraform

```
terraform init;
terraform apply;
```

3. Test

```
curl --location --request POST 'https://ssah8op3a5.execute-api.ap-southeast-2.amazonaws.com/default/FakeChimera' --header 'Content-Type: application/json' --data-raw '{}'
```
