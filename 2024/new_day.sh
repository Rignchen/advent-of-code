#!/bin/bash

format_day() {
	DAY=$(echo $1 | sed 's/^0*//')
	if [[ $DAY -lt 10 ]]; then
		DAY="0$(echo $DAY | sed 's/^0*//')"
	fi
	echo $DAY
}
validate_day() {
	if [ -d $(format_day $1) ]; then
		printf "Day already exists\n"
		exit 1
	fi
	if [[ $1 -lt 1 ]] || [[ $1 -gt 25 ]]; then
		printf "Invalid day\n"
		exit 1
	fi
}
prepare_for_day() {
	DAY=$(format_day $1)
	test "$(validate_day $DAY)" && exit 1
	cp -r template $DAY
	echo $DAY
}

if ! [ "$1" ]; then
	# get the currente date (day)
	DAY=$(date +%d)

	# change the date to a 2 digit number
	validate_day $DAY
	if [[ $? -gt 0 ]]; then
		exit 1
	fi
	DAY=$(prepare_for_day $DAY)

	# check if 6am has passed
	if [ $(date +%H) -lt 6 ]; then
		# wait until 6am
		WAIT=$((((6 - $(date +%H)) * 60 - $(date +%M)) * 60 - $(date +%S)))
		printf "Waiting until 6am to create new day($WAIT sec)\n"
		sleep $WAIT
	fi
else
	validate_day $DAY
	if [[ $? -gt 0 ]]; then
		exit 1
	fi
	DAY=$(prepare_for_day $1)
fi

# run initialisation script for the new day
$DAY/data/input.sh

