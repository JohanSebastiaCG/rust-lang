fn main() {
  let num = 5;
  println!("the value of the variable is: {num}");

  /*ERORR variables inmutables: cannot assign twice to immutable variable `x` */
  num = 6;
  println!("the value of the variable is: {num}");

  /*Variables mut */

  let mut data = 10;
  println!("the value of the variables is: {data}");

  /*Variable mutable: very good!"*/
  data = 11;
  println!("the value of the variables is: {data}");
}