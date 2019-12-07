# HTTP request client

Support a client to test HTTP/HTTPS request. Supports:
 - HTTP
 - HTTPs request
 - Custom custom/unpopular certificate authority
 - Support skip verify certificate authority

Docker images at: https://hub.docker.com/r/ledongthuc/http-request-client
Source at: https://github.com/ledongthuc/snippet/tree/master/go/http-request-client

# Quick start

Start client with docker
```
docker run -e URL="http://postman-echo.com/get" ledongthuc/http-request-client
```
# Parameters

Name | Default | Description
------------ | ------------- | -------------
URL | (empty) |  it's required. It supports both http and https.
METHOD | GET | [https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods](http request method)
REQUEST_BODY | (empty) | Request body to send to server. Won't send anything if leaving it empty.
BASIC_AUTH_USERNAME | (empty) | Support basic auth if it's set.
BASIC_AUTH_PASSWORD | (empty) | Support basic auth if it's set.
CERT_PATH | (empty) | allows to add a certificate authority cert file. Frequently to use with [https://www.digitalocean.com/community/tutorials/how-to-create-a-self-signed-ssl-certificate-for-nginx-in-ubuntu-16-04](self-certificate ssl) or untrusted certificate authority.
SKIP_VERIFY | false | skip to verify certificate authority issuer with https request.

## Request https

```
docker run -e URL="https://postman-echo.com/get" ledongthuc/http-request-client
```

## Custom http request method

```
docker run \
-e URL="https://postman-echo.com/post" \
-e METHOD="POST" \
ledongthuc/http-request-client
```

## Request body data

```
docker run \
-e URL="https://postman-echo.com/post" \
-e METHOD="POST" \
-e REQUEST_BODY="{'test':'abc'}" \
ledongthuc/http-request-client
```

## Basic authentication

```
docker run \
-e URL="https://postman-echo.com/basic-auth" \
-e BASIC_AUTH_USERNAME="postman" \
-e BASIC_AUTH_PASSWORD="password" \
ledongthuc/http-request-client
```

## Send request to HTTPS with additonal certificate authority

You need to [the https://superuser.com/questions/97201/how-to-save-a-remote-server-ssl-certificate-locally-as-a-file](download SSL certificate locally as a file) first. In this example, I named it `untrusted_cert.pem`

```
docker run \
-v /path/to/self/untrusted_cert.pem:/untrusted_cert.pem \
-e URL="https://untrusted-root.badssl.com" \
-e CERT_PATH="/untrusted_cert.pem" \
ledongthuc/http-request-client
```

## Skip verify certificate authority

```
docker run \
-e URL="https://untrusted-root.badssl.com" \
-e SKIP_VERIFY="true" \
ledongthuc/http-request-client
```

# TODO

 - Support cookies
 - Support headers
 - Support form data
 - Support OAuth
 - Support configuration file

Pull requests are always welcome
