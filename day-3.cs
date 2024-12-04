using System.Text.RegularExpressions;

// var sample = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
var line = string.Concat(File.ReadAllLines("day-3.data"));

var regex = new Regex(@"mul\(((?<one>\d{1,3})),((?<two>\d{1,3})\))");

void AddToTotal(Match match, ref int total)
{
  var first = int.TryParse(match.Groups["one"].ValueSpan, out int one);
  var second = int.TryParse(match.Groups["two"].ValueSpan, out int two);

  if (!first || !second)
  {
    Console.WriteLine("Could not parse" + match);

    return;
  }

  total += one * two;
}

int PartOne()
{
  var total = 0;
  var matches = regex.Matches(line);

  foreach (Match match in matches)
  {
    AddToTotal(match, ref total);
  }

  return total;
}

int PartTwo()
{
  var allRegex = new Regex(@"mul\((?<one>\d+),(?<two>\d+)\)|do\(\)|don't\(\)");
  var total = 0;

  var matches = allRegex.Matches(line);
  var enabled = true;
  foreach (Match match in matches)
  {
    if (match.Value == "don't()")
    {
      enabled = false;
      continue;
    }

    if (match.Value == "do()")
    {
      enabled = true;
      continue;
    }

    if (!enabled || match.Value.StartsWith('d'))
    {
      continue;
    }

    AddToTotal(match, ref total);
  }

  return total;
}
var answer = PartOne();
var answer2 = PartTwo();

Console.WriteLine($"One: {answer} Two: {answer2}");
