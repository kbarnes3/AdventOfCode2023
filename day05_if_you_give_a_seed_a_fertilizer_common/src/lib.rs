pub struct Mapping {
    pub destination_range_start: u64,
    pub source_range_start: u64,
    pub range_length: u64,
}

pub struct Almanac<
    const N: usize,
    const S: usize,
    const F: usize,
    const W: usize,
    const L: usize,
    const T: usize,
    const H: usize,
    const M: usize,
> {
    pub seeds: [u64; N],
    pub seed_to_soil: [Mapping; S],
    pub soil_to_fertilizer: [Mapping; F],
    pub fertilizer_to_water: [Mapping; W],
    pub water_to_light: [Mapping; L],
    pub light_to_temperature: [Mapping; T],
    pub temperature_to_humidity: [Mapping; H],
    pub humidity_to_location: [Mapping; M],
}

pub const SAMPLE_DATA: Almanac<4, 2, 3, 4, 2, 3, 2, 2> = Almanac {
    seeds: [79, 14, 55, 13],
    seed_to_soil: [
        {
            Mapping {
                destination_range_start: 50,
                source_range_start: 98,
                range_length: 2,
            }
        },
        {
            Mapping {
                destination_range_start: 52,
                source_range_start: 50,
                range_length: 48,
            }
        },
    ],
    soil_to_fertilizer: [
        {
            Mapping {
                destination_range_start: 0,
                source_range_start: 15,
                range_length: 37,
            }
        },
        {
            Mapping {
                destination_range_start: 37,
                source_range_start: 52,
                range_length: 2,
            }
        },
        {
            Mapping {
                destination_range_start: 39,
                source_range_start: 0,
                range_length: 15,
            }
        },
    ],
    fertilizer_to_water: [
        {
            Mapping {
                destination_range_start: 49,
                source_range_start: 53,
                range_length: 8,
            }
        },
        {
            Mapping {
                destination_range_start: 0,
                source_range_start: 11,
                range_length: 42,
            }
        },
        {
            Mapping {
                destination_range_start: 42,
                source_range_start: 0,
                range_length: 7,
            }
        },
        {
            Mapping {
                destination_range_start: 57,
                source_range_start: 7,
                range_length: 4,
            }
        },
    ],
    water_to_light: [
        {
            Mapping {
                destination_range_start: 88,
                source_range_start: 18,
                range_length: 7,
            }
        },
        {
            Mapping {
                destination_range_start: 18,
                source_range_start: 25,
                range_length: 70,
            }
        },
    ],
    light_to_temperature: [
        {
            Mapping {
                destination_range_start: 45,
                source_range_start: 77,
                range_length: 23,
            }
        },
        {
            Mapping {
                destination_range_start: 81,
                source_range_start: 45,
                range_length: 19,
            }
        },
        {
            Mapping {
                destination_range_start: 68,
                source_range_start: 64,
                range_length: 13,
            }
        },
    ],
    temperature_to_humidity: [
        {
            Mapping {
                destination_range_start: 0,
                source_range_start: 69,
                range_length: 1,
            }
        },
        {
            Mapping {
                destination_range_start: 1,
                source_range_start: 0,
                range_length: 69,
            }
        },
    ],
    humidity_to_location: [
        {
            Mapping {
                destination_range_start: 60,
                source_range_start: 56,
                range_length: 37,
            }
        },
        {
            Mapping {
                destination_range_start: 56,
                source_range_start: 93,
                range_length: 4,
            }
        },
    ],
};
