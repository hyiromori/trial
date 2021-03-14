const fs = require('fs');
const acorn = require('acorn');
// https://github.com/estree/estree
const espurify = require('espurify');

const stdin = fs.readFileSync('/dev/stdin', 'utf8');
const acornAst = acorn.parse(stdin);

const standardAst = espurify(acornAst);
console.log(JSON.stringify(standardAst, null, 2));
