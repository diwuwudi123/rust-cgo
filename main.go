package main

/*
#cgo CFLAGS: -I${SRCDIR}/build/include
#cgo linux LDFLAGS: -L${SRCDIR}/build/linux
#cgo windows LDFLAGS: -L${SRCDIR}/build/windows
#cgo linux LDFLAGS: -ltest
#cgo windows LDFLAGS: -ltest
#include "test.h"
*/
import "C"

func main() {
	C.hello_world()

}
