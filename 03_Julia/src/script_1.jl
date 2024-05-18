open("/app/example.txt","r") do f
        while ! eof(f)
                line = readline(f)
                println(line)
        end
end

