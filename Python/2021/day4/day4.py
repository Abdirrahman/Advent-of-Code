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


def setup_bingo() -> list[list]:
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


def mark_board(num, board) -> list[list]:
    """Marks board where bingo number occurs on the board"""

    new_board = [["X" if i == num else i for i in line] for line in board]

    return new_board


def check_board(board) -> bool:
    """Checks if board is marked"""

    # Check vertically
    for j in range(5):
        for i in board:
            if i[j] == "X":
                return True

    # Check horizontally
    for i in board:
        for j in i:
            if j == "X":
                return True


def check_win(picked_numbers, board) -> bool:
    """Checks if board meets the win condition"""

    # Check vertically
    for j in range(5):
        score = 0
        for i in board:
            if i[j] in picked_numbers:
                score += 1
            if score == 5:
                return True

    # Check horizontally
    for i in board:
        score = 0
        for j in i:
            if j in picked_numbers:
                score += 1
            if score == 5:
                return True


def calculate_sum(board, picked_numbers) -> int:
    """Sum of unpicked numbers from the board"""

    unpicked_numbers = []

    for line in board:
        for num in line:
            if num not in picked_numbers:
                unpicked_numbers.append(num)

    return sum(unpicked_numbers)


def game():
    """Simulate bingo game"""

    random_numbers, boards = setup_bingo()
    picked_numbers = []
    winning_boards = []
    for num in random_numbers:
        for board in boards:
            new_board = mark_board(num, board)
            check = check_board(new_board)
            if check is True:
                picked_numbers.append(num)
                win = check_win(picked_numbers, board)
                if win is True:
                    final_score = calculate_sum(board, picked_numbers)
                    if board not in winning_boards:
                        #  Part 2
                        winning_boards.append(board)
                        last = calculate_sum(board, picked_numbers) * num

    return winning_boards, last


if __name__ == "__main__":
    console.print(game())
