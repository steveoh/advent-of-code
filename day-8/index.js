import { readFileSync } from 'node:fs';

const input = readFileSync('./day-8.data', 'utf-8');
const lines = input.split('\n').filter((line) => line.length > 0);
const width = lines[0].length;
const height = lines.length;
let frequencies = {};

for (let i = 0; i < lines.length; i++) {
  for (let j = 0; j < lines[i].length; j++) {
    const item = lines[i][j];

    // if (lines[i][j] === 'a') {
    if (lines[i][j] !== '.' && lines[i][j] !== '#') {
      if (!frequencies[item]) {
        frequencies[item] = [];
      }

      frequencies[item].push([i, j]);
    }
  }
}

const antinodes = new Set([]);
for (let key of Object.keys(frequencies)) {
  const frequency = frequencies[key];
  for (let i = 0; i < frequency.length; i++) {
    for (let j = i + 1; j < frequency.length; j++) {
      const [x1, y1] = frequency[i];
      const [x2, y2] = frequency[j];
      const [dx, dy] = [x2 - x1, y2 - y1];

      let iteration = 1;
      const opposites = [[x1, y1], [x2, y2]];
      while (x1 - dx * iteration >= 0 && x1 - dx * iteration < height && y1 - dy * iteration >= 0 && y1 - dy * iteration < width) {
        opposites.push([x1 - dx * iteration, y1 - dy * iteration]);
        iteration++;
      }

      iteration = 1;
      while (x2 + dx * iteration >= 0 && x2 + dx * iteration < height && y2 + dy * iteration >= 0 && y2 + dy * iteration < width) {
        opposites.push([x2 + dx * iteration, y2 + dy * iteration]);
        iteration++;
      }

      for (let antinode of opposites) {
        const [vx, vy] = antinode;
        console.log('hit', lines[vx][vy]);
        antinodes.add(`[${vx},${vy}]`);
      }
    }
  }
}

// 1006 is too low
console.log('total', antinodes.size);
