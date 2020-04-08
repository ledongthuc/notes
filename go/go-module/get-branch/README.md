# Use latest commit in branch of module

You can define the branch name when use `go get` to update version of library
It will get latest commit of branch and update to `go.sum`

Run command:
```
go get rsc.io/quote@master;
go mod tidy;
```

Check fields:

go.mod
```
module github.com/ledongthuc/notes/go/go-module/get-branch

go 1.14

require rsc.io/quote v1.5.3-0.20180710144737-5d9f230bcfba
```

go.sum
```
golang.org/x/text v0.0.0-20170915032832-14c0d48ead0c h1:qgOY6WgZOaTkIIMiVjBQcw93ERBE4m30iBm00nkL0i8=
golang.org/x/text v0.0.0-20170915032832-14c0d48ead0c/go.mod h1:NqM8EUOU14njkJ3fqMW+pc6Ldnwhi/IjpwHt7yyuwOQ=
rsc.io/quote v1.5.3-0.20180710144737-5d9f230bcfba h1:YPbK3ry9YRfDxnLRK3p/sSWjMthEyxN44AV/SQpLfYo=
rsc.io/quote v1.5.3-0.20180710144737-5d9f230bcfba/go.mod h1:7YuuA+XbqchTpjYHB4zQUyH3QJ6NfNQwBeWLrZ9BH2k=
rsc.io/quote/v3 v3.0.0 h1:OEIXClZHFMyx5FdatYfxxpNEvxTqHlu5PNdla+vSYGg=
rsc.io/quote/v3 v3.0.0/go.mod h1:yEA65RcK8LyAZtP9Kv3t0HmxON59tX3rD+tICJqUlj0=
rsc.io/sampler v1.3.0 h1:7uVkIFmeBqHfdjD+gZwtXXI+RODJ2Wc4O7MPEh/QiW4=
rsc.io/sampler v1.3.0/go.mod h1:T1hPZKmBbMNahiBKFy5HrXp6adAjACjK9JXDnKaTXpA=
```

