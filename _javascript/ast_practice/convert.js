const fs = require('fs');
const estraverse = require('estraverse');

const stdin = fs.readFileSync('/dev/stdin', 'utf8');
const ast = JSON.parse(stdin);

estraverse.traverse(ast, {
  enter: (currentNode, parentNode) => {
    // ここに変換処理を入れる
  },
  leave: (currentNode, parentNode) => {
    // ここに変換処理を入れる
  },
});

console.log(JSON.stringify(ast, null, 2));
