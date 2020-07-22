pub struct Coloring {
    value: String,
    hash: u64,
}

impl Coloring {
    pub fn new(value: &str) -> Coloring {
        Coloring {
            value: String::from(value),
            hash: Coloring::read_string(&String::from(value)),
        }
    }

    pub fn get_value(self) -> String {
        return self.value;
    }

    pub fn to_hexadecimal(&self) -> String {
        let mut color = String::from("#");

        for i in 0..3 {
            let value = (self.hash >> (i * 8)) & 255;
            color.push_str(&format!("{:0>2}", format!("{:x}", value)));
        }

        return color;
    }

    pub fn to_rgb(&self) -> [u64; 3] {
        let mut colors: [u64; 3] = [0; 3];

        for i in 0..2 {
            let value = (self.hash >> (i * 8)) & 255;
            colors[i] = value;
        }

        return colors;
    }

    fn read_string(string: &String) -> u64 {
        let mut hash: u64 = 0;
        let mut increase = true;

        for char_current in string.to_owned().as_bytes() {
            if hash > (u64::MAX / 2) {
                increase = false
            }

            if !increase && (hash < (u64::MAX / 10)) {
                increase = true
            }

            if increase && (hash << 2) > hash {
                hash = (*char_current as u64) + ((hash << 2) - hash);
            } else {
                hash = (*char_current as u64) + (hash - (hash >> 2))
            }
        }

        return hash;
    }
}
