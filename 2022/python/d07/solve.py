# -*- coding: utf-8 -*-
"""
Python solution to part 1/2 of Advent of Code day 7!

"""

class Directory:
    def __init__(self, name, parent):
        self.name = name
        self.parent = parent
        self.files = {}
        self.dirs = {}

    def evaluate_dir_sizes(self, dir_sizes):
        filesize = sum([v for _, v in self.files.items()])
        dirsize = sum([v.evaluate_dir_sizes(dir_sizes) for _, v in self.dirs.items()])
        dir_sizes.append((self.name, filesize + dirsize))
        return filesize + dirsize


with open("input.txt", "r") as f:
    lines = f.readlines()

file_system = Directory(name="/", parent="")
current_dir = file_system
for line in lines[1:]:
    line = line.strip().split(" ")

    if line[0] == "$":
        if line[1] == "cd":
            changing_to = line[-1]
            if changing_to == "..":
                current_dir = current_dir.parent
            else:
                current_dir = current_dir.dirs[changing_to]
            continue
    else:
        if line[0] == "dir":
            current_dir.dirs.setdefault(
                line[-1],
                Directory(
                    name=line[-1],
                    parent=current_dir
                )
            )
        else:
            current_dir.files.setdefault(
                line[-1],
                int(line[0])
            )

dir_sizes = []
used_ds = file_system.evaluate_dir_sizes(dir_sizes)
total_space = sum([
    size if size < 100000 else 0 for _, size in dir_sizes
])
print(
    "Part 1: The total space used by directories satisfying the size req is "
    f"{total_space}."
)

threshold = used_ds - 40000000

options = [(k, v) for k, v in dir_sizes if v > threshold]

print(
    "Part 2: the size of smallest directory that, when deleted, will satisfy"
    f" the requirement is {min([v for _, v in options])}."
)
