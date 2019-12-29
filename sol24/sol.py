from copy import deepcopy
import sys

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
width = len(lines[0])
height = len(lines)

# Part 1
def index_val(state, x, y):
    if x < 0 or x > width - 1 or y < 0 or y > height - 1:
        return 0
    return 1 if state[y][x] == '#' else 0

def print_state(state):
    for row in state:
        print(''.join(row))
    print('')

def biodiversity(state):
    i = 0
    val = 0
    for y in range(height):
        for x in range(width):
            if state[y][x] == '#':
                val += pow(2, i)
            i += 1
    return val

seen = set()
initial = tuple(tuple(row) for row in lines)
state = initial
seen.add(state)
iteration = 0
while True:
    if iteration % 50 == 0:
        print(f"On {iteration} minutes:")
        print_state(state)

    new_state = []
    for y in range(height):
        new_row = []
        for x in range(width):
            adj_count = sum(index_val(state, new_x, new_y) for (new_x, new_y) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)])
            # print(x, y, adj_count)
            if state[y][x] == '.':
                new_row.append('#' if (adj_count == 1 or adj_count == 2) else '.')
            else:
                new_row.append('#' if adj_count == 1 else '.')
        new_state.append(new_row)

    new_state = tuple(tuple(row) for row in new_state)
    state = new_state
    if state in seen:
        break
    seen.add(state)
    iteration += 1

print("REPEAT:")
print_state(state)
print(biodiversity(state))

# Part 2
from collections import defaultdict
state = [[c for c in row] for row in lines]
state[2][2] = '?'
blank = [['.' for x in range(width)] for y in range(height)]
blank[2][2] = '?'

outer_to_inner = {
    (1, 2): [(0, y) for y in range(height)],
    (2, 1): [(x, 0) for x in range(width)],
    (3, 2): [(4, y) for y in range(height)],
    (2, 3): [(x, 4) for x in range(width)]
}
inner_to_outer = defaultdict(list)
for outer, inners in outer_to_inner.items():
    for inner in inners:
        inner_to_outer[inner].append(outer)

print(outer_to_inner)
print(inner_to_outer)

loc_to_neighbors = defaultdict(list)
for y in range(height):
    for x in range(width):
        loc = (x, y)
        for (new_x, new_y) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]:
            if new_x < 0 or new_x >= width or new_y < 0 or new_y >= width:
                continue
            else:
                loc_to_neighbors[loc].append(((new_x, new_y), 0))
        for outer_loc in inner_to_outer.get(loc, []):
            loc_to_neighbors[loc].append((outer_loc, -1))
        for inner_loc in outer_to_inner.get(loc, []):
            loc_to_neighbors[loc].append((inner_loc, 1))


layers = {0: state}
def layered_index_val(layer_num, x, y):
    if layer_num not in layers:
        return 0
    return 1 if layers[layer_num][y][x] == '#' else 0

def print_layers(layers):
    for layer_num in sorted(layers.keys()):
        print(f"Depth {layer_num}")
        print_state(layers[layer_num])

for minute in range(200):
    print(f"Minute {minute}")
    print("")
    print_layers(layers)
    print("=" * 100)
    new_layers = {}
    for num, layer in layers.items():
        new_layer = []
        for y in range(height):
            new_row = []
            for x in range(width):
                cur = layer[y][x]
                if cur == '?':
                    new_row.append('?')
                    continue
                adj_count = sum(layered_index_val(num + layer_diff, new_x, new_y) for ((new_x, new_y), layer_diff) in
                                loc_to_neighbors[(x, y)])
                # print(x, y, adj_count)
                if cur == '.':
                    new_row.append('#' if (adj_count == 1 or adj_count == 2) else '.')
                elif cur == '#':
                    new_row.append('#' if adj_count == 1 else '.')
                else:
                    assert False, f"unexpected char: {cur}"
            new_layer.append(new_row)
            new_layers[num] = new_layer

    # Add new layers if needed
    outermost = min(layers.keys())
    new_outer_num = outermost - 1
    new_outer = None
    for (outer_x, outer_y), inner_locs in outer_to_inner.items():
        adj_count = sum(layered_index_val(outermost, inner_x, inner_y) for (inner_x, inner_y) in inner_locs)
        # print(f"Checking ({outer_x}, {outer_y} against {inner_locs}, found {adj_count} bugs")
        if adj_count == 1 or adj_count == 2:
            if new_outer is None:
                new_outer = deepcopy(blank)
            new_outer[outer_y][outer_x] = '#'
    if new_outer is not None:
        new_layers[new_outer_num] = new_outer

    innermost = max(layers.keys())
    new_inner_num = innermost + 1
    new_inner = None
    for (inner_x, inner_y), outer_locs in inner_to_outer.items():
        adj_count = sum(layered_index_val(innermost, outer_x, outer_y) for (outer_x, outer_y) in outer_locs)
        # print(f"Checking ({inner_x}, {inner_y} against {outer_locs}, found {adj_count} bugs")
        if adj_count == 1 or adj_count == 2:
            if new_inner is None:
                new_inner = deepcopy(blank)
            new_inner[inner_y][inner_x] = '#'
    if new_inner is not None:
        new_layers[new_inner_num] = new_inner

    layers = new_layers


print_layers(layers)
count = 0
for layer in layers.values():
    for row in layer:
        for val in row:
            if val == '#':
                count += 1
print(f"Total {count} bugs")
