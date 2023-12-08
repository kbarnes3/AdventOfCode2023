pub struct RaceRecords<const N: usize> {
    pub times: [u64; N],
    pub distances: [u64; N],
}

pub const SAMPLE_DATA: RaceRecords<3> = RaceRecords {
    times: [7, 15, 30],
    distances: [9, 40, 200],
};

pub const REAL_DATA: RaceRecords<4> = RaceRecords {
    times: [59, 70, 78, 78],
    distances: [430, 1218, 1213, 1276],
};

pub const SAMPLE_DATA_PART2: RaceRecords<1> = RaceRecords {
    times: [71530],
    distances: [940200],
};

pub const REAL_DATA_PART2: RaceRecords<1> = RaceRecords {
    times: [59707878],
    distances: [430121812131276],
};
