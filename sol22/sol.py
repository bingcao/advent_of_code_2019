import sys

def deal(card):
    return - card - 1

def cut(card, cut_i):
    return card - cut_i

def increment(card, inc):
    return card * inc

def run_command(line, card):
    (command, num) = line.rsplit(" ", 1)
    if num == 'stack':
        new_card = deal(card)
    else:
        num = int(num)
        if command == 'cut':
            new_card = cut(card, num)
        else:
            new_card = increment(card, num)
    return new_card

"""
Cut x + deal y: i -> (((i - x) % len) * y) % len = ((i - x) * y) % len = (i*y - x*y) % len
deal x + cut y: i -> (((i * x) % len) - y) % len
"""

# Testing:
# assert deal(3, 10) == 6
# assert cut(1, 10, 3) == 8
# assert cut(8, 10, 3) == 5
# assert [increment(i, 10, 3) for i in range(10)] == [0, 3, 6, 9, 2, 5, 8, 1, 4, 7]

if len(sys.argv) > 1:
    IS_TEST = True
else:
    IS_TEST = False

if IS_TEST:
    INPUT_FILE = "test.txt"
else:
    INPUT_FILE = "input.txt"

with open(INPUT_FILE) as f:
    lines = f.readlines()
lines = [line[:-1] for line in lines]

# Part 1
val = 2019
deck_len = 10007
for i in range(1):
    for line in lines:
        val = run_command(line, val)
print(val % deck_len)

# Part 2
def egcd(a, b):
    if a == 0:
        return (b, 0, 1)
    else:
        g, y, x = egcd(b % a, a)
        return (g, x - (b // a) * y, y)

def modinv(a, m):
    g, x, y = egcd(a + m, m)
    if g != 1:
        raise Exception('modular inverse does not exist')
    else:
        return x % m

def inv_deal(a, b, deck_len):
    return -a % deck_len, (-b - 1) % deck_len

def inv_cut(a, b, cut_i, deck_len):
    return a % deck_len, (b + cut_i) % deck_len

def inv_increment(a, b, inc, deck_len):
    inv = modinv(inc, deck_len)
    return (a * inv) % deck_len, (b * inv) % deck_len

def run_inv_command(line, a, b, deck_len):
    (command, num) = line.rsplit(" ", 1)
    if num == 'stack':
        (new_a, new_b) = inv_deal(a, b, deck_len)
    else:
        num = int(num)
        if command == 'cut':
            (new_a, new_b) = inv_cut(a, b, num, deck_len)
        else:
            (new_a, new_b) = inv_increment(a, b, num, deck_len)
    return new_a, new_b

val = 2020
deck_len = 119315717514047
N = 101741582076661
(a, b) = (1, 0)
for line in lines[::-1]:
    (a, b) = run_inv_command(line, a, b, deck_len)
# f(x) = a * x + b
# orig = f(f(...f(final))), f applied N times
# = a^N * x + a^(N - 1) * b + a^(N - 2) * b ... + b
# = a^N * x + b ( (1 - a^(N-1)) / (1 - a))
print(a, b)

new_val = pow(a, N, deck_len) * val + b * (pow(a, N, deck_len) - 1) * modinv(a - 1, deck_len)
new_val = new_val % deck_len
# new_val = (a * val + b) % deck_len
print(new_val)

