// Read file given by user
const fs = require('node:fs');
if (process.argv.length == 2) {
	console.error("Usage: node script <file>");
	exit(1);
}
let content = fs.readFileSync(process.argv[2], 'utf8').split('\n');

console.log(content);
