# -*- coding: utf-8 -*-
"""
Python solutions to part 1/2 of Advent of Code day 3!

"""

with open("input.txt", "r") as f:
    lines = f.readlines()

# --- Part 1 ---
f = []
for line in lines:
    c = ord(list(
        set(line.strip()[:len(line) // 2]) & set(line.strip()[len(line) // 2:])
    )[0])
    f.append(c - ord('A') + 27 if c < 91 else c - ord('a') + 1)
print(f"Solution to Part 1: {sum(f)}")

# --- Part 2 ---
i, f = 0, []
while i < len(lines):
    c = ord(list(
        set(lines[i].strip()) & set(lines[i+1].strip()) & set(lines[i+2].strip())
    )[0])
    f.append(c - ord('A') + 27 if c < 91 else c - ord('a') + 1)
    i += 3
print(f"Solution to Part 2: {sum(f)}")
