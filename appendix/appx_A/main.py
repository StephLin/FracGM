import random
from fracgm import FracGM

# random initial guess
initial_list = random.sample(range(-100000, 100000), 10)

for initial_guess in initial_list:
    print("=" * 25)
    FracGM().solve(initial=initial_guess)
