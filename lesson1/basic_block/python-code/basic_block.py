#!/usr/bin/python3

import json
import sys

TERMINATORS = ['jmp', 'br', 'ret']


def form_blocks(body):
    cur_block = []

    for instr in body:
        if 'op' in instr:
            cur_block.append(instr)
            if instr['op'] in TERMINATORS:
                yield cur_block
                cur_block = []
        else:
            if cur_block:
                yield cur_block
            cur_block = [instr]

    if cur_block:
        yield cur_block


def main():
    if len(sys.argv) <= 1:
        print("Usage: {} <json-file>".format(sys.argv[0]))
        sys.exit(-1)

    path = sys.argv[1]
    with open(path) as f:
        prog = json.load(f)
    for func in prog['functions']:
        for block in form_blocks(func['instrs']):
            print("=> {}".format(json.dumps(block)))


if __name__ == '__main__':
    main()
