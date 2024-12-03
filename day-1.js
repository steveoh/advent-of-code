import { readFileSync } from 'node:fs';

const input = readFileSync('./day-1.data', 'utf-8');
const lines = input.split('\n');
const list1 = [];
const frequency = {};
let score = 0;

for (const line of lines) {
  if (!line) {
    continue;
  }

  let [one, two] = line.split(/\s+/);
  one = parseInt(one.trim());
  two = parseInt(two.trim());

  frequency[two] = frequency[two] ? frequency[two] + 1 : 1;

  list1.push(one);
}

for (let i = 0; i < lines.length; i++) {
  const value = list1[i];
  const occurances = frequency[value] ?? 0;

  score += occurances * value;
}

console.log(score);
