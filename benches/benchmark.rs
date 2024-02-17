use criterion::{Criterion, criterion_group, criterion_main};
use suffix_tree::tree::{Matrix, SuffixMatcher};
use rand::prelude::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut test_array = vec![];

    let mut rng = rand::thread_rng();
    for i in 0i64..1_000_00i64 {
        let rand_len: u32 = rng.gen();
        let len = rand_len % 12;
        let len = len + 4;
        let mut arr = vec![0u8; len as usize];
        for char in &mut arr {
            let random_u8: u8 = rng.gen();
            *char = random_u8 % 40 + 60;
        }
        let s = String::from_utf8(arr).unwrap();
        test_array.push((i, s));
    }

    let mut matrix = Matrix::new(&test_array);
    println!("Begin Sorting");
    matrix.sort_in_place();
    println!("Begin Tree building");
    let build_tree = matrix.build_tree();
    let matcher = SuffixMatcher::new(build_tree);
    c.bench_function("Find speed", |f| f.iter(||
        matcher.find("apple.com")
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);