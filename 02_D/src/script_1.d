import std.stdio;
import std.array;
import std.conv;

void main(string[] args)
{
        // read file
        auto file = File(args[1]);
        string[] text;
        while(!file.eof())
        {
                string line = file.readln();
                if (!file.eof())
                {
                        text ~= line;
                }
        }
        file.close();
        writeln("Text: ",text);

        // parse passwords
        struct pswd
        {
                int min;
                int max;
                char letter;
                string password;
        }
        pswd[] passwords;
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
                passwords ~= p;
        }
        writeln("Passwords: ",passwords);

        // count correct passwords
        foreach(pswd p; passwords)
        {
                int count;
                foreach(char c; p.password)
                {
                        if (c == p.letter)
                        {
                                count++;
                        }
                }
                writeln("Password \"", p.password, "\" has ", p.letter, " times the char ", count);
        }
}
