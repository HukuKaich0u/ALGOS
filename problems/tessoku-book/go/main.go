package main

import (
	"fmt"
	"sync"
)

func recv(c <-chan string, wg *sync.WaitGroup) {
	defer wg.Done()
	for {
		s, ok := <-c
		if !ok {
			fmt.Println("* channel closed")
			break
		}
		fmt.Println("[R]", s)
	}
}

func send(c chan<- string, wg *sync.WaitGroup) {
	defer wg.Done()
	fmt.Println("[S] Sending...")
	for i := 0; i < 5; i++ {
		c <- fmt.Sprintf("Hello %d", i)
	}
	fmt.Println("[S] Done")
	close(c)
}

func main() {
	var wg sync.WaitGroup
	wg.Add(2)
	c := make(chan string)
	go recv(c, &wg)
	go send(c, &wg)
	wg.Wait()
}
