# Generate 100k records and group each 10k

Inspired from https://avi.im/blag/2021/fast-sqlite-inserts/#io-time

I tried to implement a record generator that generate 100k and groups each 10k. The test doesn't include database inserting.

It's not fair to compare my result with them, without inconsistent running machine and purpose. Just for fun.

run `make`

```
goos: darwin
goarch: amd64
pkg: github.com/ledongthuc/notes/go/generate-100k-fields
cpu: Intel(R) Core(TM) i7-4770HQ CPU @ 2.20GHz
BenchmarkChannel-8             1        24091777075 ns/op
BenchmarkNative-8              1        29403914453 ns/op
BenchmarkThreaded-8            1        58827834681 ns/op
```
