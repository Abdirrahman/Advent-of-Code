def read_input() -> list[str]:
    with open("input.txt", "r") as f:
        return f.readlines()

#  First read each line and store that number
# Get next line and compare that with first and then store that number
# when comparing if the number increased, then count increased by 1. 

# 0 1 2 3 4 5 6 
# def find_increases():
#  '''Part 1 '''

#     lines = read_input()
#     count = 0
#     i = -1
#     while i <= len(lines) - 2:
#         if lines[i] < lines[i+1]:
#             # if i == 0:
#             #     print(lines[i])
#             # print(lines[i+1])
#             # print(len(lines))
#             count += 1
#             # print(f" count loop:{count}")
#         i +=1

#     print(f" final count: {count}")
#     return count
def find_increases():
    '''Part 2'''
    lines = read_input()
    count = 0
    i = -1
    while i <= len(lines) - 4:
        sum_a = int(lines[i]) + int(lines[i+1]) + int(lines[i+2])
        sum_b = int(lines[i+1]) + int(lines[i+2]) + int(lines[i+3])
        print(f"suma: {sum_a} and sumb: {sum_b}")
        if sum_a < sum_b:
            # print(f" sum {lines[i] + lines[i+1] + lines[i+2]}")
            # if i == 0:
            #     print(lines[i])
            # print(lines[i+1])
            # print(len(lines))
            count += 1
            # print(f" count loop:{count}")
        i +=1

    # print(f" final count: {count}")
    return count