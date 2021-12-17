#[derive(Debug, Clone)]
pub struct ProfileCfg {
    pub number_of_copies: usize,
    pub stripe_min: usize,
    pub stripe_max: usize,
    pub parity: usize,
}

#[derive(Debug)]
pub enum BtrfsProfile {
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
    pub fn new(number_of_copies: usize, stripe_min: usize, stripe_max: usize, parity: usize) -> Self {
        ProfileCfg { number_of_copies, stripe_min, stripe_max, parity }
    }
}

impl BtrfsProfile {
    pub fn configuration(&self) -> ProfileCfg {
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