#[derive(Debug)]
pub struct Bill {
    pub name: Option<String>,
    pub amount: f32,
}

pub struct Bills {
    inner: Vec<Bill>,
}

impl Bills {
    pub fn new() -> Self {
        Self { inner: vec![] }
    }

    pub fn add_bills(&mut self, bill: Bill) -> &Vec<Bill> {
        self.inner.push(bill);

        &self.inner
    }

    pub fn get_all(&self) -> Vec<&Bill> {
        self.inner.iter().collect()
    }
}

impl Bill {
    pub fn new(name: Option<String>, amount: f32) -> Self {
        Bill { name, amount }
    }
}
