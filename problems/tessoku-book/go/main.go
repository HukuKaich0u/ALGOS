package main

import (
	"fmt"
	"sync"
)

func recv(ch <-chan string, wg *sync.WaitGroup) {
	defer wg.Done()
	s := <-ch
	fmt.Println("[R]", s)
}

func send(ch chan<- string, wg *sync.WaitGroup) {
	defer wg.Done()
	ch <- "Hello"
	ch <- "World"
	ch <- "Bye"
	close(ch)
	fmt.Println("[S] Done")
}

func main() {
	var wg sync.WaitGroup
	wg.Add(2)

	ch := make(chan string, 3)
	go recv(ch, &wg)
	go send(ch, &wg)
	wg.Wait()

	for s := range ch {
		fmt.Println("[Z]", s)
	}
}
