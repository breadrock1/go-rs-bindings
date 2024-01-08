package main

// #cgo CFLAGS: -g -Wall
// #cgo LDFLAGS: -L../target/debug -lrslib
// #include <stdlib.h>
// #include "../rslib.h"
import "C"
import "fmt"

func add(left, right int) int {
	result := C.addC(C.int(left), C.int(right))
	return int(result)
}

func del(left, right int) int {
	result := C.delC(C.int(left), C.int(right))
	return int(result)
}

func main() {
	sumRes := add(2, 2)
	delRes := del(4, 2)
	fmt.Printf("Sum: %d\nDel: %d\n", sumRes, delRes)
}
