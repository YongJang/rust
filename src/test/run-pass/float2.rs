// xfail-boot
fn main() {
  auto a = 1.5e6;
  auto b = 1.5E6;
  auto c = 1e6;
  auto d = 1E6;
  auto e = 3.0f32;
  auto f = 5.9f64;
  auto g = 1.e6f32;
  auto h = 1.0e7f64;
  auto i = 1.0E7f64;
  auto j = 3.1e+9;
  auto k = 3.2e-10;
 
  check(a == b);
  check(c < b);
  check(c == d);
  check(e < g);
  check(f < h);
  check(g == 1000000.0f32);
  check(h == i);
  check(j > k);
  check(k < a);
}