use std::collections::HashMap;

#[allow(unused_imports)]
use day05_if_you_give_a_seed_a_fertilizer_common::{Almanac, Mapping, SAMPLE_DATA};

fn main() {
    let result = do_work(SAMPLE_DATA);
    println!("{}", result);
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
    let mappings = build_mapping_ranges(&data);
    let mut closest_location: Option<u64> = None;

    for seed in data.seeds {
        let soil = mappings.seed_to_soil.get(&seed).unwrap_or(&seed);
        let fertilizer = mappings.soil_to_fertilizer.get(soil).unwrap_or(soil);
        let water = mappings.fertilizer_to_water.get(fertilizer).unwrap_or(fertilizer);
        let light = mappings.water_to_light.get(water).unwrap_or(water);
        let temperature = mappings.light_to_temperature.get(light).unwrap_or(light);
        let humidity = mappings.temperature_to_humidity.get(temperature).unwrap_or(temperature);
        let location = mappings.humidity_to_location.get(humidity).unwrap_or(humidity);

        match closest_location {
            None => closest_location = Some(*location),
            Some(current) => {
                if *location < current {
                    closest_location = Some(*location);
                }
            }
        }
    }

    closest_location.unwrap()
}

struct MappingRanges {
    seed_to_soil: HashMap<u64, u64>,
    soil_to_fertilizer: HashMap<u64, u64>,
    fertilizer_to_water: HashMap<u64, u64>,
    water_to_light: HashMap<u64, u64>,
    light_to_temperature: HashMap<u64, u64>,
    temperature_to_humidity: HashMap<u64, u64>,
    humidity_to_location: HashMap<u64, u64>,
}

fn build_mapping_ranges<
    const N: usize,
    const S: usize,
    const F: usize,
    const W: usize,
    const L: usize,
    const T: usize,
    const H: usize,
    const M: usize,
>(
    data: &Almanac<N, S, F, W, L, T, H, M>
) -> MappingRanges {
    let seed_to_soil = build_mapping_range(&data.seed_to_soil);
    let soil_to_fertilizer = build_mapping_range(&data.soil_to_fertilizer);
    let fertilizer_to_water = build_mapping_range(&data.fertilizer_to_water);
    let water_to_light = build_mapping_range(&data.water_to_light);
    let light_to_temperature = build_mapping_range(&data.light_to_temperature);
    let temperature_to_humidity = build_mapping_range(&data.temperature_to_humidity);
    let humidity_to_location = build_mapping_range(&data.humidity_to_location);

    MappingRanges { seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location }
}

fn build_mapping_range<const N: usize>(data: &[Mapping; N]) -> HashMap<u64, u64> {
    let mut result = HashMap::new();
    for mapping in data {
        for i in 0..mapping.range_length {
            result.insert(mapping.source_range_start + i, mapping.destination_range_start + i);
        }
    }

    result
}
