from collections import defaultdict

def bfs(start, end_condition, loc_to_neighbors):
    horizon = [start]
    seen = set(start)
    steps = 1
    while len(horizon) > 0:
        new_horizon = []
        for loc in horizon:
            neighbors = loc_to_neighbors[loc]
            for neighbor in neighbors:
                if neighbor in seen:
                    continue
                new_horizon.append(neighbor)
                seen.add(neighbor)
                if end_condition(neighbor):
                    return steps
        steps += 1
        horizon = new_horizon

def recursive_bfs(start, end_condition, loc_to_neighbors, outer_to_inner, inner_to_outer):
    horizon = [(start, 0)]
    seen = set((start, 0))
    steps = 1
    while len(horizon) > 0:
        new_horizon = []
        for (loc, layer) in horizon:
            neighbors = loc_to_neighbors[loc]

            for neighbor in neighbors:
                if outer_to_inner.get(loc) == neighbor:
                    # Went up a level
                    if layer == 0:
                        continue
                    layer -= 1
                elif inner_to_outer.get(loc) == neighbor:
                    # Went down a level
                    if layer == len(inner_to_outer) - 1:
                        break
                    layer += 1

                if (neighbor, layer) in seen:
                    continue
                new_horizon.append((neighbor, layer))
                seen.add((neighbor, layer))
                if end_condition((neighbor, layer)):
                    return steps
        steps += 1
        horizon = new_horizon

INPUT_FILE = "input.txt"
# INPUT_FILE = "test2.txt"

with open(INPUT_FILE) as f:
    lines = f.readlines()

lines = [line[:-1] for line in lines]
width = len(lines[0])
height = len(lines)

locs = []
portal_to_locs = defaultdict(list)
loc_to_neighbors = defaultdict(list)
end_position = None
for y, row in enumerate(lines):
    for x, val in enumerate(row):
        locs.append((x, y))
        if val != '.':
            continue
        for ((x_n, y_n), (x_nn, y_nn)) in [((x + 1, y), (x+2, y)), ((x - 1, y), (x-2,y)), ((x, y - 1), (x,y-2)), ((x, y + 1), (x,y+2))]:
            val_n = lines[y_n][x_n]
            if val_n == '.':
                loc_to_neighbors[(x, y)].append((x_n, y_n))
            elif val_n.isalpha():
                if x_nn < x_n or y_nn < y_n:
                    portal_name = lines[y_nn][x_nn] + val_n
                else:
                    portal_name = val_n + lines[y_nn][x_nn]
                portal_to_locs[portal_name].append((x, y))

# Now add portal neighbors
for portal, locs in portal_to_locs.items():
    # AA and ZZ aren't portals
    if len(locs) > 1:
        loc_to_neighbors[locs[0]].append(locs[1])
        loc_to_neighbors[locs[1]].append(locs[0])

# Part 1
# Now bfs to find shortest path
start = portal_to_locs['AA'][0]
end = portal_to_locs['ZZ'][0]
steps = bfs(start, lambda loc: loc == end, loc_to_neighbors)
print(steps)

# Part 2
outer_to_inner = {}
inner_to_outer = {}
for portal, locs in portal_to_locs.items():
    if len(locs) == 1:
        continue
    loc_a = locs[0]
    loc_b = locs[1]
    if loc_a[0] == 2 or loc_a[0] == width - 3 or loc_a[1] == 2 or loc_a[1] == height - 3:
        outer_to_inner[loc_a] = loc_b
        inner_to_outer[loc_b] = loc_a
    else:
        outer_to_inner[loc_b] = loc_a
        inner_to_outer[loc_a] = loc_b
print("out to in:", outer_to_inner)
print("in to out:", inner_to_outer)
start = portal_to_locs['AA'][0]
end = portal_to_locs['ZZ'][0]
steps = recursive_bfs(start, lambda state: state[0] == end and state[1] == 0, loc_to_neighbors, outer_to_inner, inner_to_outer)
print(steps)

