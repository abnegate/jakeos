import collections
import json

WEIGHTS = {"S": 2, "M": 5, "L": 10, "XL": 20}
RANK = ["V0", "V0.5", "V1", "V2", "V3", "V4", "1.0", "LATER"]


def main() -> None:
    tasks = {task["id"]: task for task in json.load(open("generated/index.json"))["tasks"]}
    live = {key: task for key, task in tasks.items() if task["status"] != "dropped"}
    for milestone in RANK:
        todo = [task for task in live.values() if task["milestone"] == milestone and task["status"] != "done"]
        print(f"{milestone:5} todo {len(todo):4} agent-days {sum(WEIGHTS[task['size']] for task in todo):5}")
    scope = {key: task for key, task in live.items() if task["milestone"] == "V0"}
    wave: dict[str, int] = {}
    length: dict[str, int] = {}

    def visit(key: str) -> int:
        if key in wave:
            return wave[key]
        task = scope[key]
        if task["status"] == "done":
            wave[key] = length[key] = 0
            return 0
        deps = [dep for dep in task["depends_on"] if dep in scope and scope[dep]["status"] != "done"]
        wave[key] = 1 + max((visit(dep) for dep in deps), default=0)
        length[key] = WEIGHTS[task["size"]] + max((length[dep] for dep in deps), default=0)
        return wave[key]

    for key in scope:
        visit(key)
    todo = [key for key in scope if scope[key]["status"] != "done"]
    print("V0 waves:", sorted(collections.Counter(wave[key] for key in todo).items()))
    end = max(todo, key=lambda key: length[key])
    print("V0 critical path agent-days:", length[end])
    chain = [end]
    while True:
        deps = [dep for dep in scope[chain[-1]]["depends_on"] if dep in scope and scope[dep]["status"] != "done"]
        if not deps:
            break
        chain.append(max(deps, key=lambda dep: length[dep]))
    print(" <- ".join(chain))


if __name__ == "__main__":
    main()
