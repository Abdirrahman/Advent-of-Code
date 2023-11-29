from day4 import day4, store_boards, check_bingo, calculate_score


# def test_day4():

#     res = day4()

#     assert res == '0000'

# def test_store_boards():

#     res = store_boards()


#     assert res == []


def test_check_bingo():
    board = [
        [90, 67, 30, 22, 58],
        [35, 50, 63, 59, 19],
        ["X", "X", "X", "X", "X"],
        [43, 47, 56, 96, 20],
        [87, 57, 33, 37, 49],
    ]

    board2 = [
        [90, 67, 30, "X", 58],
        [35, 50, 63, "X", 19],
        [55, "X", "X", "X", "X"],
        [43, 47, 56, "X", 20],
        [87, 57, 33, "X", 49],
    ]

    # board3 = [[90, 67, 30, 22, 58], [35, 50, 63, 59, 19], [55, 'X', 'X', 'X', 'X'], [43, 47, 56, 96, 'X'], [87, 57, 33, 37, 49]]
    # board4 = [[31, 93, 'X', 11, 30], [2, 45, 40, 69, 33], [82, 21, 'X', 99, 86], [57, 'X', 34, 94, 85], ['X', 49, 'X', 14, 65]]

    res = check_bingo(board)
    res2 = check_bingo(board2)
    # res3 = check_bingo(board3)
    # res4 = check_bingo(board4)

    assert res is True
    assert res2 is True
    # assert res3 == 'bingo!'
    # assert res4 == 'Bingo'


def test_calculate_score():
    board = [
        [90, 67, 30, 22, 58],
        [35, 50, 63, 59, 19],
        ["X", "X", "X", "X", "X"],
        [43, 47, 56, 96, 20],
        [87, 57, 33, 37, 49],
    ]
    # board = [[56, 'X', 'X', 24, 'X'], [70, 'X', 'X', 'X', 9], [97, 'X', 'X', 82, 'X'], ['X', 12, 'X', 'X', 'X'], [14, 'X', 'X', 'X', 'X']]
    res = calculate_score(board)

    assert res == 1018
