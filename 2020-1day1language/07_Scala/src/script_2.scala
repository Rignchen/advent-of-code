class ArrayList[T](maxSize: Int)(using m: scala.reflect.Manifest[T]) {
  val array = new Array[T](maxSize)
  var length = 0
  def apply(index: Int): T = {
    array(index)
  }
  def add(value: T): Unit = {
    array(length) = value
    length += 1
  }
  def copyTo(other: ArrayList[T]): Unit = {
    for (i <- 0 until length) {
      other.add(array(i))
    }
  }
  def empty(): Unit = {
    length = 0
  }
  def copy(): ArrayList[T] = {
    val out = new ArrayList[T](maxSize)
    copyTo(out)
    return out
  }
  def replaceWith(other: ArrayList[T]): Unit = {
    empty()
    other.copyTo(this)
  }
  def any(f: T => Boolean): Boolean = {
    for (i <- 0 until length) {
      if (f(array(i))) {
        return true
      }
    }
    return false
  }
  def forEach(f: T => Unit): Unit = {
    for (i <- 0 until length) {
      f(array(i))
    }
  }
  def copyIfNotExists(other: ArrayList[T]): Unit = {
    for (i <- 0 until array.length) {
      if (!other.any(_ == array(i))) {
        other.add(array(i))
      }
    }
  }
  def filter(f: T => Boolean): ArrayList[T] = {
    val out = new ArrayList[T](maxSize)
    for (i <- 0 until length) {
      if (f(array(i))) {
        out.add(array(i))
      }
    }
    return out
  }
}

class Bag(val color: String, val contains: Array[String]) {
  override def toString(): String = {
    s"$color: ${contains.mkString(", ")}"
  }
  def canContain(bag: Bag): Boolean = {
    contains.contains(bag.color)
  }
  def canContain(bags: ArrayList[Bag]): Boolean = {
    bags.any(canContain)
  }
}

object Script1 {
  def main(args: Array[String]) = {
    val data = readFile(args)

    val can_contain = new ArrayList[Bag](data.length)
    val new_can_contain = new ArrayList[Bag](data.length)
    new_can_contain.add(new Bag("shiny gold", Array()))
    val old_can_contain = new ArrayList[Bag](data.length)

    while
      old_can_contain.replaceWith(new_can_contain)
      new_can_contain.empty()
      data.filter(_.canContain(old_can_contain)).foreach(new_can_contain.add)
      new_can_contain.copyIfNotExists(can_contain)
      new_can_contain.length != 0
    do ()

    val out = can_contain.filter(_ != null)

    out.forEach(println)
    println(out.length)
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
