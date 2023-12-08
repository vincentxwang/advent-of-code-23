MAX_DICE = {'blue': 14, 'green': 13, 'red': 12}

answer = 0

# Note that we need to cd into day-2/part-2 for this to work!
with open("day-2/part-1/input.txt", 'r', encoding="utf-8") as f:
    while True:
        line = f.readline().split(":")
        # Apparently readline() returns '' when the next line is... empty?
        if line == ['']:
            break
        id = int(line[0].split(" ")[1])
        games = line[1].split(";")
        valid = True
        for game in games:
            game = game.strip()
            colors = game.split(", ")
            for color in colors:
                temp = color.split()
                if int(temp[0]) > MAX_DICE[temp[1]]:
                    valid = False
        if valid == True:
            answer += id
            
print(answer)
    