use std::collections::HashMap;

#[derive(Debug)]
pub struct Bill {
    pub name: Option<String>,
    pub amount: f32,
}

pub struct Bills {
    inner: HashMap<String, Bill>,
}

impl Bills {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn add_bills(&mut self, bill: Bill) -> &HashMap<String, Bill> {
        let name = match &bill.name {
            Some(name) => name,
            None => "unknown",
        };

        self.inner.insert(name.to_owned(), bill);

        &self.inner
    }

    pub fn get_all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }

    pub fn remove(&mut self, key: &str) -> bool {
        self.inner.remove(key).is_some()
    }

    pub fn update(&mut self, name: &str, amount: f32) -> bool {
        match self.inner.get_mut(name) {
            Some(bill) => {
                bill.amount = amount;
                true
            }
            None => false,
        }
    }
}

impl Bill {
    pub fn new(name: Option<String>, amount: f32) -> Self {
        Bill { name, amount }
    }
}
