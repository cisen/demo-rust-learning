struct Duck;
struct Pig;
// trait可以作为一个类型，传给泛型限制
trait Fly {
  fn fly(&self) -> bool,
}
// 给Duck实现具体的fly方法（注意impl下面的fly是方法不是函数）
// 这里其实就是多态
impl Fly for Duck {
  fn fly(&self) -> bool {
    return true;
  }
}
impl Fly for Pig {
  fn fly(&self) -> bool {
    return false;
  }
}
// 泛型，静态分发，限制fly_static的参数类型必须是包含fly函数的
fn fly_static<T: Fly>(s: T) -> bool {
  s.fly();
}
// 动态分发，这里&Fly是一种动态类型，表示拥有fly行为
// 这里根泛型的区别是，1. 调用方法不同，2. 底层调用原理不同
fn fly_dyn(s: &Fly) -> bool {
  s.fly()
}
fn main() {
  let pig = Pig;
  assert_eq!(fly_static::<Pig>(pig), flase);
  let duck = Duck;
  assert_eq!(fly_static::<Duck>(duck), true);
  assert_eq!(fly_dyn(&Pig), false);
  assert_eq!(fly_dyn(&Duck), true);
}
