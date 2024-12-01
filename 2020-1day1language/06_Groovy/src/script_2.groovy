class Script1 {
	static main(String[] args) {
		def answers = get_file_input(args).split("\n\n").collect { it.split("\n").collect { it.trim().split("") } }
		// [[[a, b, c]], [[a], [b], [c]], [[a, b], [a, c]], [[a], [a], [a], [a]], [[b]]]

		def common_answers = []

		for (group in answers) {
			// [[a, b], [a, c]]
			def common = group[0]
			for (person in group) {
				// [a, b]
				def temp = []
				for (c in person) {
					// a
					if (common.contains(c)) {
						temp.add(c)
					}
				}
				common = temp
			}
			common_answers.add(common)
		}

		println(common_answers.collect { it.size() }.sum())
	}

	static String get_file_input(String[] args) {
		if (args.length == 0) {
			println("Usage: groovy script_1.groovy <file_path>")
			System.exit(1)
		}

		return new File(args[0]).text
	}
}
