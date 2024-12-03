import * as fs from 'fs';

const file = fs.readFileSync('data.dat').toString().trim().split('\n').join('')
//const file = fs.readFileSync('smallData.dat').toString().trim().split('\n').join('')

let total = 0;

const results = file.matchAll(/mul\((\d+),(\d+)\)/g);

// basic caching
let last = 1
let lastIndex = 0

for (const match of results) {
	let found = 0
	for (let i = match.index; i >= lastIndex; i--) {
		if (file.substring(i, i + 4) == 'do()') {
			total += parseInt(match[1]) * parseInt(match[2])
			found = 1
			last = 1
			lastIndex = i
			break
		}
		if (file.substring(i, i + 7) == "don't()") {
			found = 1
			last = 0
			lastIndex = i
			break
		}
	}
	if (!found && last == 1) {
		total += parseInt(match[1]) * parseInt(match[2])
	}

}

console.log(total)
