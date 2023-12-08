#[allow(unused_imports)]
use day06_wait_for_it_common::{RaceRecords, REAL_DATA_PART2, SAMPLE_DATA_PART2};

fn main() {
    let result = do_work(REAL_DATA_PART2);
    println!("{}", result);
}

fn do_work<const N: usize>(data: RaceRecords<N>) -> u64 {
    let mut result = 1;

    for i in 0..N {
        let time = data.times[i];
        let distance = data.distances[i];
        result *= count_winning_options(time, distance);
    }

    result
}

fn count_winning_options(time: u64, distance_to_beat: u64) -> u64 {
    let mut result = 0;

    for hold_time in 0..time {
        let distance_for_hold_time = get_distance_for_hold_time(time, hold_time);
        if distance_for_hold_time > distance_to_beat {
            result += 1;
        }
    }

    result
}

fn get_distance_for_hold_time(total_time: u64, hold_time: u64) -> u64 {
    if hold_time > total_time {
        panic!("hold_time must be less than or equal to total_time")
    }

    let running_time = total_time - hold_time;

    running_time * hold_time
}
