

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


# Integrate as web component

[documentation](https://github.com/gradio-app/gradio/blob/f346118133866a4186b46ce9d3c7e3aab844577a/ui/packages/app/src/main.ts)

```html
<!DOCTYPE html>
<html lang="en">

<head>
  <script type="module" src="https://gradio.s3-us-west-2.amazonaws.com/3.4/gradio.js">
  </script>
</head>

<body>
  <gradio-app src="https://vk3lxhamsu45sdy6ne3kkyitiy0scbdm.lambda-url.eu-west-1.on.aws/"></gradio-app>
</body>

</html>
```