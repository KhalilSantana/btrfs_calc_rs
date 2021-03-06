#![allow(dead_code)]

use crate::drive::*;
use crate::profiles::BtrfsProfile;
use tabled::Table;

mod drive;
mod profiles;

#[derive(Debug)]
struct CalcStats<'a> {
    profile: &'a BtrfsProfile,
    raw_capacity: usize,
    usable_capacity: usize,
    unusable_space: usize,
}

fn calc<'a>(profile: &'a BtrfsProfile, drives: &mut [Drive]) -> CalcStats<'a> {
    let mut stats = CalcStats {
        profile,
        raw_capacity: 0,
        usable_capacity: 0,
        unusable_space: 0,
    };
    // Ensure the selected profile can be computed for `drives.len()` number of devices.
    // TODO: Use a less naïve check to handle other cases with parity, etc
    if drives.len() < profile.configuration().number_of_copies {
        panic!("Not enough drives...")
    }
    stats.raw_capacity = drives.iter().map(|d| d.get_capacity()).sum();
    drive::sort_drives_by_free_space_decreasing(drives);
    match profile {
        BtrfsProfile::Single => {
            stats.usable_capacity = stats.raw_capacity;
        }
        BtrfsProfile::Dup => {
            stats.usable_capacity = stats.raw_capacity / profile.configuration().number_of_copies;
        }
        BtrfsProfile::Raid5 => loop {
            let drives_free = drives.iter().filter(|d| d.has_free_space()).count();
            if drives_free
                < (profile.configuration().parity + profile.configuration().number_of_copies)
            {
                break;
            }
            let mut acc = 0;
            for i in 0..drives_free {
                drives.get_mut(i).unwrap().dec_free();
                acc += 1;
            }
            stats.usable_capacity += acc - 1;
            drive::sort_drives_by_free_space_decreasing(drives)
        },
        BtrfsProfile::Raid6 => loop {
            let drives_free = drives.iter().filter(|d| d.has_free_space()).count();
            if drives_free
                < (profile.configuration().parity + profile.configuration().number_of_copies)
            {
                break;
            }
            let mut acc = 0;
            for i in 0..drives_free {
                drives.get_mut(i).unwrap().dec_free();
                acc += 1;
            }
            stats.usable_capacity += acc - 2;
            drive::sort_drives_by_free_space_decreasing(drives)
        },
        BtrfsProfile::Raid1 | BtrfsProfile::Raid1c3 | BtrfsProfile::Raid1c4 => {
            // Unwrap usage: We already check if the `drives` array has enough elements at the start of this fn
            // and this fn doesn't add or remove items to the `drives` array, as such, there's no need to check here again
            // While the drive with the least amount of space still has _some_ space left..
            // Note: `number_of_copies` is always a value >= 1, as such we need to offset this here
            while drives
                .get(profile.configuration().number_of_copies - 1)
                .unwrap()
                .has_free_space()
            {
                for i in 0..profile.configuration().number_of_copies {
                    drives.get_mut(i).unwrap().dec_free();
                }
                stats.usable_capacity += 1;
                drive::sort_drives_by_free_space_decreasing(drives);
            }
        }
        BtrfsProfile::Raid0 => loop {
            let drives_free = drives.iter().filter(|d| d.has_free_space()).count();
            if drives_free < 1 {
                break;
            }
            for i in 0..drives_free {
                drives.get_mut(i).unwrap().dec_free();
                stats.usable_capacity += 1;
            }
            drive::sort_drives_by_free_space_decreasing(drives);
        },
        BtrfsProfile::Raid10 => loop {
            let drives_free = drives.iter().filter(|d| d.has_free_space()).count();
            //            println!("Drives with free space: {drives_free}");
            let n = if drives_free % 2 == 0 {
                drives_free
            } else {
                drives_free - 1
            };
            let stripes = n - profile.configuration().number_of_copies;
            //            println!("Working on {n} drives this round");
            //            println!("Stripe count: {stripes}");
            //            let drive_t = Table::new(&*drives).to_string();
            //            println!("{}", drive_t);
            if n < 2 || stripes < 2 {
                break;
            }
            for d in 0..profile.configuration().number_of_copies {
                drives.get_mut(d).unwrap().dec_free()
            }
            stats.usable_capacity += 1;
            let start = profile.configuration().number_of_copies;
            let end = start + stripes;
            for s in start..end {
                drives.get_mut(s).unwrap().dec_free()
            }
            //            println!("B: Usable capacity: {}", stats.usable_capacity);
            stats.usable_capacity += stripes / 2;
            //            println!("A: Usable capacity: {}", stats.usable_capacity);
            drive::sort_drives_by_free_space_decreasing(drives)
        },

        // TODO: Handle non-standard profile configurations
        _ => {
            unimplemented!()
        }
    }
    for drive in drives {
        stats.unusable_space += drive.get_free();
    }
    stats
}

fn main() {
    let mut drives: Vec<Drive> = vec![
        Drive::new(300),
        Drive::new(200),
        Drive::new(50),
        Drive::new(1),
        Drive::new(1),
        Drive::new(1),
        Drive::new(1),
    ];
    let stats = calc(&BtrfsProfile::Raid6, &mut drives);
    let drive_t = Table::new(&drives).to_string();
    println!("{:?}", stats);
    println!("{}", drive_t);
}
