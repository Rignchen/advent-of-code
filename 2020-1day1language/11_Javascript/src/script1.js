// Read file given by user
const fs = require('node:fs');
if (process.argv.length == 2) {
	console.error("Usage: node script <file>");
	exit(1);
}
let content = fs.readFileSync(process.argv[2], 'utf8').split('\n');

/**
 * Return all adjacent entry to a given entry
 * @param {number} x
 * @param {number} y
 * @param {any[]} content
 * @returns {any[]}
 */
function adjacent(x, y, content) {
	let adj = [];
	for (let i = x - 1; i <= x + 1; i++) {
		for (let j = y - 1; j <= y + 1; j++) {
			if (i < 0 || j < 0 || i >= content.length || j >= content[i].length) continue;
			if (i == x && j == y) continue;
			adj.push(content[i][j]);
		}
	}
	return adj;
}

let old;
while (content !== old) {
	old = content;
}

console.log(content);
