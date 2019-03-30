module.exports = [
  { type: "space", regex: /^\s/ },
  { type: "lParen", regex: /^\(/ },
  { type: "rParen", regex: /^\)/ },
  { type: "number", regex: /^[0-9\.]+/ },
  { type: "string", regex: /^".*?"/ },
  { type: "variable", regex: /^[^\s\(\)]+/ } // take from the beginning 1+ characters until you hit a ' ', '(', or ')' // TODO - support escaped double quote
];
