#![allow(unused_variables, dead_code)]

use crate::BtrfsProfile::{Dup, Raid1, Raid1c3, Raid1c4, Single};

#[derive(Debug, Clone)]
struct ProfileCfg {
    btrfs_profile: BtrfsProfile,
    number_of_copies: usize,
    parity: u8,
}

#[non_exhaustive]
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
}

impl ProfileCfg {
    fn new(btrfs_profile: BtrfsProfile, number_of_copies: usize, parity: u8) -> Self {
        ProfileCfg {btrfs_profile,number_of_copies, parity }
    }
}

fn get_std_profiles() -> [ProfileCfg; 4] {
    let std_profiles: [ProfileCfg; 4] = [
        ProfileCfg::new(Single, 1, 0),
        ProfileCfg::new(Raid1, 2, 0),
        ProfileCfg::new(Raid1c3, 3, 0),
        ProfileCfg::new(Raid1c4, 4, 0),
    ];
    std_profiles
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
    profile: ProfileCfg,
    raw_capacity: usize,
    usable_capacity: usize,
}

fn calc(profile: &ProfileCfg, drives: &mut [Drive]) -> CalcStats {
    let mut stats = CalcStats { profile: profile.clone(), raw_capacity: 0, usable_capacity: 0 };
    // Ensure the selected profile can be computed for `drives.len()` number of devices.
    if drives.len() < profile.number_of_copies {
        panic!("Not enough drives...")
    }
    for drive in drives.as_ref() {
        stats.raw_capacity += drive.capacity;
    }
    drives.sort_by(|a, b| b.free.partial_cmp(&a.free).unwrap());
    match profile.btrfs_profile {
        Single => {
            stats.usable_capacity = stats.raw_capacity;
        }
        Dup => {
            stats.usable_capacity = stats.raw_capacity / 2;
        }
        // TODO: Implement other profiles
        _ => {
            // TODO: Actually handle
            // Safety: We already check if the `drives` array has enough elements at the start of this fn
            // and this fn doesn't add or remove items to the `drives` array, as such, there's no need to check here again
            unsafe {
                // While the drive with the least amount of space still has _some_ space left..
                // Note: `number_of_copies` is always a value >= 1, as such we need to offset this here
                while drives.get_unchecked(profile.number_of_copies - 1).free > 0 {
                    for i in 0..profile.number_of_copies {
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
    println!("All Profiles");
    for i in get_std_profiles() {
        println!("{:?}", i);
    }
    let mut drives: Vec<Drive> = vec![];
    drives.push(Drive::new(1000));
    drives.push(Drive::new(1000));
    drives.push(Drive::new(500));
    drives.push(Drive::new(250));
    let stats = calc(&get_std_profiles()[3], &mut drives);
    println!("{:?}", stats);
}
