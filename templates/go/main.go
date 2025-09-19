package main

import (
	"fmt"
	"math/rand"
)

func add(x int, y int) int {
	return x + y
}

func main() {
	fmt.Println("My favorite number is", rand.Intn(10))

	fmt.Println(add(42, 13))
}
