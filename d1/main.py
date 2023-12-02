spelled_patterns = [
    "zero", "one", "two", "three", "four", "five",
    "six", "seven", "eight", "nine",
]
num_patterns = "0123456789"

class Value:
    index: int
    is_num: bool
    def __init__(self, i: int, b: bool) -> None:
        self.index = i
        self.is_num = b

    def parse(self, line: str) -> int:
        if self.is_num:
            return int(line[self.index])

        else:
            substr = line[self.index:self.index+5]
            for i, s in enumerate(spelled_patterns):
                if s.find(substr) != -1:
                    return i
            return 0

    def __str__(self) -> str:
        return f"[index: {self.index}, is_num: {self.is_num}]"


def hot_spots(line: str, first: bool) -> Value:
    buf = []

    # spell pattern
    for p in spelled_patterns:
        i: int = -1
        if first and i == -1:
            i = p.find(line)
        else:
            i = p.rfind(line)

        if i != -1:
            buf.append(Value(i, False))
        

    # num pattern
    n: int = -1
    for i, s in enumerate(line):
        if num_patterns.find(s) == -1:
            continue

        if first and n == -1:
            n = i
            break
        else:
            n = i


    if n != -1:
        buf.append(Value(n, True))

    buf.sort()

    if first:
        return buf[0]
    else:
        return buf[-1]



def parseLine(s: str) -> int:
    f = hot_spots(s, first=True)
    l = hot_spots(s, first=False)
    print(f, l)
    return 0


result: int = 0
with open("./input.txt") as f:
    for line in f.readlines():
        print("Reading line:", line)
        parsed = parseLine(line)
        print("Parsed:", parsed)
        result = result + parsed


print("Result:", result)
