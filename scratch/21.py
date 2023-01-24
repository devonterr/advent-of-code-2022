import ast
import re

from sympy.solvers import solve
from sympy import Symbol
from sympy.core.sympify import sympify

test_path = "data/day-21/test.txt"
input_path = "data/day-21/input.txt"

def parse(path):
    result = {}
    with open(path, 'r') as inf:
        for line in inf:
            key, value = line.strip().split(":")
            result[key] = value
    return result

def interpret(ops):
    root = ops["root"]

    ops = {k:v for k,v in ops.items() if k != "root"}
    result = None
    while result == None:
        try:
            result = eval(root)
        except:
            tokens = re.findall("[a-z]+", root)
            replacement_map = {token: ops[token] for token in tokens}

            for to_replace, replace_with in replacement_map.items():
                root = root.replace(to_replace, f"({replace_with})")
    print(result)

class Visitor(ast.NodeVisitor):
    def visit_BinOp(self, node):
        breakpoint()

def interpret2(ops):
    root = ops["root"]
    del ops["humn"]
    lhs, rhs = re.split(" . ", root)
    lhs_root = f"(lambda humn: {lhs})(0)"
    rhs_root = f"(lambda humn: {rhs})(0)"

    ops = {k:v for k,v in ops.items() if k != "root"}
    result = None
    while result == None:
        try:
            result = eval(lhs_root)
        except:
            tokens = re.findall("[a-z]{4}", lhs_root[13:])
            replacement_map = {token: ops[token] for token in tokens if token != "humn"}

            for to_replace, replace_with in replacement_map.items():
                lhs_root = lhs_root.replace(to_replace, f"({replace_with})")

    result = None
    while result == None:
        try:
            result = eval(rhs_root)
        except:
            tokens = re.findall("[a-z]{4}", rhs_root[13:])
            replacement_map = {token: ops[token] for token in tokens if token != "humn"}

            for to_replace, replace_with in replacement_map.items():
                rhs_root = rhs_root.replace(to_replace, f"({replace_with})")

    # print(lhs_root)
    # print(rhs_root)

    lhs_lambda = eval(lhs_root[:-3])
    rhs_lambda = eval(rhs_root[:-3])

    to_search = lhs_lambda if "humn" in lhs_root[12:] else rhs_lambda
    stable = lhs_lambda if "humn" in rhs_root[12:] else rhs_lambda

    stable_result = stable(0)

    # arg = -100000
    # search_result = to_search(arg)
    # while search_result != stable_result:
    #     if arg % 1000 == 0:
    #         print(f"\tSearching... at {arg}, result {search_result}, diff {stable_result - search_result}")
    #     arg += 1
    #     search_result = to_search(arg)

    # print(f"Stable Result {stable_result}")
    # print(f"arg: {arg}, search_result {search_result}")
    # print(f"Stable {stable_result}")
    
    # print(list(ast.walk(ast.parse(lhs_root))))
    # print(list(ast.walk(ast.parse(rhs_root))))

    # Visitor().visit(ast.parse(rhs_root))

    rhs = sympify(stable_result)
    lhs = sympify(lhs_root[14:-4])

    sym = Symbol('humn')

    print(solve(lhs - rhs, sym))

if __name__ == "__main__":
    test_ops = parse(test_path)
    interpret(test_ops)

    input_ops = parse(input_path)
    interpret(input_ops)

    test_ops = parse(test_path)
    interpret2(test_ops)

    input_ops = parse(input_path)
    interpret2(input_ops)
    
