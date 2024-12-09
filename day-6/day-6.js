import { readFileSync } from 'node:fs';

let total = 1;
const input = readFileSync('./day-6.data', 'utf-8');
const lines = input.split('\n');

var directionIndex = 0;
var directions = [
  [-1, 0],  // ↑
  [0, 1],   // →
   [1, 0],   // ↓
  [0, -1],  // ←
];

const createBoard = (lines) => {
  const board = [];
  let position = [];

  for (let i = 0; i < lines.length; i++) {
    for (let j = 0; j < lines[i].length; j++) {
      if (!board[i]) {
        board[i] = [];
      }

      if (lines[i][j] === '.') {
        board[i][j] = '.';
      } else if (lines[i][j] === '#') {
        board[i][j] = '#';
      } else if (lines[i][j] === '^') {
        board[i][j] = 'X';
        position = [i, j];
      }
    }
  }

  return { board, position };
}

let { board, position } = createBoard(lines);

const length = board.length - 1;
const width = board[0].length - 1;

const stillInArea = (position) => {
  return position[0] >= 0 && position[0] <= length && position[1] >= 0 && position[1] <= width;
}

while (stillInArea(position)) {
  const [dx, dy] = directions[directionIndex % 4];
  const [x, y] = position;
  const newPosition = [x + dx, y + dy];

  if (!stillInArea(newPosition)) {
    break;
  }

  if (board[newPosition[0]][newPosition[1]] === '.') {
    board[newPosition[0]][newPosition[1]] = 'X';
    total++;
    position = newPosition;
  } else if (board[newPosition[0]][newPosition[1]] === 'X') {
    position = newPosition;
  } else {
    directionIndex = (directionIndex + 1) % 4;
  }
}

console.log(total);
