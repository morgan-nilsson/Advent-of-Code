import * as fs from 'fs';

const lines = fs.readFileSync('data.dat').toString().split('\n')
//const lines = fs.readFileSync('smallData.txt').toString().split('\n')

let xmas = 0

console.log(lines)

for (let i = 0; i < lines.length; i++) {
	let init = xmas
	for (let j = 0; j < lines[i].length; j++) {
		if (lines[i][j] != 'A') continue
		let left = false
		let right = false

		if (i == lines.length) continue
		if (i == 0) continue
		if (j == lines[i].length) continue
		if (j == 0) continue
		const leftSide = "" + (lines[i+1][j-1] || '0') + 'A' + (lines[i-1][j+1] || '0')
		if (leftSide == "MAS" || leftSide == "SAM") left = true

		const rightSide = "" + (lines[i+1][j+1] || '0') + 'A' + (lines[i-1][j-1] || '0')
		if (rightSide == "MAS" || rightSide == "SAM") right = true

		if (right && left) xmas++

		if (init != xmas) console.log(i, j, xmas - init)
		init = xmas
	}
}

console.log(xmas)
