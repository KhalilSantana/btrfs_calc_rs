#[derive(Debug)]
pub(crate) struct Drive {
    pub capacity: usize,
    pub free: usize,
}

impl Drive {
    pub fn new(capacity: usize) -> Self {
        Drive { capacity, free: capacity }
    }
}