const fs = require('fs');
const escodegen = require('escodegen');

const stdin = fs.readFileSync('/dev/stdin', 'utf8');
const astObject = JSON.parse(stdin);

const jsSource = escodegen.generate(astObject);
console.log(jsSource);
