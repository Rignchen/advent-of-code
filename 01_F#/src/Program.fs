let readLines filePath = System.IO.File.ReadLines(filePath);;
let lines = readLines "example.txt";;
let numbers = lines |> Seq.map int |> Seq.toList;;

printfn "%A" numbers;;

