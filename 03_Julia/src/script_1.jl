text = []
open("/app/example.txt","r") do f
        while ! eof(f)
                line = readline(f)
                push!(text,line)
        end
end
println(text)

y = 0
count = 0
for line in text
        println("$line : $y")
        if line[y % length(line)+1] == '#'
                println("$y: $line")
                global count += 1
        end
        global y += 3
end

println("Count of tree: $count")

