// xfail-boot
// Unsigned integer operations

fn main() {
  check (0u8 < 255u8);
  check (0u8 <= 255u8);
  check (255u8 > 0u8);
  check (255u8 >= 0u8);
  check (250u8 / 10u8 == 25u8);
  check (255u8 % 10u8 == 5u8);
  check (0u16 < 60000u16);
  check (0u16 <= 60000u16);
  check (60000u16 > 0u16);
  check (60000u16 >= 0u16);
  check (60000u16 / 10u16 == 6000u16);
  check (60005u16 % 10u16 == 5u16);
  check (0u32 < 4000000000u32);
  check (0u32 <= 4000000000u32);
  check (4000000000u32 > 0u32);
  check (4000000000u32 >= 0u32);
  check (4000000000u32 / 10u32 == 400000000u32);
  check (4000000005u32 % 10u32 == 5u32);

  // 64-bit numbers have some flakiness yet. Not tested
}
