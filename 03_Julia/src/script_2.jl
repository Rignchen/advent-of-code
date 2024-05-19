text = []
open("/app/example.txt","r") do f
        while ! eof(f)
                line = readline(f)
                push!(text,line)
        end
end
println(text)

result = 1
dif_x = [1,3,5,7,1]
dif_y = [1,1,1,1,2]
for i in 1:5
        x = 0
        y = 0
        count = 0
        while y < length(text)
                line = text[y + 1]
                println("$line : $x - $y")
                if line[x % length(line) + 1] == '#'
                        println("$x - $y: $line")
                        count += 1
                end
                x += dif_x[i]
                y += dif_y[i]
        end
        println("Count of tree for slide $i: $count")
        global result *= count
end

println("Count of tree: $result")

