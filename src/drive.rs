use tabled::Tabled;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering;

static DRIVE_ID_COUNTER: AtomicU8 = AtomicU8::new(0);

#[derive(Debug, Tabled)]
pub(crate) struct Drive {
    id: u8,
    capacity: usize,
    free: usize,
}

impl Drive {
    pub fn new(capacity: usize) -> Self {
        Drive {
            id: DRIVE_ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            capacity,
            free: capacity,
        }
    }

    pub fn get_capacity(&self) -> usize {
        self.capacity.clone()
    }

    pub fn get_free(&self) -> usize {
        self.free.clone()
    }

    pub fn dec_free(&mut self) {
        self.free -= 1;
    }

    pub fn has_free_space(&self) -> bool {
        self.free > 0
    }
}

pub(crate) fn sort_drives_by_free_space_decreasing(drives: &mut [Drive]) {
    // Yes, this is ugly, but I couldn't find a more idiomatic way to do this
    drives.sort_by(|a, b| b.free.partial_cmp(&a.free).unwrap());
}