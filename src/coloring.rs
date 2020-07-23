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

    pub fn to_rgb(&self) -> [i64; 3] {
        let mut colors: [i64; 3] = [0; 3];

        for i in 0..3 {
            let value = (self.hash >> (i * 8)) & 255;
            colors[i] = value as i64;
        }

        return colors;
    }

    pub fn to_hsl(&self) -> [f64; 3] {
        let mut colors: [f64; 3] = [0.0; 3];
        let mut colors_hsl: [f64; 3] = [0.0; 3];

        for i in 0..3 {
            let value = (self.hash >> (i * 8)) & 255;
            colors[i] = (value as f64) / 255.0;
        }

        let min = colors.iter().fold(f64::INFINITY, |a, &b| a.min(b)) as f64;
        let max = colors.iter().fold(f64::MIN, |a, &b| a.max(b)) as f64;
        let delta = max - min;

        colors_hsl[2] = (max + min) / 2.0;

        if delta == 0.0 {
            colors_hsl[0] = 0.0;
            colors_hsl[1] = 0.0;
        } else {
            colors_hsl[1] = delta / (1.0 - libm::fabs(2.0 * colors_hsl[2] - 1.0));

            match max {
                x if x == colors[0] => {
                    colors_hsl[0] = 60.0 * libm::fmod((colors[1] - colors[2]) / delta, 6.0);
                    if colors[2] > colors[1] {
                        colors_hsl[0] += 360.0;
                    }
                }
                x if x == colors[1] => {
                    colors_hsl[0] = 60.0 * ((colors[2] - colors[0]) / delta + 2.0);
                }
                x if x == colors[2] => {
                    colors_hsl[1] = 60.0 * ((colors[0] - colors[1]) / delta + 4.0);
                }
                _ => {}
            }
        }

        colors_hsl[0] = libm::round(colors_hsl[0]);
        colors_hsl[1] = libm::round(colors_hsl[1] * 100.0);
        colors_hsl[2] = libm::round(colors_hsl[2] * 100.0);

        return colors_hsl;
    }

    pub fn to_cmyk(&self) -> [f64; 4] {
        let mut colors: [f64; 3] = [0.0; 3];
        let mut colors_cmyk: [f64; 4] = [0.0; 4];

        for i in 0..3 {
            let value = (self.hash >> (i * 8)) & 255;
            colors[i] = value as f64;
        }

        if colors[0] == 0.0 && colors[1] == 0.0 && colors[2] == 0.0 {
            return [0.0, 0.0, 0.0, 1.0];
        }

        colors_cmyk[0] = 1.0 - (colors[0] / 255.0);
        colors_cmyk[1] = 1.0 - (colors[1] / 255.0);
        colors_cmyk[2] = 1.0 - (colors[2] / 255.0);
        colors_cmyk[3] = [colors_cmyk[0], colors_cmyk[1], colors_cmyk[2]]
            .iter()
            .fold(f64::INFINITY, |a, &b| a.min(b)) as f64;

        colors_cmyk[0] = (colors_cmyk[0] - colors_cmyk[3]) / (1.0 - colors_cmyk[3]);
        colors_cmyk[1] = (colors_cmyk[1] - colors_cmyk[3]) / (1.0 - colors_cmyk[3]);
        colors_cmyk[2] = (colors_cmyk[2] - colors_cmyk[3]) / (1.0 - colors_cmyk[3]);

        colors_cmyk[0] = libm::round(colors_cmyk[0] * 100.0);
        colors_cmyk[1] = libm::round(colors_cmyk[1] * 100.0);
        colors_cmyk[2] = libm::round(colors_cmyk[2] * 100.0);
        colors_cmyk[3] = libm::round(colors_cmyk[2] * 100.0);

        return colors_cmyk;
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
