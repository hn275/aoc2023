use std::env;

fn main() {
    // parse file
    let file_name = &env::args().collect::<Vec<String>>()[1];
    let mut input: Vec<String> = Vec::new();

    for line in std::fs::read_to_string(file_name).unwrap().lines() {
        input.push(line.to_string());
    }

    let times = input[0].split(":").collect::<Vec<&str>>()[1]
        .split(' ')
        .filter(|c| *c != "")
        .collect::<Vec<&str>>();

    let records = input[1].split(":").collect::<Vec<&str>>()[1]
        .split(' ')
        .filter(|c| *c != "")
        .collect::<Vec<&str>>();

    let p_one = part_one(&times, &records);
    dbg!(p_one);
    let p_two = part_two(&times, &records);
    dbg!(p_two);
}
fn part_two(t: &Vec<&str>, r: &Vec<&str>) -> u64 {
    let mut time: String = String::new();
    for i in t {
        time.push_str(*i);
    }
    let time = time.parse::<u64>().unwrap();

    let mut record: String = String::new();
    for i in r {
        record.push_str(*i);
    }
    let record = record.parse::<u64>().unwrap();

    return race(time, record);
}

fn part_one(t: &Vec<&str>, r: &Vec<&str>) -> u64 {
    let times = t
        .iter()
        .map(|c| c.to_string().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let records = r
        .iter()
        .map(|c| c.to_string().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut rec: Vec<u64> = Vec::new();
    for i in 0..times.len() {
        let result = race(times[i], records[i]);
        if result != 0 {
            rec.push(result);
        }
    }

    let mut c = 0;
    for r in rec {
        if c == 0 {
            c = r;
        } else {
            c *= r;
        }
    }

    return c;
}

fn race(time: u64, record: u64) -> u64 {
    let mut count: u64 = 0;
    for i in 0..time {
        let d = i * (time - i);
        if d > record {
            count += 1;
        }
    }

    return count;
}
