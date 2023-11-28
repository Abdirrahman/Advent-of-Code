from rich.console import Console

#  5x5 board of nums
#  nums marked along and down count.

console = Console()

# STEPS:
# Read the first line and store into a list.

# Read the boards and store into separate lists for each board.
# [
# [14 21 17 24  4]
# [10 16 15  9 19]
# [18  8 23 26 20]
# [22 11 13  6  5]
# [2  0 12  3  7]
# ]

# Mark:
# replace the number with X.

# Checks:
# On the next loops check:
# If rows is marked. - check the whole list has X
# If columns is marked. - all indexes at that pos are Xs.  i.e [][i].


def read_input() -> list[str]:
    with open("input.txt", "r") as f:
        return f.read().splitlines()


def setup_bingo():
    """Set up bingo game"""

    lines = read_input()
    random_numbers = []
    board = []
    boards = []
    for i, line in enumerate(lines):
        if i == 0:
            random_numbers = [int(num) for num in lines[i].split(",")]
            continue

        if line == "":
            if len(board) != 0:
                boards.append(board)
            board = []
            continue
        else:
            board.append([int(num) for num in lines[i].split()])

    return random_numbers, boards


if __name__ == "__main__":
    # console.print(game())
    console.print(setup_bingo())
