import * as fs from 'fs';

const lines = fs.readFileSync('data.dat').toString().split('\n')

let xmas = 0

console.log(lines)

for (let i = 0; i < lines.length; i++) {
	let init = xmas
	for (let j = 0; j < lines[i].length; j++) {
		//if (lines[i][j] != 'X' && lines[i][j] != 'S') continue
		if (lines[i].substring(j, j + 4) == 'XMAS') xmas++;
		if (lines[i].substring(j, j + 4) == 'SAMX') xmas++;
		let word = ""
		for (let x = 0; x < 4; x++) {
			if(lines[i + x]) word += lines[i + x][j + x]
			else break
		}
		if (word == "XMAS") xmas++
		if (word == "SAMX") xmas++
		word = ""
		for (let x = 0; x < 4; x++) {
			if(lines[i - x]) word += lines[i - x][j + x]
			else break
		}
		if (word == "XMAS") xmas++
		if (word == "SAMX") xmas++
		word = "";
		for (let x = 0; x < 4; x++) {
			if(lines[i - x]) word += lines[i - x][j]
			else break
		}
		if (word == "XMAS") xmas++
		if (word == "SAMX") xmas++
		if (init != xmas) console.log(i, j, xmas - init)
		init = xmas
	}
}

console.log(xmas)
