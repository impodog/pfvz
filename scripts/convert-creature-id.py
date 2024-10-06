import re
import os

for root, dirs, files in os.walk("./src"):
    for name in files:
        path = os.path.join(root, name)
        if path.endswith(".rs"):
            with open(path, "r") as f:
                content = f.read().split("\n")
            prev = -1
            push = 0
            indent = ""
            new_content = list()
            for i in range(len(content)):
                if prev != -1:
                    pat = r"(\s*)map\.insert\((\w+), creature\);"
                    matches = re.match(pat, content[i])
                    if matches:
                        name = matches.group(2)
                        new_content.insert(prev + 1 + push, indent + "id: %s," % name)
                        new_content.append("%smap.insert(creature);" % matches.group(1))
                        push += 1
                        prev = -1
                        continue
                else:
                    pat = r"(\s*)let creature = game::Creature\(Arc::new\(game::CreatureShared \{"
                    matches = re.match(pat, content[i])
                    if matches:
                        prev = i
                        indent = matches.group(1) + " " * 4
                new_content.append(content[i])
            new = "\n".join(new_content)
            with open(path, "w") as f:
                f.write(new)
