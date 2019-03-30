package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {

	for {
		evaluate(read())
	}
}

func read() string {
	fmt.Printf("lisp> ")
	reader := bufio.NewReader(os.Stdin)
	text, _ := reader.ReadString('\n')
	return text
}

func evaluate(input string) {

	fmt.Printf(input)
}
