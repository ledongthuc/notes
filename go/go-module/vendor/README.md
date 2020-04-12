# Vendor

Go module support vendor folder update. It allows we stores the library source into vendor folder

Run command:

```
go mod vendor;
go mod tidy;
```

It will create/update the go.mod. And pull source code of version of libs that you require in go.sum in `./vendor`
