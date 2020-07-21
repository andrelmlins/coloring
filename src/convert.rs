pub fn convert_hexadecimal(string: &str) -> String {
    let hash = read_string(string);
    let mut color = String::from("#");

    for i in 0..3 {
        let value = (hash >> (i * 8)) & 255;
        color.push_str(&format!("{:0>2}", format!("{:x}", value)));
    }

    return color;
}

pub fn convert_rgb(string: &str) -> [usize; 3] {
    let hash = read_string(string);
    let mut colors: [usize; 3] = [0; 3];

    for i in 0..2 {
        let value = (hash >> (i * 8)) & 255;
        colors[i] = value;
    }

    return colors;
}

fn read_string(string: &str) -> usize {
    let mut hash: usize = 0;
    let chars = string
        .chars()
        .filter(|s| !"./-".contains(s.to_owned()))
        .collect::<Vec<_>>();

    for char_current in chars {
        hash = (char_current as usize) + ((hash << 5) - hash);
        hash = hash & hash;
    }

    return hash;
}
