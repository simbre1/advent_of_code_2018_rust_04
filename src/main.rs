#![allow(dead_code,unused_variables)]
extern crate chrono;

use chrono::prelude::*;
use std::collections::HashMap;
use std::fs;

fn main() {
    let i = "01".parse::<u32>().unwrap();

    let contents = fs::read_to_string("D:\\dev\\advent_of_code_2018\\rust-04\\input.txt")
        .expect("peut");

    let mut lines: Vec<&str> = contents.lines().collect();
    lines.sort_by(|a, b|
            get_datetime_from_str(a)
                .partial_cmp(&get_datetime_from_str(b))
                .unwrap_or(std::cmp::Ordering::Equal));

    let mut guards: HashMap<u32, &mut Guard> = HashMap::new();

    let mut current_id = 0;
    let mut current_start = None;

    for line in lines {
        let datetime = get_datetime_from_str(line);
        let new_id = get_id_from_str(line.chars().as_str());

        if new_id.is_some() {
            current_id = new_id.unwrap();
            current_start = Some(datetime);
        } else if line.find("wakes up").is_some() {
            current_start = Some(datetime);
        } else {
            let timeslot = TimeSlot {
                start: current_start.unwrap(),
                stop: datetime
            };

            let guard_opt = guards.get_mut(&current_id);
            if guard_opt.is_some() {
              guard_opt.unwrap().schedule.push(timeslot);
            } else {
                current_start = None;
            }
        }

    }

    let datetimes: Vec<DateTime<Utc>> = contents.lines()
        .map(|line| get_datetime_from_str(line))
        .collect();

    datetimes.iter().for_each(|dt| println!("{}", dt));

}

/*
012345678901234567
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
*/

struct TimeSlot {
    start: DateTime<Utc>,
    stop: DateTime<Utc>
}

struct Guard {
    id: u32,
    schedule: Vec<TimeSlot>
}

fn get_datetime_from_str(str: &str) -> DateTime<Utc> {
    let ys = 1 + str.find("[").unwrap();
    let ye = 1 + str[ys..].find("-").unwrap();
    let me = ye + 1 + str[ye+1..].find("-").unwrap();
    let de = me + 1 + str[me+1..].find(" ").unwrap();
    let he = de + 1 + str[de + 1..].find(":").unwrap();
    let mine = str.find("]").unwrap();

    let y = str[1..ye].parse::<i32>().unwrap();
    let m = str[ye+1..me].parse::<u32>().unwrap();
    let d = str[me+1..de].parse::<u32>().unwrap();
    let h = str[de+1..he].parse::<u32>().unwrap();
    let min = str[he+1..mine].parse::<u32>().unwrap();

    Utc
        .ymd(y, m, d)
        .and_hms(h, min, 0)
}

fn get_id_from_str(str: &str) -> Option<u32> {
    let id_start = str.find("#");
    if id_start.is_none() {
        return None;
    }

    let id_stop = str[id_start.unwrap()..].find(" ").unwrap();
    Some(
        str.chars()
            .as_str()[id_start.unwrap() + 1..id_start.unwrap() + id_stop]
            .parse::<u32>().unwrap())
}