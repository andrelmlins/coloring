pub struct Coloring {
    value: String,
}

impl Coloring {
    pub fn new(value: &str) -> Coloring {
        Coloring {
            value: String::from(value),
        }
    }

    pub fn to_hexadecimal(&self) -> String {
        let hash = Coloring::read_string(&self.value);
        let mut color = String::from("#");

        for i in 0..3 {
            let value = (hash >> (i * 8)) & 255;
            color.push_str(&format!("{:0>2}", format!("{:x}", value)));
        }

        return color;
    }

    pub fn to_rgb(&self) -> [usize; 3] {
        let hash = Coloring::read_string(&self.value);
        let mut colors: [usize; 3] = [0; 3];

        for i in 0..2 {
            let value = (hash >> (i * 8)) & 255;
            colors[i] = value;
        }

        return colors;
    }

    fn read_string(string: &String) -> usize {
        let mut hash: usize = 0;
        let chars = string
            .to_owned()
            .chars()
            .filter(|s| !"./-".contains(s.to_owned()))
            .collect::<Vec<_>>();

        for char_current in chars {
            hash = (char_current as usize) + ((hash << 5) - hash);
            hash = hash & hash;
        }

        return hash;
    }
}
