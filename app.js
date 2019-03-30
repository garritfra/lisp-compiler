const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
  prompt: "lisp> "
});

rl.prompt();

rl.on("line", line => {
  evaluate(line.trim());
  rl.prompt();
});

function evaluate(input) {
  console.log(input);
}
