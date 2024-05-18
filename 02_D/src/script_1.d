import std.stdio;

void main(string[] args)
{
        auto file = File(args[1]);
        string[] text;
        while(!file.eof())
        {
                string line = file.readln();
                writef("Line: %s", line);
                text ~= line;
        }
        file.close();
        writefln("Text: %s", text);
}
