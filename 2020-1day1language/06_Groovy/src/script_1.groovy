class Script1 {
	static main(String[] args) {
		String[][] answers = get_file_input(args).split("\n\n").collect { it.split("\n") }
		String[] unique_answers = answers.collect { it.join().toSet().join() }
		println(unique_answers.collect { it.size() }.sum())
	}

	static String get_file_input(String[] args) {
		if (args.length == 0) {
			println("Usage: groovy script_1.groovy <file_path>")
			System.exit(1)
		}

		return new File(args[0]).text
	}
}
