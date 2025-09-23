package main

import "fmt"

func main() {
	var a [10]int
	for i := 0; i < 10; i++ {
		a[i] = i + 1
		fmt.Println(a[i])
	}
}
