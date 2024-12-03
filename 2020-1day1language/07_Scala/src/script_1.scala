object Script1 {
  def main(args: Array[String]) = {
    val file = readFile(args)
    println(file)
  }

  def readFile(args: Array[String]): String = {
    if (args.length != 1) {
      println("Usage: scala script_1.scala <file>")
      sys.exit(1)
    }
    val source = scala.io.Source.fromFile(args(0))
    val lines = try source.mkString finally source.close()
    lines
  }
}
