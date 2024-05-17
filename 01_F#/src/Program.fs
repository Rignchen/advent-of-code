let readLines filePath = System.IO.File.ReadLines(filePath);;
let lines = readLines "example.txt";;
let numbers = lines |> Seq.map int |> Seq.toList;;


for i in 0..numbers.Length-1 do
   printfn "%d"%d" numbers[i]
