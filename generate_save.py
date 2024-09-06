import argparse
import json

parser = argparse.ArgumentParser(
    "generate_save",
    "Generates applicable save for game pfvz with proper unlocked items",
)
parser.add_argument("stage", type=int)
parser.add_argument("level", type=int)
parser.add_argument("--file", "-f", type=str, default="save.json")
args = parser.parse_args()

assert 0 < args.stage
assert 0 < args.level <= 10

plants = (args.stage - 1) * 8 + args.level
plants -= args.level // 5
slots = args.stage + 5

with open(args.file, "w") as f:
    j = {
        "slots": slots,
        "selection": [],
        "money": 0,
        "plants": [i for i in range(-plants, 0)],
        "adventure": {"stage": args.stage, "level": args.level},
        "ach": [],
    }
    json.dump(j, f)
