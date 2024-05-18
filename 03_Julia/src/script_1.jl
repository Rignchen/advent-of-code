text = []
open("/app/example.txt","r") do f
        while ! eof(f)
                line = readline(f)
                push!(text,line)
        end
end
println(text)

y = 0
for line in text
        println("$line : $y")
        println(line[y % length(line)+1])
        global y += 3
end
