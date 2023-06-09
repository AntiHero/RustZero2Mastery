#[derive(Debug)]
pub struct Bill {
    pub name: Option<String>,
    pub amount: f32,
}

impl Bill {
    pub fn new(name: Option<String>, amount: f32) -> Self {
        Bill { name, amount }
    }
}
