// https://rustcc.gitbooks.io/rustprimer/content/closure/as_argument_return_value.html
// rust作为一个安全的语言，函数参数都是固定的，所以不需要柯里化
// 闭包
fn factory() -> Box<Fn(i32) -> i32> {
  let num = 5;

  Box::new(move |x| x + num)
}

pub fn curry() {
  // let b = add(1);
  // println!(b);
}
