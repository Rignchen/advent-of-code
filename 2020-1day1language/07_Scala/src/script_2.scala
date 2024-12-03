class ArrayList2[T](maxSize: Int)(using m: scala.reflect.Manifest[T]) {
  val array = new Array[T](maxSize)
  var length = 0
  def apply(index: Int): T = {
    array(index)
  }
  def add(value: T): Unit = {
    array(length) = value
    length += 1
  }
  def copyTo(other: ArrayList2[T]): Unit = {
    for (i <- 0 until length) {
      other.add(array(i))
    }
  }
  def empty(): Unit = {
    length = 0
  }
  def copy(): ArrayList2[T] = {
    val out = new ArrayList2[T](maxSize)
    copyTo(out)
    return out
  }
  def replaceWith(other: ArrayList2[T]): Unit = {
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
  def copyIfNotExists(other: ArrayList2[T]): Unit = {
    for (i <- 0 until array.length) {
      if (!other.any(_ == array(i))) {
        other.add(array(i))
      }
    }
  }
  def filter(f: T => Boolean): ArrayList2[T] = {
    val out = new ArrayList2[T](maxSize)
    for (i <- 0 until length) {
      if (f(array(i))) {
        out.add(array(i))
      }
    }
    return out
  }
  def indexOf(f: T => Boolean): Int = {
    for (i <- 0 until length) {
      if (f(array(i))) {
        return i
      }
    }
    return -1
  }
}

class ColorCount(val color: String, var count: Int) {
  override def toString(): String = {
    s"$count ${color}"
  }
  def add(count: Int): Unit = {
    this.count += count
  }
}

class Bag2(val color: String, val contains: Array[ColorCount]) {
  override def toString(): String = {
    s"$color: ${contains.mkString(", ")}"
  }
  def canContain(bag: Bag2): Boolean = {
    contains.map(_.color).contains(bag.color)
  }
  def canContain(bags: ArrayList2[Bag2]): Boolean = {
    bags.any(canContain)
  }
  
  def countTotalBagContainedIn(rules: Array[Bag2]): Int = {
    var num = 1
    for (i <- 0 until contains.length) {
      val bag = rules.filter(_.color == contains(i).color)(0)
      num = num + contains(i).count * bag.countTotalBagContainedIn(rules)
    }
    num
  }
}

object Script2 {
  def main(args: Array[String]) = {
    val data = readFile(args)
    val goldBag = data.filter(_.color == "shiny gold")(0)
    val num = goldBag.countTotalBagContainedIn(data)
    println(num - 1)
  }

  def readFile(args: Array[String]): Array[Bag2] = {
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
    val out = new Array[Bag2](bags.length)
    for (i <- 0 until bags.length) {
      if (contains(i).length == 1 && contains(i)(0) == "no other") {
        out(i) = new Bag2(bags(i)(0), Array())
      } else {
        // otherwise remove the first word
        val count = contains(i).map(_.split(" ")(0).toInt)
        val color = contains(i).map(_.split(" ").drop(1).mkString(" "))
        out(i) = new Bag2(bags(i)(0), color.zip(count).map((x) => new ColorCount(x._1, x._2)))
      }
    }
    return out
  }
}
