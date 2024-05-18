import std.stdio;
import std.array;
import std.conv;

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
                if (!file.eof())
                {
                        text ~= line;
                }
        }
        file.close();
        writeln("Text: %s", text);
        foreach(string line; text)
        {
                string[] exploded = line.split(" ");
                string[] range = exploded[0].split("-");
                pswd p = pswd(
                        to!int(range[0]),
                        to!int(range[1]),
                        exploded[1][0],
                        exploded[2]
                );
                writeln(p);
                passwords ~= p;
        }
        writeln("Passwords: ",passwords);
}
