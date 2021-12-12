#![allow(dead_code)]

#[derive(Debug, Clone)]
struct ProfileCfg {
    number_of_copies: usize,
    stripe_min: usize,
    stripe_max: usize,
    parity: usize,
}

#[derive(Debug)]
enum BtrfsProfile {
    /// `BtrfsProfile::Single` is the simplest of all BTRFS profiles, it will allocate one chunk at a time, and write to any drive,
    /// thus, the usable space on this profile is trivially found by adding the raw space on all drives
    Single,
    /// `BtrfsProfile::Dup` is another rather simple profile, it will allocate two identical chunks at a time, and place them on **any** device
    /// which includes having both copies on the same drive. Thus, the usable space for this profile is defined as ```RAW_SPACE/2```
    Dup,
    Raid0,
    /// `BtrfsProfile::Raid1` is identical to `BtrfsProfile::Dup`, however it will ensure both copies will be written to **different** drives.
    Raid1,
    /// Identical to `BtrfsProfile::Raid1`, but with three copies on three distinct drives.
    Raid1c3,
    /// Identical to `BtrfsProfile::Raid1`, but with four copies on four distinct drives.
    Raid1c4,
    Raid10,
    Raid5,
    Raid6,
    Unknown(ProfileCfg),
}

impl ProfileCfg {
    fn new(number_of_copies: usize, stripe_min: usize, stripe_max: usize, parity: usize) -> Self {
        ProfileCfg { number_of_copies, stripe_min, stripe_max, parity }
    }
}

impl BtrfsProfile {
    fn configuration(&self) -> ProfileCfg {
        match self {
            Self::Single => ProfileCfg::new(1, 1, 1, 0),
            Self::Dup => ProfileCfg::new(2, 1, 1, 0),
            Self::Raid1 => ProfileCfg::new(2, 1, 1, 0),
            Self::Raid1c3 => ProfileCfg::new(3, 1, 1, 0),
            Self::Raid1c4 => ProfileCfg::new(4, 1, 1, 0),
            Self::Raid0 => ProfileCfg::new(1, 2, usize::MAX, 0),
            Self::Raid10 => ProfileCfg::new(2, 2, usize::MAX, 0),
            Self::Raid5 => ProfileCfg::new(1, 1, usize::MAX, 1),
            Self::Raid6 => ProfileCfg::new(1, 1, usize::MAX, 2),
            Self::Unknown(cfg) => cfg.clone()
        }
    }
}

#[derive(Debug)]
struct Drive {
    capacity: usize,
    free: usize,
}

impl Drive {
    fn new(capacity: usize) -> Self {
        Drive { capacity, free: capacity }
    }
}

#[derive(Debug)]
struct CalcStats<'a> {
    profile: &'a BtrfsProfile,
    raw_capacity: usize,
    usable_capacity: usize,
}

fn calc<'a>(profile: &'a BtrfsProfile, drives: &mut [Drive]) -> CalcStats<'a> {
    let mut stats = CalcStats { profile, raw_capacity: 0, usable_capacity: 0 };
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
            todo!("Implement raid5 && Handle degenerate cases like 2xdrive RAID5")
        }
        BtrfsProfile::Raid6 => {
            todo!("Implement raid6 && Handle degenerate cases like 3xdrive RAID6")
        }
        BtrfsProfile::Raid1 | BtrfsProfile::Raid1c3 | BtrfsProfile::Raid1c4 => {
            // Unwrap usage: We already check if the `drives` array has enough elements at the start of this fn
            // and this fn doesn't add or remove items to the `drives` array, as such, there's no need to check here again
            // While the drive with the least amount of space still has _some_ space left..
            // Note: `number_of_copies` is always a value >= 1, as such we need to offset this here
            while drives.get(profile.configuration().number_of_copies - 1).unwrap().free > 0 {
                for i in 0..profile.configuration().number_of_copies {
                    drives.get_mut(i).unwrap().free -= 1;
                }
                stats.usable_capacity += 1;
                drives.sort_by(|a, b| b.free.partial_cmp(&a.free).unwrap());
            }
        }
        // TODO: Handle non-standard profile configurations
        _ => {
            unimplemented!()
        }
    }
    println!("Drives: {:?}", drives);
    stats
}

fn main() {
    let mut drives: Vec<Drive> = vec![
        Drive::new(1000),
        Drive::new(1000),
        Drive::new(500),
        Drive::new(250),
    ];
    let stats = calc(&BtrfsProfile::Raid1c4, &mut drives);
    println!("{:?}", stats);
}
