package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {

	for {
		evaluate()
	}
}

func evaluate() {
	fmt.Printf("lisp> ")
	reader := bufio.NewReader(os.Stdin)
	text, _ := reader.ReadString('\n')
	fmt.Printf(text)
}
