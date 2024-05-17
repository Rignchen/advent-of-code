let readLines filePath = System.IO.File.ReadLines(filePath);;
let lines = readLines "example.txt";;
let numbers = lines |> Seq.map int |> Seq.toList;;

for i in 0..numbers.Length-2 do
    for j in i+1..numbers.Length-1 do
        printfn "%d %d" numbers.[i] numbers.[j];;
