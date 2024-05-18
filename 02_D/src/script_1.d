import std.stdio;

void main()
{
        auto file = File("/app/example.txt");
        writefln("File size: %s", file.size());
        writefln("File name: %s", file.name());
        writefln("File type: %s", typeid(file));
        file.close();
}
