pub fn raindrops(n: u32) -> String {
    // todo!("what sound does Raindrop #{n} make?")
    let mut ans = String::from("");

    if n % 3 == 0 {
        ans.push_str("Pling");
    }
    if n % 5 == 0 {
        ans.push_str("Plang");
    }
    if n % 7 == 0 {
        ans.push_str("Plong");
    }
    if ans.as_str() == "" {
        ans = n.to_string();
    }

    ans
}
