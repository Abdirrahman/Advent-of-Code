def read_input() -> list[str]:
    with open("input.txt", "r") as f:
        return f.read().splitlines()


"""Part 1"""

# Gamma rate - most common bit at each index.
# Epsilon rate - the least common bit - do this with if else.
# power = G*E


def find_rates():
    lines = read_input()

    gamma_rate = ""
    epsilon_rate = ""
    # print(lines)
    for i in range(0, len(lines[0])):
        zeros = 0
        ones = 0

        for binary in lines:
            if int(binary[i]) == 0:
                zeros += 1
            else:
                ones += 1
        if ones < zeros:
            gamma_rate += "0"
            epsilon_rate += "1"
        else:
            gamma_rate += "1"
            epsilon_rate += "0"
    print(ones)
    product = int(gamma_rate, 2) * int(epsilon_rate, 2)
    return product


"""Part 2"""

# 100101111010
# check pos[i] for all nums
# add to counters
# o2 crit:  if 1 > 0 or 1 == 0 then 1 else 0
# co2 crit: if 0 > 1 or 1 == 0 then 0 else 1
# delete nums in pos[i] based on crit.


def find_o2_rates():
    lines = read_input()
    for i in range(0, len(lines[0])):
        zeros = 0
        ones = 0
        for binary in lines:
            if int(binary[i]) == 0:
                zeros += 1
            else:
                ones += 1

        lines = [num for num in lines if num[i] == ("1" if ones >= zeros else "0")]
        if len(lines) == 1:
            o2 = int("".join(lines), 2)
            return o2


def find_co2_rates():
    lines = read_input()
    for i in range(0, len(lines[0])):
        zeros = 0
        ones = 0
        for binary in lines:
            if int(binary[i]) == 0:
                zeros += 1
            else:
                ones += 1

        lines = [num for num in lines if num[i] == ("0" if ones >= zeros else "1")]
        if len(lines) == 1:
            co2 = int("".join(lines), 2)
            return co2


def find_rates():
    o2 = find_o2_rates()
    co2 = find_co2_rates()

    return o2 * co2
