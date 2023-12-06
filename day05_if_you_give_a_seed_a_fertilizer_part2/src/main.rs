#[allow(unused_imports)]
use day05_if_you_give_a_seed_a_fertilizer_common::{Almanac, Mapping, REAL_DATA, SAMPLE_DATA};
use std::vec::Vec;

fn main() {
    let result = do_work(SAMPLE_DATA);
    println!("{}", result);
}

struct SeedRange {
    start: u64,
    range: u64,
}

fn do_work<
    const N: usize,
    const S: usize,
    const F: usize,
    const W: usize,
    const L: usize,
    const T: usize,
    const H: usize,
    const M: usize,
>(
    data: Almanac<N, S, F, W, L, T, H, M>,
) -> u64 {
    let mut closest_location: Option<u64> = None;
    let mut seed_ranges: Vec<SeedRange> = Vec::new();

    if (N % 2) != 0 {
        panic!("N must be even");
    }

    for i in (0..N).step_by(2) {
        seed_ranges.push(SeedRange {
            start: data.seeds[i],
            range: data.seeds[i + 1],
        });
    }

    for seed_range in seed_ranges {
        for seed in seed_range.start..seed_range.start + seed_range.range {
            let soil = get_mapped_value(&data.seed_to_soil, seed);
            let fertilizer = get_mapped_value(&data.soil_to_fertilizer, soil);
            let water = get_mapped_value(&data.fertilizer_to_water, fertilizer);
            let light = get_mapped_value(&data.water_to_light, water);
            let temperature = get_mapped_value(&data.light_to_temperature, light);
            let humidity = get_mapped_value(&data.temperature_to_humidity, temperature);
            let location = get_mapped_value(&data.humidity_to_location, humidity);

            match closest_location {
                None => closest_location = Some(location),
                Some(current) => {
                    if location < current {
                        closest_location = Some(location);
                    }
                }
            }
        }
    }

    closest_location.unwrap()
}

fn get_mapped_value<const N: usize>(mappings: &[Mapping; N], value: u64) -> u64 {
    for mapping in mappings {
        if value >= mapping.source_range_start
            && value < mapping.source_range_start + mapping.range_length
        {
            return mapping.destination_range_start + (value - mapping.source_range_start);
        }
    }

    value
}
