text = []
open("/app/example.txt","r") do f
        while ! eof(f)
                line = readline(f)
                push!(text,line)
        end
end
println(text)

