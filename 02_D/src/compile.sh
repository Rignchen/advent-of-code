# get the list of all files ending with .d and compile them with ``gdc <name>.d -o bin/<name>.o``
FILES=$(ls *.d)
for file in $FILES
do
    gdc $file -o bin/${file%.d}.o
done

