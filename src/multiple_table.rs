pub fn table_of(size: u32) {
  for i in 1..=size {
    for j in 1..=i {
      print!("{}×{}={}\t", i, j, i * j);
    }
    println!("")
  }
}