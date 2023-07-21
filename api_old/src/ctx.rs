use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Ctx {
    name: u64,
}

impl Ctx {
    pub fn new(name: u64) -> Self {
        Self { name }
    }
}

impl Ctx {
    pub fn name(&self) -> u64 {
        self.name
    }
}
