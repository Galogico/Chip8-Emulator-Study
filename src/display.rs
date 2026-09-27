pub struct Display {
    pub height: u8,
    pub width: u8,
}
impl Display {
    pub fn new(height: u8, width: u8) -> Self {
        Display { height, width }
    }
    pub fn render(self, sprite: Vec<u8>) {
        for row in sprite {
            let bits: String = format!("{row:b}");
            let mut word: Vec<String> = [" "]
                .repeat(self.width.into())
                .into_iter()
                .map(|w| w.to_string())
                .collect();
            for (i, c) in bits.chars().enumerate() {
                let v = match c {
                    '1' => "#",
                    _ => " ",
                };
                word[i] = v.to_string();
            }

            println!("{:?}", word);
        }
    }
}
