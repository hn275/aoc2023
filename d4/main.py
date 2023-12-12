copies: list[int] = list()
max_len = 0

with open("./example.txt", encoding="utf8", mode="r") as f:
    lines: list[str] = f.read().strip().split("\n")

    for i, _ in enumerate(lines):
        max_len = i

    for i, line in enumerate(lines):
        score = 0
        matching_count = 0
        l = line.split(":")[1]
        winnings = [int(i) for i in l.split("|")[0].split(" ") if i != ""]
        hands = [int(i) for i in l.split("|")[1].split(" ") if i != ""]
        for hand in hands:
            if hand in winnings:
                matching_count += 1
                if score == 0:
                    score = 1
                else:
                    score = score << 1

        copies.append(matching_count)


score = copies.copy()
print("copies_map:", copies)
print("max_len", max_len)
for i, v in enumerate(copies):
    print("i, v, j, copies")
    for j in range(i+1, i+1+v):
        if j > max_len:
            continue
        copies[j] += 1
        print(i, v, j, copies)
    print()

print(copies)
print(score)
