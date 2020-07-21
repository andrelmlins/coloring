fn main() {
    println!("{}", string_color("Paulo"));
}

fn string_color(string: &str) -> String {
    let mut hash: u64 = 0;
    let chars = string
        .chars()
        .filter(|s| !"./-".contains(s.to_owned()))
        .collect::<Vec<_>>();

    if chars.len() == 0 {
        return String::from("#000000");
    }
    for char_current in chars {
        hash = (char_current as u64) + ((hash << 5) - hash);
        hash = hash & hash;
    }
    println!("HASH {}", hash);

    let mut color = String::from("#");
    for i in 0..3 {
        let value = (hash >> (i * 8)) & 255;
        color.push_str(&format!("{:x}", value));
    }
    return color;
}
