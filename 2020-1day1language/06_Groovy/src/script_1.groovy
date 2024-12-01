class Script1 {
	static main(String[] args) {
		println(get_file_input(args))
	}

	static String get_file_input(String[] args) {
		if (args.length == 0) {
			println("Usage: groovy script_1.groovy <file_path>")
			System.exit(1)
		}

		return new File(args[0]).text
	}
}
