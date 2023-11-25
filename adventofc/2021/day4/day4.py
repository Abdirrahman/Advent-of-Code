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
# If rows is marked. - check the whole list has X
# If columns is marked. - all indexs at that pos are Xs.  i.e [][i].

def read_input() -> list[str]:
    with open("input.txt", "r") as f:
        return f.read().splitlines()
    

def day4():

    lines = read_input()
    bingo_nums = lines[0]
    # console.print(lines)
    return len(lines)


def store_boards():

    lines = read_input()
    boards = []
    single_board = []
    
    for i in range(2,len(lines)):

        if lines[i] == '':
            boards.append(single_board)
            single_board = []
        else:
            single_board.append([int(num) for num in lines[i].split()])
        # console.print(boards)

    # console.print(boards)
    return boards

def mark_board(board, bingo_num):
    '''Marks a single board where bingo number is at.'''
    if check_bingo(board):
        print("Already at 5 Xs")
    board = [["X" if num == bingo_num else num for num in line ] for line in board ]
    # print(board)
    # print(board)

    return board
       
def calculate_score(board):
    '''Calculates score of remaining numbers on the board.'''

    score = 0
    for line in board:
        for i in range(len(line)):
            if line[i] == 'X'  or line[i] == ' ':
                score += 0
            else:
                score += int(line[i])
    return score 

def check_bingo(marked_board):
    '''Checks if a board has met bingo conditions.'''

    # Check horizontally
    for line in marked_board:
        count = 0
        # print("before",marked_board)
        for i in range(len(line)):
            if line[i] == "X":
                count += 1
        if count == 5:
            return True
        else:
            count = 0
    # Check vertically
    count_vert = 0
    for i in range(len(marked_board[0])):
        for line in marked_board:
            if line[i] == "X":
                count_vert += 1
        if  count_vert == 5:
            return True
        else:
            count_vert = 0
    return False


def simulate_bingo_game():
    '''Run Bingo game'''
    lines = read_input()
    bingo_nums = lines[0].split(',')
    boards = store_boards()
    count = 0
    completed = []
    for board in boards:
        for i in bingo_nums:
            m = mark_board(board,int(i))
            board = m
            # print(board)
            if check_bingo(board) is True:
                count += 1
                completed.append({"b": count, "num": i})
                score = calculate_score(board)
                # print(score, i)
                break
    first_num = None
    winning_board = None
    for num in bingo_nums:
        for c in completed:
            if str(num) == c["num"]:
                if first_num is None:
                    first_num = num
                    winning_board = c["b"]
                elif num < first_num:
                    first_num = num
                    winning_board = c["b"]

    print(first_num, winning_board) 

if __name__=="__main__":
    # boards = store_boards()
    # board = boards[0]
    # mark_board(board, 4)
    simulate_bingo_game()
    # lines = read_input()
    # bingo_nums = lines[0].split(',')
    # board = store_boards()
    # # for i in range(len(bingo_nums)):
    # #         board = mark_board(board[-1],bingo_nums[i])
    # #         print(board)
    # #         # check_bingo(board)


    # # print(board)
    # nums = [90,67,2,3,4]
    # for i in nums:
    #     m = mark_board(board[-1], i)
    #     board[-1] = m
    #     print(m)

    # boards = store_boards()
    # print(boards[79])