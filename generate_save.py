import argparse
import json

parser = argparse.ArgumentParser(
    "generate_save",
    "Generates applicable save for game pfvz with proper unlocked items",
)
parser.add_argument("stage", type=int)
parser.add_argument("level", type=int)
parser.add_argument("--expansion", "-e", default=False, action="store_true")
parser.add_argument("--file", "-f", type=str, default="save.json")
args = parser.parse_args()

assert 0 < args.stage
assert 0 < args.level <= 10

plants = (args.stage - 1) * 8 + args.level
plants -= args.level // 5
slots = args.stage + 5

adventure = list()
for stage_index in range(1, args.stage + 1):
    if stage_index == args.stage:
        level_max = args.level
    elif args.expansion:
        level_max = 20
    else:
        level_max = 11

    for level_index in range(1, level_max + 1):
        adventure.append((stage_index, level_index))

with open(args.file, "w") as f:
    j = {
        "slots": slots,
        "selection": [],
        "money": 0,
        "plants": [i for i in range(-plants, 0)],
        "adventure": adventure,
        "ach": [],
    }
    json.dump(j, f)
