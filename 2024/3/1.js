import * as fs from 'fs';

const file = fs.readFileSync('data.dat').toString().trim().split('\n').join('')

let total = 0;

const results = file.matchAll(/mul\((\d+),(\d+)\)/g);

for (const match of results) {
	total += parseInt(match[1]) * parseInt(match[2])
}

console.log(total)
