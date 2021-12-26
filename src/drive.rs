use tabled::Tabled;
#[derive(Debug, Tabled)]
pub(crate) struct Drive {
    pub id: u8,
    pub capacity: usize,
    pub free: usize,
}

impl Drive {
    pub fn new(id: u8, capacity: usize) -> Self {
        Drive {
            id,
            capacity,
            free: capacity,
        }
    }
}

pub(crate) fn sort_drives_by_free_space_decreasing(drives: &mut [Drive]) {
    // Yes, this is ugly, but I couldn't find a more idiomatic way to do this
    drives.sort_by(|a, b| b.free.partial_cmp(&a.free).unwrap());
}