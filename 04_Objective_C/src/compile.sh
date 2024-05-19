#!/bin/bash
. /usr/share/GNUstep/Makefiles/GNUstep.sh
FILES=$(ls *.m)
for file in $FILES
do
        gcc -o bin/${file%.m}.o $file `gnustep-config --objc-flags` -lgnustep-base -lobjc
done
