1. Generate auth file

```
htpasswd -c .htpasswd user1
htpasswd .htpasswd user2
```

2. Verify auth file

```
cat .htpasswd
```

3. Start app service and nginx

```
docker-compose up
```

4. Access URL

```
http://localhost:8080/
```

Enter username/password
