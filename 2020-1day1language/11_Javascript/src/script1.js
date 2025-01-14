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

/**
 * fill chairs based on a rule set:
 * - If empty (L) & no occupied seats near: becomes occupied (#)
 * - If occupied (#) & 4+ occupied seats near: becomes empty (L)
 * - Otherwise, the seat's state does not change.
 * @param {string[]} old
 * @returns {string[]}
 */
function peopleMove(old) {
	let content = [];
	old.forEach((line, i) => {
		let newLine = '';
		line.split('').forEach((c, j) => {
			const adj = adjacent(i, j, old);
			const filledNear = adj.filter(x => x == '#').length;

			if (c == 'L' && filledNear == 0) {
				newLine += '#';
			} else if (c == '#' && filledNear >= 4) {
				newLine += 'L';
			} else {
				newLine += c;
			}
		});
		content.push(newLine);
	});
	return content;
}

let old = [];
while (content.join('') !== old.join('')) {
	old = content;
	content = peopleMove(content);
}

console.log(content.join('\n'));
console.log(content.join('').split('').filter(x => x === '#').length);
