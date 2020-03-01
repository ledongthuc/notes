# Setup Grafana with docker

## Quickstart

```
docker run -p 3000:3000 grafana/grafana
```

http://localhost:3000
Default username: admin
Default password: admin


## Start grafana with persistent storage

/var/lib/grafana
```
docker run \
	-p 3000:3000 \
	-v $(pwd)/nopush/storage/var/lib/grafana:/var/lib/grafana \
	grafana/grafana
```
