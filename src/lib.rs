pub mod tree;
// use core::arch::aarch64::vld1q_u8_x3;
// use std::{arch::aarch64::{uint32x4_t, vandq_u32, vceqq_u32, vceqq_u8, vclzq_u32, vget_high_u64, vget_lane_u64, vgetq_lane_u32, vgetq_lane_u64, vld1q_dup_u32, vld1q_u8, vorrq_u32, vorrq_u8, vreinterpret_u64_u16, vreinterpretq_p128_u32, vreinterpretq_u32_u8, vreinterpretq_u64_s32, vreinterpretq_u64_u32, vreinterpretq_u8_u32, vshlq_n_u32, vshr_n_u32, vshrn_n_u32, vshrq_n_u32}, mem::MaybeUninit, ptr, vec};
// mod tree;

// #[inline]
// pub fn check_contains(pattern: &str, text: *const char, len: usize) -> i8 {
//     debug_assert!(pattern.len() == 4);
//     debug_assert!(len % 4 == 0);
//     let loop_needed = len / 4;

//     for i in 0..loop_needed {
//         let offset = i * 4;
//         unsafe {
//             let ptr_to_text_arr = text.add(offset);
//             let v = vld1q_u8(ptr_to_text_arr as *const u8);

//             let ptr_to_pattern_arr = pattern.as_ptr();
//             let pattern_v = vld1q_dup_u32(ptr_to_pattern_arr as *const u32);

//             // let first_register = vreinterpretq_u32_u8(v);

//             let pattern_u8 = vreinterpretq_u8_u32(pattern_v);

//             let first_register_eq = vceqq_u8(v, pattern_u8);

//             let after_or = vorrq_u8(first_register_eq, pattern_u8);

//             let could_use_clz = vshrq_n_u32::<31>(vreinterpretq_u32_u8(after_or));

//             let could_use_clz = vshlq_n_u32::<31>(could_use_clz);

//             let shifted = vshrn_n_u32(could_use_clz, 16);

//             let convert_to_64 = vreinterpret_u64_u16(shifted);

//             let number = vget_lane_u64::<0>(convert_to_64);

//             let first_res = 3i8 - ((number.leading_zeros() >> 4) as i8);

//             if first_res != -1 {
//                 return first_res;
//             }

//             return -1;
//         }
//     }
//     return -1;
// }

// fn match_index(pattern: &str, texts: &Vec<String>) -> i8 {
//     let padding = 4 - pattern.len() % 4;
//     let len = texts.len() * 4 + padding;
//     let mut data = Vec::with_capacity(len);

//     for text in texts {
//         let need_add_size = 4 - text.len();
//         for _ in 0..need_add_size {
//             data.push(0xFF);
//         }
//         data.extend(text.as_bytes());
//     }
//     for _ in 0..padding {
//         data.push(0x00);
//     }

//     check_contains(pattern, data.as_ptr() as *const char, len)
// }

// #[inline]
// unsafe fn check_u128_first_u32_with_all_1(v: uint32x4_t) -> i8 {
//     let shifted = vshrn_n_u32(v, 16);
//     let convert_to_64 = vreinterpret_u64_u16(shifted);
//     let number = vget_lane_u64::<0>(convert_to_64);

//     3i8 - ((number.leading_zeros() >> 4) as i8)
// }

// #[inline]
// pub fn check_contains2(pattern: &str, text: &str) -> i8 {
//     unsafe {
//         let arr: [u8; 4] = pattern.as_bytes()[..4].try_into().unwrap_unchecked();
//         let pattern_v = std::mem::transmute::<[u8; 4], u32>(arr);

//         let text = text.as_bytes();
//         for i in 0i8..18i8 {
//             let offset: usize = (i * 4).try_into().unwrap_unchecked();
//             let content: [u8; 4] = text[offset .. offset + 4].try_into().unwrap_unchecked();
//             let content = std::mem::transmute::<[u8; 4], u32>(content);

//             if pattern_v == content {
//                 return i;
//             }
//         }
//         return -1;
//     }
// }

// struct Index {
//     lower: u64,
//     higher: u64,
// }

// pub struct SuffixTreeLeafNode<'a> {
//     char: u8,
    
//     next_index: Index,

//     child: MaybeUninit<&'a [SuffixTreeLeafNode<'a>]>,
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let index = match_index("abcd", &vec!["badc".to_string(), "qcd".to_string(), "ecd".to_string(), "cd".to_string()]);
//         assert_eq!(index, 3);
//     }
// }
