#!/bin/bash

git clone https://gitlab.eduxiji.net/nscscc/compiler2022.git
set_dir="compiler2022/"
sysy_dir=$set_dir"公开样例与运行时库/"
lib_dir="libsysy/"
mv $sysy_dir"functional" $set_dir"lv1_functional"
mv $sysy_dir"hidden_functional" $set_dir"lv2_hidden_functional"
mv $sysy_dir"performance" $set_dir"lv3_performance"
mv $sysy_dir"final_performance" $set_dir"lv4_final_performance"
`mkdir $lib_dir`
mv $sysy_dir"sylib.c" $lib_dir
mv $sysy_dir"sylib.h" $lib_dir
`gcc -c $lib_dir"sylib.c" -I $lib_dir -o "libsysy/sylib.o"`
`rm -rf $set_dir"编译系统设计赛通知.pdf" $set_dir"编译系统设计赛章程.pdf" $set_dir"公开样例与运行时库" $set_dir"README.md" $set_dir"SysY2022语言定义-V1.pdf" $set_dir"SysY2022运行时库-V1.pdf" $set_dir".git"`

