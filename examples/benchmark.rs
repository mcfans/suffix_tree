use core::arch::aarch64::vld1q_u8_x3;
use std::{ptr, arch::aarch64::{vld1q_u8, vld1q_dup_u32, vceqq_u8, vceqq_u32, vreinterpretq_u32_u8, vget_lane_u64, vgetq_lane_u64, vreinterpretq_u64_s32, vreinterpretq_u64_u32, uint32x4_t, vgetq_lane_u32, vclzq_u32, vreinterpretq_p128_u32}};
use suffix_tree::check_contains;


fn main() -> Result<(), &'static str> {
    let mut args = std::env::args();
    let pattern = args.nth(1).ok_or("123")?;
    let text = args.nth(0).ok_or("234")?;

    println!("{}", pattern);

    let mut res = 0;
    for _ in 0..100000000 {
        res = check_contains(&pattern, &text);
    }
    println!("{}", res);
    Ok(())
    // let mut res = 0;
    // for i in 0..1000000000 {
        // res = check_contains("abcd", "badcabcdbadcbadcbadcabcebadcbadcbadcabcebadcbadc");
    // }
    // println!("{}", res);
}