package main

import (
	"bufio"
	"fmt"
	"os"

	"github.com/garritfra/lisp-compiler/parser"
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
	parser := parser.NewParser()

	fmt.Println(&parser)
}
