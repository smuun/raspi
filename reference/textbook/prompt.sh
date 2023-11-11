#!/usr/bin/bash

for f in appx*
do
	echo $f
	read number
	if [[ -z $number ]]
	then
		echo doing nothing
	else
		cp $f "${f}.bak"
		echo mv $f "${number}_${f}"
		mv $f "${number}_${f}"
	fi
done

