package main

/*
#cgo CFLAGS: -I.
#cgo LDFLAGS: -L. -lmylib
*/
// #include <stdio.h>
// #include <errno.h>
// #include <stdlib.h>
// #include "mylib.h"
import "C"
import (
	"fmt"
	"unsafe"
)

func main() {
	name := C.CString("World")
	defer C.free(unsafe.Pointer(name))

	result := C.SayHello(name)
	fmt.Println(C.GoString(result))
}
