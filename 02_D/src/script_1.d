import std.stdio;

void main(string[] args)
{
        auto file = File(args[1]);
        while(!file.eof())
        {
                writefln("Line: %s", file.readln());
        }
        file.close();
}
