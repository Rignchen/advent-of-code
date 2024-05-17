let readLines filePath = System.IO.File.ReadLines(filePath);;
let inline add a b c = a + b + c;;
let inline multiply a b c = a * b * c;;

let lines = readLines "example.txt";;
let numbers = lines |> Seq.map int |> Seq.toList;;

for i in 0..numbers.Length-3 do
    for j in i+1..numbers.Length-2 do
        for k in j+1..numbers.Length-1 do
            let s = add numbers.[i] numbers.[j] numbers.[k]
            if s = 2020 then
                let p = multiply numbers.[i] numbers.[j] numbers.[k]
                printfn "%d + %d + %d = %d" numbers.[i] numbers.[j] numbers.[k] s
                printfn "%d * %d * %d = %d" numbers.[i] numbers.[j] numbers.[k] p;;

