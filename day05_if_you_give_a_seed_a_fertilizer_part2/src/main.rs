#[allow(unused_imports)]
use day05_if_you_give_a_seed_a_fertilizer_common::{Almanac, Mapping, REAL_DATA, SAMPLE_DATA};
use std::cmp::Ordering;
use sorted_vec::SortedSet;

fn main() {
    let result = do_work(SAMPLE_DATA);
    println!("{}", result);
}

#[derive(Clone, Copy, Eq)]
struct Range {
    start: u64,
    range: u64,
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
    }
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
    let mut seed_ranges: SortedSet<Range> = SortedSet::new();

    if (N % 2) != 0 {
        panic!("N must be even");
    }

    for i in (0..N).step_by(2) {
        seed_ranges.push(Range {
            start: data.seeds[i],
            range: data.seeds[i + 1],
        });
    }

    for seed_range in seed_ranges.into_vec() {
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

fn get_mapped_ranges<const N: usize>(mappings: &[Mapping; N], input_ranges: SortedSet<Range>) -> SortedSet<Range> {
    let mut mapped_ranges: SortedSet<Range> = SortedSet::new();
    let input_ranges = input_ranges.into_vec();
    let sorted_mappings: SortedSet<Mapping> = SortedSet::from_unsorted(mappings.to_vec());
    let sorted_mappings = sorted_mappings.into_vec();

    for input_range in input_ranges {
        let mut remaining_start = input_range.start;
        let mut remaining_length = input_range.range;
        // Find the indices of the mappings on either side of the start of input_range
        let partition_point: usize = sorted_mappings.partition_point(|&x| x.source_range_start < input_range.start);

        let mut current_mapping: Option<Mapping> = None;
        if partition_point > 0 {
            current_mapping = Some(sorted_mappings[partition_point - 1]);
        }

        let mut next_mapping: Option<Mapping> = None;
        if partition_point < sorted_mappings.len() {
            next_mapping = Some(sorted_mappings[partition_point]);
        }

        while remaining_length > 0 {
            // See if any of the input_range are mapped by current_range
            if let Some(current_mapping) = current_mapping {
                if current_mapping.source_range_start > remaining_start {
                    panic!("current_mapping.source_range_start > remaining_start");
                }

                let current_mapping_end = current_mapping.source_range_start + current_mapping.range_length - 1;
                if remaining_start <= current_mapping_end {
                    // At least some of our remaining items are handled by current_mapping
                    let mapped_length = current_mapping.range_length - (remaining_start - current_mapping.source_range_start);
                    if mapped_length > remaining_length {
                        // All the remaining items are mapped by current_mapping
                        let mut mapped_start = (remaining_start - current_mapping.source_range_start) + current_mapping.destination_range_start;
                        mapped_ranges.push(Range {
                            start: mapped_start,
                            range: remaining_length,
                        });
                        remaining_length = 0;
                    }
                }
            }
        }

    }

    mapped_ranges
}
