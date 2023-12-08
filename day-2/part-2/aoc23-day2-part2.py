MAX_DICE = {'blue': 14, 'green': 13, 'red': 12}

answer = 0

# Note that we need to cd into day-2/part-2 for this to work!
with open("day-2/part-2/input.txt", 'r', encoding="utf-8") as f:
    while True:
        line = f.readline().split(":")
        # Apparently readline() returns '' when the next line is... empty?
        if line == ['']:
            break
        games = line[1].split(";")
        valid = True
        min_dice = {'blue': 0, 'green': 0, 'red': 0}
        for game in games:
            game = game.strip()
            colors = game.split(", ")
            for entry in colors:
                temp = entry.split()
                number = int(temp[0])
                color = temp[1]
                min_dice[color] = max(min_dice[color], number)
        power = 1
        for num in min_dice.values():
            power *= num
        answer += power

print(answer)
    