#!/bin/python3

import sys

def remove_blank(s):
    return s.replace(" ", "").replace("\r", "").replace("\n", "")

if __name__ == "__main__":
    with open(sys.argv[1]) as ret_file:
        ret = ret_file.read()

    with open(sys.argv[2]) as ans_file:
        ans = ans_file.read()

    ret = remove_blank(ret)
    ans = remove_blank(ans)
    print(ret == ans)

