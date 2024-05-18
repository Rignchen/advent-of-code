import std.stdio;
import std.array;

void main(string[] args)
{
        auto file = File(args[1]);
        struct pswd
        {
                int min;
                int max;
                char letter;
                string password;
        }
        string[] text;
        pswd[] passwords;
        while(!file.eof())
        {
                string line = file.readln();
                text ~= line;
        }
        file.close();
        writeln("Text: %s", text);
        foreach(string line; text)
        {

                // lines = min-max letter: password
                string[] exploded = line.split(" ");
                writeln(exploded);
        }
}
