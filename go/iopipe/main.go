package main

import (
	"fmt"
	"io"
	"sync"
	"time"
)

func main() {
	r, w := io.Pipe()

	wg := sync.WaitGroup{}
	wg.Add(2)

	go func() {
		b, err := io.ReadAll(r)
		if err != nil {
			panic(err)
		}
		fmt.Println("DEBUG: ", string(b))
		r.Close()
		wg.Done()
	}()

	go func() {
		for i := 0; i < 5; i++ {
			fmt.Fprintf(w, "%d ", i)
			time.Sleep(1 * time.Second)
		}
		w.Close()
		wg.Done()
	}()

	wg.Wait()
}
