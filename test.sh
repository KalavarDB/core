#!/bin/bash

trap '' SIGINT
trap ''  SIGQUIT
trap '' SIGTSTP

# display message and pause
pause(){
	local m="$@"
	echo "$m"
	read -p "Press [Enter] key to continue..." key
}

# set an
while :
do
	# show menu
	clear
	echo "---------------------------------"
	echo "	     M A I N - M E N U"
	echo "---------------------------------"
	echo "1. Show Dependencies"
	echo "2. Show what users are doing"
	echo "3. Show top memory & cpu eating process"
	echo "4. Show network stats"
	echo "5. Exit"
	echo "---------------------------------"
	read -r -p "Enter your choice [1-5] : " c
	# take action
	case $c in
		1) utilities/version-checker/target/debug/version-checker;
		  echo;
		  pause;;
		2) w| less;;
		3) cargo build && sudo target/debug/kalavar-core;
		   echo;
		   pause;;
		4) netstat -s | less;;
		5) break;;
		*) Pause "Select between 1 to 5 only"
	esac
done

