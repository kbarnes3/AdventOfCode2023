#[allow(unused_imports)]
use day05_if_you_give_a_seed_a_fertilizer_common::{Almanac, Mapping, REAL_DATA, SAMPLE_DATA};
use sorted_vec::SortedSet;
use std::cmp::Ordering;

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

    let soil_ranges = get_mapped_ranges(&data.seed_to_soil, seed_ranges);
    let fertilizer_ranges = get_mapped_ranges(&data.soil_to_fertilizer, soil_ranges);
    let water_ranges = get_mapped_ranges(&data.fertilizer_to_water, fertilizer_ranges);
    let light_ranges = get_mapped_ranges(&data.water_to_light, water_ranges);
    let temperature_ranges = get_mapped_ranges(&data.light_to_temperature, light_ranges);
    let humidity_ranges = get_mapped_ranges(&data.temperature_to_humidity, temperature_ranges);
    let location_ranges = get_mapped_ranges(&data.humidity_to_location, humidity_ranges);

    if location_ranges.is_empty() {
        panic!("No locations found");
    }

    location_ranges[0].start
}

fn get_mapped_ranges<const N: usize>(
    mappings: &[Mapping; N],
    input_ranges: SortedSet<Range>,
) -> SortedSet<Range> {
    let mut mapped_ranges: SortedSet<Range> = SortedSet::new();
    let input_ranges = input_ranges.into_vec();
    let sorted_mappings: SortedSet<Mapping> = SortedSet::from_unsorted(mappings.to_vec());
    let sorted_mappings = sorted_mappings.into_vec();

    for input_range in input_ranges {
        let mut remaining_start = input_range.start;
        let mut remaining_length = input_range.range;
        // Find the indices of the mappings on either side of the start of input_range
        let mut partition_point: usize =
            sorted_mappings.partition_point(|&x| x.source_range_start < input_range.start);

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
            if let Some(current_mapping_value) = current_mapping {
                if current_mapping_value.source_range_start > remaining_start {
                    panic!("current_mapping.source_range_start > remaining_start");
                }

                let current_mapping_end = current_mapping_value.source_range_start
                    + current_mapping_value.range_length
                    - 1;
                if remaining_start <= current_mapping_end {
                    // At least some of our remaining items are handled by current_mapping
                    let mapped_length = current_mapping_value.range_length
                        - (remaining_start - current_mapping_value.source_range_start);
                    if mapped_length > remaining_length {
                        // All the remaining items are mapped by current_mapping
                        let mapped_start = (remaining_start
                            - current_mapping_value.source_range_start)
                            + current_mapping_value.destination_range_start;
                        mapped_ranges.push(Range {
                            start: mapped_start,
                            range: remaining_length,
                        });
                        remaining_length = 0;
                    } else {
                        // Only some of our remaining items are handled by current_mapping
                        let mapped_length = current_mapping_end - remaining_start + 1;
                        let mapped_start = (remaining_start
                            - current_mapping_value.source_range_start)
                            + current_mapping_value.destination_range_start;

                        mapped_ranges.push(Range {
                            start: mapped_start,
                            range: mapped_length,
                        });

                        remaining_start += mapped_length;
                        remaining_length -= mapped_length;
                    }
                } else {
                    // Clear out current_mapping because none of the remaining items are handled by it
                    current_mapping = None;
                }
            } else {
                // Some of the remaining items may not handled by a mapping
                if let Some(next_mapping_value) = next_mapping {
                    let unmapped_range_length =
                        next_mapping_value.source_range_start - remaining_start;
                    if unmapped_range_length > remaining_length {
                        // All the remaining items are unmapped
                        mapped_ranges.push(Range {
                            start: remaining_start,
                            range: remaining_length,
                        });
                        remaining_length = 0;
                    } else {
                        // Only some of the remaining items are unmapped
                        mapped_ranges.push(Range {
                            start: remaining_start,
                            range: unmapped_range_length,
                        });
                        remaining_start += unmapped_range_length;
                        remaining_length -= unmapped_range_length;
                    }

                    // See if we should advance current_mapping and next_mapping
                    if remaining_start >= next_mapping_value.source_range_start {
                        current_mapping = next_mapping;
                        partition_point += 1;
                        if partition_point < sorted_mappings.len() {
                            next_mapping = Some(sorted_mappings[partition_point]);
                        } else {
                            next_mapping = None;
                        }
                    }
                } else {
                    // All mappings are below the remaining items, pass through the remaining range unmapped
                    mapped_ranges.push(Range {
                        start: remaining_start,
                        range: remaining_length,
                    });
                    remaining_length = 0;
                }
            }
        }
    }

    mapped_ranges
}
