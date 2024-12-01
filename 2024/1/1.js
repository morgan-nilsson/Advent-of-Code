main();

import * as fs from 'fs';


function main() {
	let list1 = [];
	let list2 = [];
	const datafile = fs.readFileSync('data.txt').toString();
	const lines = datafile.split('\n');
	for (const line of lines) {
		const cmp = line.split('   ');
		list1.push(parseInt(cmp[0]));
		list2.push(parseInt(cmp[1]));
	}

	list1.sort();
	list2.sort();
	let total = 0;

	for (let i = 0; i < list1.length; i++) {
		if (!list1[i]) continue
		total += Math.abs(list1[i] - list2[i]);
	}
	console.log(total);
}
