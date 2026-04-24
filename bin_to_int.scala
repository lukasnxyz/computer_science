import scala.math.pow
import scala.annotation.tailrec

@tailrec
def btoi(b: Int, i: Int=0, n: Int=0): Int =
  b match {
    case 0b0 => n
    case _ => btoi(
      b / 0b10,
      i + 1,
      n + (b % 0b10 * pow(2, i).toInt),
    )
  }

@main def main(): Unit =
  val n: Int = 0b10101
  println(s"binary (${n}) to integer: ${btoi(n)}")
