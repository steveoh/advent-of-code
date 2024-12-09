import { readFileSync } from 'node:fs';

const input = readFileSync('./day-7.data', 'utf-8');
const lines = input.split('\n').filter(Boolean);
const operators = ['+', '*', '||'];

const getPermutations = (size) => {
  if (size === 0) {
    return [];
  }
  if (size === 1) {
    return operators.map((op) => [op]);
  }

  const permutations = [];
  const smallerPermutations = getPermutations(size - 1);

  for (const smallerPermutation of smallerPermutations) {
    for (const operator of operators) {
      permutations.push([...smallerPermutation, operator]);
    }
  }

  return permutations;
};

let total = 0;
for (const line of lines) {
  let [result, rest] = line.split(':');
  result = Number(result.trim());
  const values = rest
    .trim()
    .split(' ')
    .map((value) => Number(value));
  const permutations = getPermutations(values.length - 1);

  for (let i = 0; i < permutations.length; i++) {
    const permutation = permutations[i];

    let currentResult = values[0];
    for (let j = 1; j < values.length; j++) {
      const operator = permutation.shift();
      if (operator === '+') {
        currentResult += values[j];
      } else if (operator === '*') {
        currentResult *= values[j];
      } else {
        currentResult = Number(currentResult.toString() + values[j].toString());
      }
    }

    if (currentResult === result) {
      total += currentResult;
      // console.log('match', currentResult, values, permutation);
      break;
    }
  }
}

// 27790187165600 to low
console.log(total == 11387, total);
