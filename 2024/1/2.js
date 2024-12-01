main();

import * as fs from 'fs';


function main() {
	let list1 = [];
	let freqMap = new Map();
	const datafile = fs.readFileSync('data.txt').toString();
	const lines = datafile.split('\n');
	for (const line of lines) {
		const cmp = line.split('   ');
		list1.push(parseInt(cmp[0]));
		const intVal = parseInt(cmp[1]);
		freqMap.set(intVal, freqMap.get(intVal) + 1 || 1)
	}

	list1.sort();
	let total = 0;

	for (let i = 0; i < list1.length; i++) {
		if (!list1[i]) continue
	//console.log(list1[i], freqMap.get(list1[i]) || 0)
		total += list1[i] * (freqMap.get(list1[i]) || 0)
	}
	console.log(total);
}
