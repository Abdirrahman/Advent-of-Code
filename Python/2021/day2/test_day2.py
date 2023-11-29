from day2 import find_position


def test_find_position():
    """Test to see if correct position returned"""

    res = find_position()

    assert res == 1000
