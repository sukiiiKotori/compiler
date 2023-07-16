#!/bin/python3

import sys
import re

def pat_split(time):
    time= re.search(r'TOTAL: (\d+)H-(\d+)M-(\d+)S-(\d+)us', time)
    return time

def calc_time(time):
    return float(time[0])*3600 + float(time[1])*60 + float(time[2]) + float(time[3])/1000000

if __name__ == "__main__":
    with open(sys.argv[1]) as time_file:
        time = time_file.read()

    time = pat_split(time)
    if time == None:
        print(-1)
    else:
        time = calc_time(time.groups())
        print(time)

