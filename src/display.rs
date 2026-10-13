use colored::Colorize;

#[derive(Debug, Clone, Copy)]
pub struct Display {
    pub height: u8,
    pub width: u8,
}

fn convert_to_bin(number: u8, word_size: u8) -> Vec<u8> {
    let mut result: Vec<u8> = [0].repeat(word_size as usize);
    let mut n = number as u32;
    for i in (0..word_size).rev() {
        let q: u32 = 2_u32.pow(i as u32);
        if n >= q {
            n -= q;
            result[i as usize] = 1;
        }
    }
    result.reverse();
    result
}

fn print_row(row: Vec<&str>) {
    for c in row {
        if c == "*" {
            print!("{}", c.cyan());
        } else {
            print!("{}", c);
        }
    }
    println!(" ");
}

impl Display {
    pub fn new(height: u8, width: u8) -> Self {
        Display { height, width }
    }

    pub fn render(&self, sprite: Vec<u8>) {
        let rows_left = self.height - sprite.len() as u8;
        for row in sprite {
            let bin_word = convert_to_bin(row, self.width);
            let word: Vec<&str> = bin_word
                .iter()
                .map(|x| match x {
                    1 => "*",
                    _ => " ",
                })
                .collect();
            print_row(word);
        }
        for _ in 0..rows_left {
            print_row([" "].repeat(self.width as usize));
        }
    }

    pub fn print_hexa(&self, n: &str) {
        let res = match n {
            "0" => [0xF0, 0x90, 0x90, 0x90, 0xF0],
            "1" => [0x20, 0x60, 0x20, 0x20, 0x70],
            "2" => [0xF0, 0x10, 0xF0, 0x80, 0xF0],
            "3" => [0xF0, 0x10, 0xF0, 0x10, 0xF0],
            "4" => [0x90, 0x90, 0xF0, 0x10, 0x10],
            "5" => [0xF0, 0x80, 0xF0, 0x10, 0xF0],
            "6" => [0xF0, 0x80, 0xF0, 0x90, 0xF0],
            "7" => [0xF0, 0x10, 0x20, 0x40, 0x40],
            "8" => [0xF0, 0x90, 0xF0, 0x90, 0xF0],
            "9" => [0xF0, 0x90, 0xF0, 0x10, 0xF0],
            "A" => [0xF0, 0x90, 0xF0, 0x90, 0x90],
            "B" => [0xE0, 0x90, 0xE0, 0x90, 0xE0],
            "C" => [0xF0, 0x80, 0x80, 0x80, 0xF0],
            "D" => [0xE0, 0x90, 0x90, 0x90, 0xE0],
            "E" => [0xF0, 0x80, 0xF0, 0x80, 0xF0],
            "F" => [0xF0, 0x80, 0xF0, 0x80, 0x80],
            _ => [0x00, 0x00, 0x00, 0x00, 0x00],
        };
        self.render(res.to_vec());
    }
}

mod tests {
    #[test]
    fn test_convert_to_bin_2() {
        let result: Vec<u8> = Vec::from([0, 0, 1, 0, 0, 0, 0, 0]);
        let converted = super::convert_to_bin(0x20, 8);
        assert_eq!(result, converted);
    }

    #[test]
    fn test_convert_to_bin_4_size_4() {
        let result: Vec<u8> = Vec::from([0, 1, 1, 0, 0, 0, 0, 0]);
        let converted = super::convert_to_bin(0x60, 8);
        assert_eq!(result, converted);
    }

    #[test]
    fn test_convert_to_bin_zero() {
        let result: Vec<u8> = Vec::from([0, 0, 0]);
        let converted = super::convert_to_bin(0, 3);
        assert_eq!(result, converted);
    }
}
