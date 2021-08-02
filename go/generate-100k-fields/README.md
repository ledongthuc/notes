Generate 100k fields and grouping them per 10k

run `make`

```
goos: darwin
goarch: amd64
pkg: github.com/ledongthuc/notes/go/generate-100k-fields
cpu: Intel(R) Core(TM) i7-4770HQ CPU @ 2.20GHz
BenchmarkNative-8              1        30631749853 ns/op
BenchmarkThreaded-8            1        23456511512 ns/op
```
