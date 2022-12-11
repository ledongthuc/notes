.SUFFIXES: .peg .go

.peg.go:
	peg -switch -inline -strict -output $@ $<

all: grammar.go
