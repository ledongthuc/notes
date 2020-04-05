# Convert govendor to go module

Go module support to convert from Govendor with init command:

## Convert
```
go mod init github.com/ledongthuc/notes/go/go-module/govendor2gomodule;
go mod tidy;
rm vendor;
```
Result:

go.mod
```
module github.com/ledongthuc/notes/go/go-module/govendor2gomodule

go 1.14

require (
	golang.org/x/text v0.3.3-0.20200306154105-06d492aade88 // indirect
	rsc.io/quote v1.5.3-0.20180710144737-5d9f230bcfba
	rsc.io/sampler v1.99.99 // indirect
)
```

## Try run

```
go run main.go
```

go.sum
```
golang.org/x/text v0.0.0-20170915032832-14c0d48ead0c/go.mod h1:NqM8EUOU14njkJ3fqMW+pc6Ldnwhi/IjpwHt7yyuwOQ=
golang.org/x/text v0.3.3-0.20200306154105-06d492aade88 h1:FrVQfHlSD67oSu7MiglkpXjapTEe2ZF/wS9JaYQX/xA=
golang.org/x/text v0.3.3-0.20200306154105-06d492aade88/go.mod h1:5Zoc/QRtKVWzQhOtBMvqHzDpF6irO9z98xDceosuGiQ=
golang.org/x/tools v0.0.0-20180917221912-90fa682c2a6e/go.mod h1:n7NCudcB/nEzxVGmLbDWY5pfWTLqBcC2KZ6jyYvM4mQ=
rsc.io/quote v1.5.3-0.20180710144737-5d9f230bcfba h1:YPbK3ry9YRfDxnLRK3p/sSWjMthEyxN44AV/SQpLfYo=
rsc.io/quote v1.5.3-0.20180710144737-5d9f230bcfba/go.mod h1:7YuuA+XbqchTpjYHB4zQUyH3QJ6NfNQwBeWLrZ9BH2k=
rsc.io/quote/v3 v3.0.0 h1:OEIXClZHFMyx5FdatYfxxpNEvxTqHlu5PNdla+vSYGg=
rsc.io/quote/v3 v3.0.0/go.mod h1:yEA65RcK8LyAZtP9Kv3t0HmxON59tX3rD+tICJqUlj0=
rsc.io/sampler v1.3.0/go.mod h1:T1hPZKmBbMNahiBKFy5HrXp6adAjACjK9JXDnKaTXpA=
rsc.io/sampler v1.99.99 h1:7i08f/p5TBU5joCPW3GjWG1ZFCmr28ybGqlXtelhEK8=
rsc.io/sampler v1.99.99/go.mod h1:T1hPZKmBbMNahiBKFy5HrXp6adAjACjK9JXDnKaTXpA=
```
