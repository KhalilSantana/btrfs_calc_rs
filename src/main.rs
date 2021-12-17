#![allow(dead_code)]

use crate::drive::Drive;
use crate::profiles::BtrfsProfile;

mod drive;
mod profiles;

#[derive(Debug)]
struct CalcStats<'a> {
    profile: &'a BtrfsProfile,
    raw_capacity: usize,
    usable_capacity: usize,
}

fn calc<'a>(profile: &'a BtrfsProfile, drives: &mut [Drive]) -> CalcStats<'a> {
    let mut stats = CalcStats {
        profile,
        raw_capacity: 0,
        usable_capacity: 0,
    };
    // Ensure the selected profile can be computed for `drives.len()` number of devices.
    // TODO: Use a less naïve check to handle other cases with parity, etc
    if drives.len() < profile.configuration().number_of_copies {
        panic!("Not enough drives...")
    }
    stats.raw_capacity = drives.iter().map(|d| d.capacity).sum();
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
            while drives
                .get(profile.configuration().number_of_copies - 1)
                .unwrap()
                .free
                > 0
            {
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
