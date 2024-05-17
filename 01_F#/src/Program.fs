let readLines filePath = System.IO.File.ReadLines(filePath);;
let inline add a b = a + b;;
let inline multiply a b = a * b;;

let lines = readLines "example.txt";;
let numbers = lines |> Seq.map int |> Seq.toList;;

for i in 0..numbers.Length-2 do
    for j in i+1..numbers.Length-1 do
        let s = add numbers.[i] numbers.[j]
        if s = 2020 then
            let p = multiply numbers.[i] numbers.[j]
            printfn "%d + %d = %d" numbers.[i] numbers.[j] s
            printfn "%d * %d = %d" numbers.[i] numbers.[j] p;;
