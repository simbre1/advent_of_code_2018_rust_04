#![allow(dead_code,unused_variables)]
extern crate chrono;

use chrono::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::ops::Range;

fn main() {
    let i = "01".parse::<u32>().unwrap();

    let contents = fs::read_to_string("D:\\dev\\advent_of_code_2018\\rust-04\\input.txt")
        .expect("peut");

    let mut lines: Vec<&str> = contents.lines().collect();
    lines.sort_by(|a, b|
            get_datetime_from_str(a)
                .partial_cmp(&get_datetime_from_str(b))
                .unwrap_or(std::cmp::Ordering::Equal));

    let mut guards_map: HashMap<u32, Guard> = HashMap::new();

    let mut current_id = 0;
    let mut current_start = None;

    for line in lines {
        let datetime = get_datetime_from_str(line);
        let new_id = get_id_from_str(line.chars().as_str());

        if new_id.is_some() {
            current_id = new_id.unwrap();
            if !guards_map.contains_key(&current_id) {
                guards_map.insert(
                    current_id,
                    Guard { id: current_id, schedule: Vec::new() });
            }
        } else if line.find("wakes up").is_some() {
            let timeslot = TimeSlot {
                start: current_start.unwrap(),
                stop: datetime
            };

            let guard_opt = guards_map.get_mut(&current_id);
            if guard_opt.is_some() {
                guard_opt.unwrap().schedule.push(timeslot);
            } else {
                println!("noooooo");
                current_start = None;
            };
        } else if line.find("falls").is_some() {
            current_start = Some(datetime);
        };
    };

    let datetimes: Vec<DateTime<Utc>> = contents.lines()
        .map(|line| get_datetime_from_str(line))
        .collect();

    let guards: Vec<&Guard> = guards_map.values().collect();

    {
        let sleepiest_guard = guards.iter()
            .max_by(|a, b| a.sleepy_time().cmp(&b.sleepy_time()));
        if sleepiest_guard.is_some() {
            let sleepiest_guard = sleepiest_guard.unwrap();
            show_guard(sleepiest_guard);
        };
    }

    {
        let part_two_guard = guards.iter()
            .max_by(|a, b| a.deepest_sleep().1.cmp(&b.deepest_sleep().1));
        if part_two_guard.is_some() {
            let part_two_guard = part_two_guard.unwrap();
            show_guard(part_two_guard);
        };
    }
}

fn show_guard(guard: &Guard) {
    println!(
        "guard {}, sleepy {}, minute {} # {}, answer code: {}",
        guard.id,
        guard.sleepy_time(),
        guard.deepest_sleep().0,
        guard.deepest_sleep().1,
        guard.id * guard.deepest_sleep().0);

    for ts in &guard.schedule {
        println!("{} -> {}", ts.start, ts.stop);
    }
}

struct TimeSlot {
    start: DateTime<Utc>,
    stop: DateTime<Utc>
}

impl TimeSlot {
    fn minutes_range(&self) -> Range<u32> {
        Range{
            start: self.start.minute(),
            end: self.stop.minute()
        }
    }

    fn minutes(&self) -> u32 {
        ((self.stop.timestamp() - self.start.timestamp()) / 60) as u32
    }

    fn intersect(&self, other: &TimeSlot) -> Option<(u32, u32)> {
        let a = self.minutes_range();
        let b = self.minutes_range();

        if a.start > b.end || b.start > a.end {
            None
        } else {
            Some((
                std::cmp::max(a.start, b.start),
                std::cmp::min(a.end, b.end)
            ))
        }
    }

    fn check(&self, table: &mut [u32; 60]) {
        for i in self.minutes_range() {
            table[i as usize] += 1;
        }
    }
}


struct Guard {
    id: u32,
    schedule: Vec<TimeSlot>
}

impl Guard {
    fn sleepy_time(&self) -> u32 {
        self.schedule.iter()
            .map(|ts| ts.minutes())
            .sum()
    }

    fn deepest_sleep(&self) -> (u32, u32) {
        let mut table: [u32; 60] = [0; 60];
        self.schedule.iter().for_each(|ts| ts.check(&mut table));

        let mut max = 0;
        let mut max_i = 0;
        for i in 0..60 {
            if table[i] > max {
                max = table[i];
                max_i = i;
            }
        }
        (max_i as u32, max)
    }
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