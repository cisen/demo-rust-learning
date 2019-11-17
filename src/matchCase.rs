// match不是用于迭代的，只是类似switch
pub fn matchArr() {
  let mut arr: u32 = 1;
  // assert_eq!(1, arr);

  match arr {
    1 => println!("1"),
    2 => println!("2"),
    _ => println!("_"),
  };
}
