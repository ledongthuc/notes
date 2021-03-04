Without trimpath:
```go run main.go

panic: DEBUG

goroutine 1 [running]:
main.main()
        /Users/thucle/Projects/ledongthuc/notes/go/trimpath/main.go:4 +0x39
exit status 2
```

with trimpath:
```
go/trimpath [master●] » go run -trimpath main.go
panic: DEBUG

goroutine 1 [running]:
main.main()
        command-line-arguments/main.go:4 +0x39
```
