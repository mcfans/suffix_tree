use suffix_tree::check_contains2;

fn main() -> Result<(), &'static str> {
    let mut args = std::env::args();
    let pattern = args.nth(1).ok_or("123")?;
    let text = args.nth(0).ok_or("234")?;

    let mut res = 0;
    for _ in 0..100000000 {
        res = check_contains2(&pattern, &text);
    }
    println!("{}", res);
    Ok(())
}