#![allow(dead_code)]

#[derive(Debug, Clone)]
struct ProfileCfg {
    number_of_copies: usize,
    parity: u8,
}

#[derive(Debug, Clone)]
enum BtrfsProfile {
    Single,
    Dup,
    Raid0,
    Raid1,
    Raid1c3,
    Raid1c4,
    Raid10,
    Raid5,
    Raid6,
    Unknown(ProfileCfg),
}

impl ProfileCfg {
    fn new(number_of_copies: usize, parity: u8) -> Self {
        ProfileCfg { number_of_copies, parity }
    }
}

impl BtrfsProfile {
    fn configuration(&self) -> ProfileCfg {
        match self {
            Self::Single => ProfileCfg::new(1, 0),
            Self::Dup => ProfileCfg::new(2, 0),
            Self::Raid1 => ProfileCfg::new(2, 0),
            Self::Raid1c3 => ProfileCfg::new(3, 0),
            Self::Raid1c4 => ProfileCfg::new(4, 0),
            //TODO: Add stripe for these two
            Self::Raid0 => ProfileCfg::new(1, 0),
            Self::Raid10 => ProfileCfg::new(2, 0),
            // TODO: Add parity handling for these two
            Self::Raid5 => ProfileCfg::new(1, 2),
            Self::Raid6 => ProfileCfg::new(1, 0),
            Self::Unknown(cfg) => cfg.clone()
        }
    }
}

#[derive(Debug, Clone)]
struct Drive {
    capacity: usize,
    free: usize,
}

impl Drive {
    fn new(capacity: usize) -> Self {
        Drive { capacity, free: capacity }
    }
}

#[derive(Debug, Clone)]
struct CalcStats {
    profile: BtrfsProfile,
    raw_capacity: usize,
    usable_capacity: usize,
}

fn calc(profile: &BtrfsProfile, drives: &mut [Drive]) -> CalcStats {
    let mut stats = CalcStats { profile: profile.clone(), raw_capacity: 0, usable_capacity: 0 };
    // Ensure the selected profile can be computed for `drives.len()` number of devices.
    // TODO: Use a less naïve check to handle other cases with parity, etc
    if drives.len() < profile.configuration().number_of_copies {
        panic!("Not enough drives...")
    }
    for drive in drives.as_ref() {
        stats.raw_capacity += drive.capacity;
    }
    drives.sort_by(|a, b| b.free.partial_cmp(&a.free).unwrap());
    match profile {
        BtrfsProfile::Single => {
            stats.usable_capacity = stats.raw_capacity;
        }
        BtrfsProfile::Dup => {
            stats.usable_capacity = stats.raw_capacity / profile.configuration().number_of_copies;
        }
        BtrfsProfile::Raid5 => {
            todo!("Implement raid5")
        }
        BtrfsProfile::Raid6 => {
            todo!("Implement raid6")
        }
        _ => {
            // TODO: Move this stuff so it only handles mirroring and not other profiles
            // Safety: We already check if the `drives` array has enough elements at the start of this fn
            // and this fn doesn't add or remove items to the `drives` array, as such, there's no need to check here again
            unsafe {
                // While the drive with the least amount of space still has _some_ space left..
                // Note: `number_of_copies` is always a value >= 1, as such we need to offset this here
                while drives.get_unchecked(profile.configuration().number_of_copies - 1).free > 0 {
                    for i in 0..profile.configuration().number_of_copies {
                        drives.get_unchecked_mut(i).free -= 1;
                    }
                    stats.usable_capacity += 1;
                    drives.sort_by(|a, b| b.free.partial_cmp(&a.free).unwrap());
                }
            }
        }
    }
    println!("Drives: {:?}", drives);
    stats
}

fn main() {
    let mut drives: Vec<Drive> = vec![];
    drives.push(Drive::new(1000));
    drives.push(Drive::new(1000));
    drives.push(Drive::new(500));
    drives.push(Drive::new(250));
    let stats = calc(&BtrfsProfile::Raid1c3, &mut drives);
    println!("{:?}", stats);
}
