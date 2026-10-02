pub fn array_of(size: u32) {
  for i in 1..=size {
    print!("{}\t", fibonacci(i));
  }
  println!("");
}

fn fibonacci(n: u32) -> u32{
  if n > 2 {
    fibonacci(n - 1) + fibonacci(n - 2)
  } else {
    1
  }
}