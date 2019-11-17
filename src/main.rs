mod mathematical;
mod matchCase;
mod curry;

use self::mathematical::{add};
use self::matchCase::{matchArr};
use self::curry::{curry};

fn main() {
  // add(1, 2);
  // matchArr();
  curry();
  println!("Hello, world!");
}
