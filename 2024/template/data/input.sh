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
RESULT=$(curl "https://adventofcode.com/2024/day/$DAY/input" --compressed -H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:132.0) Gecko/20100101 Firefox/132.0' -H 'Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8' -H 'Accept-Language: en-US,en;q=0.5' -H 'Accept-Encoding: gzip, deflate, br, zstd' -H 'Referer: https://adventofcode.com/2024/day/3' -H 'Connection: keep-alive' -H 'Cookie: session=53616c7465645f5fd18e1748f44af29b97d88d24304e9989e69b229878a128ffabff30c7274a16f03cd907b810ec8dd16b9b559107d43ac95a4f76e50842cabc' -H 'Upgrade-Insecure-Requests: 1' -H 'Sec-Fetch-Dest: document' -H 'Sec-Fetch-Mode: navigate' -H 'Sec-Fetch-Site: same-origin' -H 'Priority: u=0, i' -H 'TE: trailers')
if [[ "$RESULT" == *"Please don't repeatedly request this endpoint before it unlocks!"* ]]; then
	echo "$RESULT"
	exit 1
fi

# save the input in a file
echo "$RESULT" > input.txt
rm input.sh
