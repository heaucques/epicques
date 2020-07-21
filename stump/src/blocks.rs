pub enum Blocks {
    Empty,
    Solid,
    Spike,
}

impl Blocks {
    pub fn int_to_enum(int : i64) -> Blocks {
        match int {
            0 => Blocks::Empty,
            1 => Blocks::Solid,
            2 => Blocks::Spike,
            _ => Blocks::Empty,
        }
    }
}