class Script1 {
	static function main() {
		var content:Array<String> = get_input();
		Sys.println(content);
	}

	static function get_input():Array<String> {
		var input_file = "data/example.txt";
		var content:String = sys.io.File.getContent(input_file);

		return ~/\r*\n+/g.split(content).filter(function(line:String):Bool {
			return line.length > 0;
		});
	}
}
