import * as fs from 'fs';

const reports = fs.readFileSync('data.dat').toString().split('\n').map((elem) => {return elem.split(' ').map((el) => {return parseInt(el, 10)})})

console.log(reports.map((report) => {return isSafeperm(report)}).reduce((accumalator, currentValue) => {return accumalator + currentValue}))

function isSafeperm(report) {
	if (isSafe(report)) return 1
	for (let i = 0; i < report.length; i++) if(isSafe(Array.from(report).splice(0, i).concat(Array.from(report).splice(i+1, report.length)))) return 1
	return 0
}

function isSafe(report) {
	if (!report[0]) return 0
	let sign = report[1] - report[0] > 0 ? 1 : -1

	for (let i = 1; i < report.length; i++) {
		const diff = report[i] - report[i-1];
		if (diff == 0) return 0
		if ((diff > 0 ? 1 : -1) != sign) return 0
		if (Math.abs(diff) > 3) return 0
	}
	return 1;
}
