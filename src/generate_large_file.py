import os
import random

def create_large_file(filename, target_size=1_000_000_000, line_length=100):
    with open(filename, 'w') as file:
        while os.path.getsize(filename) < target_size:
            file.write('a' * (line_length - 1) + '\n')

if __name__ == '__main__':
    create_large_file('test_file.txt')
