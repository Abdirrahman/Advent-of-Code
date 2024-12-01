def read_input() -> list[str]:
    with open("input.txt", "r") as f:
        return f.read().split()


def split(list):
    left = []
    right = []
    for i, n in enumerate(list):
        if i % 2 == 0:
            right.append(int(n))
        else:
            left.append(int(n))
    return sorted(left), sorted(right)


def calculate_differences(tups):
    left = tups[0]
    right = tups[1]
    differences = []

    for i, n in enumerate(left):
        difference = left[i] - right[i]
        differences.append(abs(difference))
    return sum(differences)


def calculate_similarities(tups):
    left = tups[0]
    right = tups[1]

    similarities = []

    for i, n in enumerate(left):
        count = 0
        for j, v in enumerate(right):
            if n == v:
                count = count + 1
        similarities.append(int(n) * count)
    return sum(similarities)


if __name__ == "__main__":

    x = read_input()
    # print(x)
    # print(split(x))
    y = split(x)
    print(calculate_similarities(y))
