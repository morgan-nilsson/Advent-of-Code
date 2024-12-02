import * as fs from 'fs';

const file = fs.readFileSync('data.dat').toString()

const reports = file.split('\n').map((elem) => {
	return elem.split(' ')
})

let total = 0
for (let i = 0; i < reports.length; i++) {
	let safe = isSafe(reports[i])
	total += safe
	//console.log(reports[i], 'safe: ', safe)
}
console.log(total)

function isSafe(report) {
	if (!report[0]) return 0
	let sign = report[1] - report[0] > 0 ? 1 : -1
	//console.log(report, 'sign: ', sign)

	for (let i = 1; i < report.length; i++) {
		const diff = report[i] - report[i-1];
		if (diff == 0) return 0
		if ((diff > 0 ? 1 : -1) != sign) return 0
		if (Math.abs(diff) > 3) return 0
	}
	return 1;
}
