import numpy as np


def part1():
    with open('../src/input/day3.txt') as f:
        data = np.matrix([
            list(map(int, line.strip()))
            for line in f
        ])

    ones_count = np.array(np.matmul(data.transpose(), data).diagonal())[0]
    gamma_rate = int(''.join([
        '1' if i / data.shape[0] > 0.5 else '0'
        for i in ones_count
    ]), 2)
    epsilon_rate = ~gamma_rate & ((1 << data.shape[1]) - 1)
    print("Gamma rate:", gamma_rate)
    print("Epsilon rate:", epsilon_rate)
    print("Power consumption:", gamma_rate * epsilon_rate)


if __name__ == '__main__':
    part1()
