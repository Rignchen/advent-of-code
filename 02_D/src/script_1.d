import std.stdio;

void main(string[] args)
{
        auto file = File(args[1]);
        writefln("File size: %s", file.size());
        writefln("File name: %s", file.name());
        writefln("File type: %s", typeid(file));
        file.close();
}
