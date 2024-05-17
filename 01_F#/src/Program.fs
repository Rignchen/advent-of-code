let readLines filePath = System.IO.File.ReadLines(filePath);;
let lines = readLines "example.txt";;

printfn "Lines: %A" lines;;
