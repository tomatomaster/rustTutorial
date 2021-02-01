//first irst-fay
//apple apple-hay
fn main() {
    let s = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s = s.trim_end().to_owned();
        s
    };

    let boin = vec!["a", "i", "u", "e", "o"];
    if boin.iter().any(|&i| i == &s[0..1]) {
        print!("{}-hay", s);
    } else {
        print!("{}-{}ay", &s[1..s.len()], &s[0..1]);
    }
}
