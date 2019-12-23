import math

def main():
    with open('input.txt', 'r') as f:
        masses = [int(line.strip()) for line in f.readlines()]
    print(sum(math.floor(mass / 3) - 2 for mass in masses))

main()
