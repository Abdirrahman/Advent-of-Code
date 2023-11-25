import math
def read_input() -> list[str]:
    with open("input.txt", "r") as f:
        return f.readlines()



def find_position():
    '''Find position of submarine.'''
    lines = read_input()
    depth = 0
    horizontal = 0
    aim = 0
    for line in lines:
        if "forward" in line:
            format = line.split()
            num = format[1]
            horizontal = horizontal + int(num)
            depth = depth + (int(num) * aim)
        
        if "down" in line:
            format = line.split()
            num = format[1]
            aim = aim + int(num)

        if "up" in line:
            format = line.split()
            num = format[1]
            aim = aim - int(num)

    pos = horizontal * depth

    return pos