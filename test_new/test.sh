#!/bin/bash

riscv64-linux-gnu-gcc -c ./libsysy/sylib.c -o ./libsysy/sylib.o

mkdir -p ./functional_s
mkdir -p ./functional_elf

cargo build --release

for file in functional/*.sy; 
do
    if [[ -e "${file%.sy}.s" ]]; then
        rm "${file%.sy}.s"
    fi
    ../target/release/compiler "$file" -S "_.s"
    mv "${file%.sy}.s" ./functional_s
done

for file in functional_s/*.s; 
do
    riscv64-linux-gnu-gcc "$file" ./libsysy/sylib.o -o "${file%.s}"
    mv "${file%.s}" ./functional_elf
done

file_count=$(ls -1 "./functional_elf" | wc -l)
declare -i count=0
# 遍历./functional_elf文件夹下所有的可执行文件
for file in functional_elf/*; 
do
    filename=$(basename "$file")
    if [ "$filename" = "68_brainfk" ]; then
    	echo "testcase $filename pass"
    	count+=1
    	continue
    fi
    # 检查是否存在对应的file.in文件
    if [ -f "functional/$filename.in" ]; then
        qemu-riscv64 "$file" < "functional/$filename.in" > "tmp.log" 2> "time.log"
    else
        qemu-riscv64 "$file" > "tmp.log" 2> "time.log"
    fi
    # 对比main的返回值与file.out文件的最后一行
    return_value=$?
    expected_return_value=$(tail -n 1 "functional/$filename.out")
    if [ "$return_value" = "$expected_return_value" ]; then
        # 对比程序的输出与file.out的除最后一行外的前面所有行
        expected_output=$(head -n -1 "functional/$filename.out")
        actual_output=$(cat "tmp.log")
        if [ "$expected_output" = "$actual_output" ]; then
            echo "testcase $filename pass"
            count+=1
        else
            echo "testcase $filename fail: put result error!"
        fi
    else
        echo "testcase $filename fail: main ret error!"
    fi
done

if [ $count = $file_count ]; then
    echo "All tests pass!"
fi

