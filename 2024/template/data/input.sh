#!/bin/bash

# get the day
cd $(dirname $0)
DAY=$(echo $PWD | rev | cut -d'/' -f 2 | rev | sed 's/^0*//')

# ensure the day is a number between 1 and 25 (included)
if ! [[ "$DAY" =~ ^([1-9]|1\d|2[0-5])$ ]]; then
	echo "Invalid day \"$DAY\", must be a number between 1 and 25 (included)"
	exit 1
fi

# get the input
RESULT=$(../../input.sh $DAY)
if [[ "$RESULT" == *"Please don't repeatedly request this endpoint before it unlocks!"* ]]; then
	echo "$RESULT"
	exit 1
fi

# save the input in a file
echo "$RESULT" > input.txt
rm input.sh
