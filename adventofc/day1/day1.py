
def read_input() -> list[str]:
    with open("input.txt", "r") as f:
        return f.read().splitlines()
    
def sum_calories():
    '''Sum calories belonging to each elf and print largest calories.'''
    lines = read_input()
    sum = 0
    sum_of_calories = []

    for line in lines:

        if line == '':
            sum_of_calories.append(sum)
            sum = 0
        else:
            sum += int(line)
    sum_of_calories.sort()
    print(sum_of_calories[-1]+ sum_of_calories[-2] + sum_of_calories[-3])




if __name__=='__main__':
    sum_calories()