pub struct RaceRecords<const N: usize> {
    pub times: [u64; N],
    pub distances: [u64; N],
}

pub const SAMPLE_DATA: RaceRecords<3> = RaceRecords {
    times: [7, 15, 30],
    distances: [9, 40, 200],
};
