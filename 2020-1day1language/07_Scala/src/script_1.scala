class Bag(val color: String, val contains: Array[String]) {
  override def toString(): String = {
    s"$color: ${contains.mkString(", ")}"
  }
  def canContain(bag: Bag): Boolean = {
    contains.contains(bag.color)
  }
  def canContain(bags: Array[Bag]): Boolean = {
    bags.exists(canContain)
  }
}

object Script1 {
  def main(args: Array[String]) = {
    val data = readFile(args)
    val can_contain = new Array[Bag](1)
    can_contain(0) = new Bag("shiny gold", Array())
    data.filter(_.canContain(can_contain)).foreach(println)
  }

  def readFile(args: Array[String]): Array[Bag] = {
    if (args.length != 1) {
      println("Usage: scala script_1.scala <file>")
      sys.exit(1)
    }
    val source = scala.io.Source.fromFile(args(0))
    val lines = try source.mkString finally source.close()
    // split lines by new lines, then split each line by " bags contain "
    val bags = lines.split("\n").map(_.split(" bags contain "))
    // split the second part on commas, trim them and remove the last word in it
    val contains = bags.map((bag: Array[String]) => bag(1).split(",").map(_.trim().split(" ").dropRight(1).mkString(" ")))
    // if the contain is "no other", then the array is empty
    val out = new Array[Bag](bags.length)
    for (i <- 0 until bags.length) {
      if (contains(i).length == 1 && contains(i)(0) == "no other") {
        out(i) = new Bag(bags(i)(0), Array())
      } else {
        // otherwise remove the first word
        contains(i) = contains(i).map(_.split(" ").drop(1).mkString(" "))
        out(i) = new Bag(bags(i)(0), contains(i))
      }
    }
    return out
  }
}
