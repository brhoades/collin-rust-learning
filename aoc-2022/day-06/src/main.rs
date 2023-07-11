fn main() {
    println!("Hello, world!");
}

fn first_marker(s: &str) -> Option<usize> {
    let chars = s.chars().collect::<Vec<_>>();
    let idx = chars.windows(4).enumerate().find(|(i, chars)| {
        let mut chs = chars.to_vec();
        chs.sort();
        chs.windows(2).all(|v| v[0] != v[1])
    });
    idx.map(|i| i.0 + 4)
}

#[test]
fn test_first_marker() {
    assert_eq!(first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
    assert_eq!(first_marker("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
}
