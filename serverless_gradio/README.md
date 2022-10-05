

# Test Local build

```bash
docker build -t test-local -f Dockerfile.lambda .
```

run

```
docker run -ti -p 8080:8080 test-local
```

## Deploy with CDK

export aws profile
```
export AWS_PROFILE=hf-sm
```

Boostrap project in the cloud

```bash
cdk bootstrap
```

build & deploy rust function

```bash
cdk deploy 
```

## Delete CDK

```bash
cdk destroy
```