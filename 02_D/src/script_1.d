import std.stdio;

void main(string[] args)
{
        auto file = File(args[1]);
        writefln("First line: %s", file.readln());
        file.close();
}
