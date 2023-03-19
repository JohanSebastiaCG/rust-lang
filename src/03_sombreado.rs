fn main() {
  let mut x = "name";

  /* se puede declarar una nueva variable con el mismo nombre que una variable anterior. */
  let x = x + 1;

  {
    let x = x * 2;
    println!("the value of x is: {x}");
  }

  println!("the value of x is: {x}");
}