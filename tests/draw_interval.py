#!/bin/python3

import sys
import matplotlib.pyplot as plt

def draw_range(i, range_from, range_to):
    my_range = list(range(range_from, range_to+1))
    my_range[-1] -=0.1
    plt.plot(my_range, [i]*(range_to-range_from+1), linewidth=1)
    plt.plot([my_range[-1], my_range[-1]+0.1], [i]*2, linewidth=1)

def draw_intervals(intervals):
    func_num = int(intervals[0])
    labels = []
    cnt = 1
    print("func_num = ", func_num)
    for i in range(0, func_num):
        label_num = int(intervals[cnt])
        print("label_num = ", label_num)
        cnt += 1
        max_to = 0
        font_size=128/label_num
        if font_size < 0:
            font_size=1
        
        for j in range(0, label_num):
            label = intervals[cnt]
            labels.append(label)
            cnt += 1
            print(label)
            range_num = int(intervals[cnt])
            cnt += 1
            for k in range(0, range_num):
                range_from = int(intervals[cnt])
                cnt += 1
                range_to = int(intervals[cnt])
                cnt += 1
                print("[", range_from, ", ", range_to, "]")
                draw_range(j, range_from, range_to)
                if range_to > max_to:
                    max_to = range_to

        plt.xticks(list(range(0, max_to+1)), rotation=90, fontsize=font_size)
        plt.yticks(list(range(0, len(labels))), labels=labels, fontsize=font_size)
        plt.grid(linewidth=0.5, color='gray')
        plt.tight_layout()
        plt.savefig('target/{}.svg'.format(i))
        labels.clear()
        plt.clf()


if __name__ == "__main__":
    with open(sys.argv[1]) as interval_log:
        intervals = interval_log.read()
    intervals = intervals.split()
    draw_intervals(intervals)

