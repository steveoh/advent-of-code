using System.Diagnostics;

static char[][] CreateMatrixFrom(string file)
{
  var data = File.ReadAllLines(file);

  var matrix = new char[data.Length][];
  for (int i = 0; i < data.Length; i++)
  {
    matrix[i] = data[i].ToCharArray();
  }

  return matrix;
}

var matrix = CreateMatrixFrom("day-4.data");
var directions = new List<(int, int)>()
{
  (0, 1),   // →
  (0, -1),  // ←
  (1, 0),   // ↓
  (-1, 0),  // ↑
  (-1, -1), // ↖
  (-1, 1),  // ↗
  (1, -1),  // ↙
  (1, 1)    // ↘
};
var patternDirections = new List<(int, int, char[])>() {
  (-1, 1, ['S', 'M']),  // ↗
  (-1, -1, ['M', 'S']), // ↖
  (1, -1, ['M', 'S']),  // ↙
  (1, 1, ['S', 'M'])    // ↘
};
var word = "XMAS";

int FindWordIn(char[][] matrix, string word)
{
  var total = 0;
  for (int i = 0; i < matrix.Length; i++)
  {
    for (int j = 0; j < matrix[i].Length; j++)
    {
      if (matrix[i][j] == word[0])
      {
        // check 8 directions for an M
        foreach (var direction in directions)
        {
          var x = 1;
          var (dx, dy) = direction;
          var (ni, nj) = (i + dx, j + dy);

          foreach (var letter in word.Skip(1))
          {
            if (ni >= 0 && ni < matrix.Length && nj >= 0 && nj < matrix[ni].Length && matrix[ni][nj] == letter)
            {
              x++;
              ni += dx;
              nj += dy;

              continue;
            }

            break;
          }

          if (x == word.Length)
          {
            total++;
          }
        }
      }
    }
  }

  return total;
}

int FindPatternIn(char[][] matrix)
{
  var total = 0;
  var center = 'A';
  var empty = '\0';

  for (int i = 0; i < matrix.Length; i++)
  {
    for (int j = 0; j < matrix[i].Length; j++)
    {
      if (matrix[i][j] == center)
      {
        // check 4 directions for pattern
        var startChar = new char[2] { empty, empty };
        for (int k = 0; k < patternDirections.Count; k++)
        {
          (int, int, char[]) direction = patternDirections[k];
          var (dx, dy, letters) = direction;
          var (ni, nj) = (i + dx, j + dy);

          if (ni < 0 || ni >= matrix.Length || nj < 0 || nj >= matrix[ni].Length || !letters.Contains(matrix[ni][nj]))
          {
            break;
          }

          var index = k % 2;
          if (startChar[index] == empty)
          {
            startChar[index] = matrix[ni][nj];
          }
          else if (startChar[index] == matrix[ni][nj])
          {
            break;
          }

          if (k == patternDirections.Count - 1)
          {
            total++;
          }
        }
      }
    }
  }

  return total;
}

Stopwatch sw = new();
sw.Start();
var mem = GC.GetTotalAllocatedBytes();
sw.Restart();

Console.WriteLine($"\nPart 1 answer: {FindWordIn(matrix, word)}");
Console.WriteLine($"Memory: {(GC.GetTotalAllocatedBytes() - mem) / 1024.0 / 1024:N2}mb");
Console.WriteLine($"Elapsed: {sw.Elapsed.TotalMilliseconds}ms");

mem = GC.GetTotalAllocatedBytes();
sw.Restart();

Console.WriteLine($"\nPart 1 answer: {FindPatternIn(matrix)}");
Console.WriteLine($"Memory: {(GC.GetTotalAllocatedBytes() - mem) / 1024.0 / 1024:N2}mb");
Console.WriteLine($"Elapsed: {sw.Elapsed.TotalMilliseconds}ms");
