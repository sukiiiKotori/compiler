#!/bin/bash

ext=$1
set_dir=$2
dst_dir=$3
mode=$4
dir_idx=$5
file_idx=$6
time_period="5m"
tmp_log="target/tmp.log"
time_log="target/time.log"

if [[ $mode == "llvm" ]]; then
	CC="clang"
	Machine=""
	compiler_flag="-llvm"
	output_ext=".ll"
	lib="./tests/libsysy/sylib.o"
elif [[ $mode == "riscv" ]]; then
	CC="riscv64-linux-gnu-gcc"
	Machine="qemu-riscv64"
	compiler_flag="-S"
	output_ext=".s"
	lib="./tests/libsysy/sylib.o"
fi
if [[ $5 != "" ]] && [[ $6 != "" ]]; then
	flag="true"
else
	flag="false"
fi

declare -a passed 
declare -a failed
declare -a tiemeouts
declare -a compare_without_blanks
passed_cnt=0
failed_cnt=0
timeouts_cnt=0
cwb_cnt=0
total_time=0

test_file() {
	dir=$1
	src=$2
	out=$3

	name=${src#$dir}
	name=${name%$ext}
	dir_name=${dir#$set_dir}
	in=$dir$name".in"
	dst=$dst_dir$dir_name$name$output_ext
	prog=$dst_dir$dir_name$name

	mkdir -p $dst_dir$dir_name
	echo "[File] $dir_name$name"
	
	if [[ -e $dst ]]; then
		rm $dst
	fi
	./target/debug/compiler $src $compiler_flag -o $dst
	
	if [[ -e $prog ]]; then
		rm $prog
	fi
	$CC -g -Wno-override-module $dst $lib -o $prog

	if [[ -e $tmp_log ]]; then
		rm $tmp_log
	fi
	if [[ -e $time_log ]]; then
		rm $time_log
	fi

	if [[ -e $in ]]; then	
		echo "[in] $in"
		timeout $time_period  $Machine $prog < $in > $tmp_log 2> $time_log
	else 
		timeout $time_period $Machine $prog > $tmp_log 2> $time_log
	fi
	val=$?
	tmp=`cat $tmp_log`
	ans=`cat $out`

	if [[ $tmp == "" ]]; then
		ret=$val
		if [[ $ret == $ans ]]; then
			is_right=1
		else 
			is_right=0
		fi
	else
		ret=$tmp
		`echo -e "$ret\n$val" > $tmp_log`
		ret+=`echo -e "\n$val"`
		if [[ $ret == $ans ]]; then
			is_right=1
		else
			cmp_without_blanks[cwb_cnt]=$dir_name$name
			(( cwb_cnt += 1 ))
			cmp_without_blank=`tests/cmp.py $tmp_log $out`
			if [[ $cmp_without_blank == "True" ]]; then
				is_right=1
			else
				echo "cmp_without_blank = $cmp_without_blank"
				is_right=0
			fi
		fi
	fi

	if [[ $flag == "true" ]]; then
		echo "[SysY]"
		cat $src
		echo ""
		echo "[COMPILED]"
		cat $dst
		echo "RET = {$ret}"
		echo "ANS = {$ans}"
		echo $dir_name$name
	fi

	this_time=`python3 ./tests/parse_time.py $time_log | bc`
	if [[ $this_time == "-1" ]]; then
		this_time=300
	fi
	total_time=`echo $total_time + $this_time | bc`

	if [[ $is_right == 1 ]]; then
		echo -e "\e[1;32mPASS\e[0m"
		passed[passed_cnt]=$dir_name$name
		((passed_cnt += 1))
	else
		echo -e "\e[1;31mFAIL\e[0m"
		failed[fail_cnt]=$dir_name$name
		((fail_cnt += 1))
		if [[ $ret == "124" ]]; then
			timeouts[timeouts_cnt]=$dir_name$name
			((timeouts_cnt += 1))
		fi
	fi
	pass_num=$(($passed_cnt))
	total_num=$(($passed_cnt + ${#failed[@]}))
	echo -e "\e[1;33mUSED TIME\e[0m = $this_time s"
	echo -e "Total \e[1;32mPASS\e[0m = $pass_num/$total_num; \e[1;33mTIME\e[0m = $total_time s"
	echo ""
}

test_in_dir() {
	dir=$1

	dir_name=${dir#$set_dir}
	mkdir -p $dst_dir$dir_name
	echo "[Dir] $dir_name"

	src_list=($(find $dir -maxdepth 1 -type f -name "*$ext" | sort))
	out_list=($(find $dir -maxdepth 1 -type f -name "*.out" | sort))
	if [[ $2 != "" ]]; then
		src=${src_list[$2]}
		out=${out_list[$2]}
		test_file $dir $src $out
	else
		src_num=${#src_list[@]}
		for ((cnt = 0; cnt < src_num; ++cnt))
		do
			test_file $dir ${src_list[$cnt]} ${out_list[$cnt]}
		done
	fi
	echo ""
}

cargo build

dir_list=($(find $set_dir -type d | sort))
unset dir_list[0]

if [[ $dir_idx != "" ]]; then
	test_in_dir ${dir_list[$dir_idx]} $file_idx
else
	for cnt in ${!dir_list[@]}
	do
		test_in_dir ${dir_list[$cnt]} $file_idx
	done
fi

echo "[Failed] {"
for ((i = 0; i < ${#failed[@]}; ++i))
do
	echo "	[$i] ${failed[$i]}"
done
echo "}"

echo "[Timeouts] {"
for ((i = 0; i < ${#timeouts[@]}; ++i))
do
	echo "	[$i] ${timeouts[$i]}"
done
echo "}"

echo "[CompareWithoutBlanks] {"
for ((i = 0; i < ${#cmp_without_blanks[@]}; ++i))
do
	echo "	[$i] ${cmp_without_blanks[$i]}"
done
echo "}"

pass_num=$(($passed_cnt))
total_num=$(($passed_cnt + ${#failed[@]}))
echo -e "Total \e[1;32mPASS\e[0m = $pass_num/$total_num; \e[1;33mTIME\e[0m = $total_time s"

