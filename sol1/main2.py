import math

def get_total_fuel(mass):
    fuel_mass = math.floor(mass / 3) - 2
    if fuel_mass <= 0:
        return 0
    return fuel_mass + get_total_fuel(fuel_mass)

def main(masses):
    return sum(get_total_fuel(mass) for mass in masses)

print(f"test with 14: {main([14])}")
print(f"test with 1969: {main([1969])}")
print(f"test with 100756: {main([100756])}")

with open('input.txt', 'r') as f:
    masses = [int(line.strip()) for line in f.readlines()]
print(f"Real: {main(masses)}")
