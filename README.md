# Simple Lisp Compiler

This Compiler is implemented in Rust.

## CLI Roadmap

Accessing a REPL
```bash
root@linux:~$ lipsy
lipsy> (+ 2 5)
7
lipsy>
```

List all available Commands
```bash
root@linux:~$ lipsy --help
```

Evaluating a statement
```bash
root@linux:~$ lipsy -e "(+ 1 2)"
3
root@linux:~$ lipsy --evaluate "(* 2 5)"
25
root@linux:~$
```

Interpreting a file
```bash
root@linux:~$ lipsy addTwoAndSeven.lips
9
root@linux:~$
```