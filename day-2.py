
def grade(levels):
    difference = [levels[i+1] - levels[i] for i in range(len(levels) - 1)]

    valid = all([abs(d) <= 3 and abs(d) >= 1 for d in difference])

    if not valid:
        return False

    increasing = all([d > 0 for d in difference])
    decreasing = all([d < 0 for d in difference])

    if not increasing and not decreasing:
        return False

    return True

def go():
    safe = 0
    with open('day-2.data', 'r') as f:
        while (report := f.readline()) and report:
            levels = [int(level.strip()) for level in report.split(' ')]

            if grade(levels):
                safe += 1
                continue

            for i in range(len(levels)):
                newLevels = levels.copy()
                newLevels.pop(i)

                if grade(newLevels):
                    safe += 1
                    break

    return safe


print('total', go())
