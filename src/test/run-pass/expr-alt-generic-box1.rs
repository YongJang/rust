// xfail-boot
// -*- rust -*-

type compare[T] = fn(@T t1, @T t2) -> bool;

fn test_generic[T](@T expected, &compare[T] eq) {
  let @T actual = alt (true) {
    case (true) {
      expected
    }
  };
  check (eq(expected, actual));
}

fn test_box() {
  fn compare_box(@bool b1, @bool b2) -> bool {
    ret *b1 == *b2;
  }
  auto eq = bind compare_box(_, _);
  test_generic[bool](@true, eq);
}

fn main() {
  test_box();
}
