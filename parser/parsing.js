function tokenize(input) {
  return input
    .split('"')
    .map(function(x, i) {
      if (i % 2 === 0) {
        // not in string
        return x.replace(/\(/g, " ( ").replace(/\)/g, " ) ");
      } else {
        // in string
        return x.replace(/ /g, "!whitespace!");
      }
    })
    .join('"')
    .trim()
    .split(/\s+/)
    .map(function(x) {
      return x.replace(/!whitespace!/g, " ");
    });
}

function parenthesize(input, tree) {
  if (tree === undefined) {
    return parenthesize(input, []);
  }

  let token = input.shift();
  if (token === undefined) {
    return tree.pop();
  } else if (token === "(") {
    tree.push(parenthesize(input, []));
    return parenthesize(input, tree);
  } else if (token === ")") {
    return tree;
  } else {
    return parenthesize(input, tree.concat(categorize(token)));
  }
}

function categorize(input) {
  if (!isNaN(parseFloat(input))) {
    return { type: "number", value: parseFloat(input) };
  } else if (input[0] === '"' && input.slice(-1) === '"') {
    return { type: "string", value: input.slice(1, -1) };
  } else {
    return { type: "identifier", value: input };
  }
}

module.exports = function parse(input) {
  return parenthesize(tokenize(input));
};
