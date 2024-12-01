class Script2 {
	static function main() {
		var content:Array<String> = get_input();

		var ids:Array<Int> = [];
		for (line in content) {
			var seat:{row:Int,column:Int,id:Int} = parse_input(line);
			ids.push(seat.id);
		}

		ids.sort(function(a:Int, b:Int):Int {
			return a - b;
		});

		for (i in 0...ids.length) {
			if (ids[i] + 1 != ids[i + 1]) {
				trace("Your seat is: " + (ids[i] + 1));
				break;
			}
		}
	}

	static function get_input():Array<String> {
		var input_file = "data/input.txt";
		var content:String = sys.io.File.getContent(input_file);

		return ~/\r*\n+/g.split(content).filter(function(line:String):Bool {
			return line.length > 0;
		});
	}

	static function parse_input(input:String):{row:Int,column:Int,id:Int} {
		// the inputs are dormated as such:
		// the first 7 character are the row, they are either F or B
		// F means the lower half, B means the upper half, the possible range is 0-127
		// the last 3 character are the column, they are either L or R
		// L means the lower half, R means the upper half, the possible range is 0-7

		var rows:Array<Int> = [0, 127];
		var columns:Array<Int> = [0, 7];

		var row:Int = 0;
		var column:Int = 0;

		for (i in 0...10) {
			/*
			trace("Rows: " + rows);
			trace("Columns: " + columns);
			trace("Row: " + row);
			trace("Column: " + column);
			trace("--------------------");
			*/
			switch (input.charAt(i)) {
				case 'F':
					if (i == 6) {
						row = rows[0];
					}
					else {
						rows = [rows[0], Std.int((rows[0] + rows[1]) / 2)];
					}
				case 'B':
					if (i == 6) {
						row = rows[1];
					}
					else {
						rows = [Std.int((rows[0] + rows[1]) / 2) + 1, rows[1]];
					}
				case 'L':
					if (i == 9) {
						column = columns[0];
					}
					else {
						columns = [columns[0], Std.int((columns[0] + columns[1]) / 2)];
					}
				case 'R':
					if (i == 9) {
						column = columns[1];
					}
					else {
						columns = [Std.int((columns[0] + columns[1]) / 2) + 1, columns[1]];
					}
			}
		}

		return {row: row, column: column, id: row * 8 + column};
	}
}
