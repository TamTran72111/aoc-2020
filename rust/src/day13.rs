use crate::utilities::read_lines;

fn part_1(earliest_depart: u64, buses: &Vec<(u64, u64)>) {
    let mut min_wait = 10_000_000;
    let mut bus_id = 0;
    for bus in buses {
        if earliest_depart % bus.0 == 0 {
            bus_id = bus.0;
            min_wait = 0;
            break;
        } else {
            let depart = (earliest_depart / bus.0 + 1) * bus.0;
            let wait = depart - earliest_depart;
            if wait < min_wait {
                min_wait = wait;
                bus_id = bus.0;
            }
        }
    }
    println!("\tPart 1: {}", min_wait * bus_id);
}

fn part_2(buses: &Vec<(u64, u64)>) {
    let product = buses.iter().fold(1, |acc, bus| acc * bus.0);
    let mut result = 0;

    for &(bus_id, wait) in buses {
        let b = (bus_id - wait % bus_id) % bus_id;
        let n = product / bus_id;
        let mut xi = n;
        let mut x = 1;
        while xi % bus_id != 1 {
            xi += n;
            x += 1;
        }
        result += b * n * x;
    }

    result %= product;

    println!("\tPart 2: {}", result);
}

pub fn main() {
    println!("Day 13");
    let data = read_lines("../inputs/day13.txt");
    let earliest_depart: u64 = data[0].parse().unwrap();
    let buses: Vec<(u64, u64)> = data[1]
        .split(',')
        .enumerate()
        .filter(|x| x.1 != "x")
        .map(|x| (x.1.parse::<u64>().unwrap(), x.0 as u64))
        .collect();

    part_1(earliest_depart, &buses);
    part_2(&buses);
}
