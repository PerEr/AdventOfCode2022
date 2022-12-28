use std::{fs, ops::RangeInclusive};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
struct Pos {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Clone)]
struct SensorData {
    sensor: Pos,
    beacon: Pos,
}

#[derive(Debug, PartialEq, Clone)]
struct ExclusionZone {
    top_left: Pos,
    bottom_right: Pos,
    sensors: Vec<SensorData>,
}

fn parse_indata(indata: &str) -> ExclusionZone {
    let sensors: Vec<SensorData> = Regex::new(r".* x=([-]?\d+), y=([-]?\d+): .* x=([-]?\d+), y=([-]?\d+)")
        .unwrap()
        .captures_iter(indata)
        .map(|cap| {
            SensorData {
                sensor: Pos {x: cap[1].parse().unwrap(), y: cap[2].parse().unwrap()},
                beacon: Pos {x: cap[3].parse().unwrap(), y: cap[4].parse().unwrap()},
         }
        })
        .collect();
    let extremes = sensors.iter().map(|sd| (
        sd.sensor.x.min(sd.beacon.x),
        sd.sensor.y.min(sd.beacon.y),
        sd.sensor.x.max(sd.beacon.x), 
        sd.sensor.y.max(sd.beacon.y)
    )).fold((i64::MAX, i64::MAX, i64::MIN, i64::MIN), |a,x| (
        a.0.min(x.0),
        a.1.min(x.1),
        a.2.max(x.2),
        a.3.max(x.3)
    ));
    let (top_left, bottom_right) = (Pos { x: extremes.0, y: extremes.1}, Pos {x: extremes.2, y:extremes.3});
    ExclusionZone { top_left, bottom_right, sensors }
}

fn calc_ranges_for_line(y: i64, ez: &ExclusionZone) -> (Vec<RangeInclusive<i64>>, Vec<i64>) {
    let mut ranges = Vec::new();
    for s in &ez.sensors {
        let dist_to_beacon = (s.sensor.x-s.beacon.x).abs() + (s.sensor.y-s.beacon.y).abs();
        let dy = (y - s.sensor.y).abs();
        if dy <= dist_to_beacon {
            let dx = dist_to_beacon - dy;
            ranges.push(s.sensor.x-dx..=s.sensor.x + dx);
        }
    }
    ranges.sort_by_key(|r| *r.start());
    let coalesced_ranges = ranges.into_iter().coalesce(|a, b| {
        if b.start() - 1 <= *a.end() {
            if b.end() > a.end() {
                Ok(*a.start()..=*b.end())
            } else {
                Ok(a)
            }
        } else {
            Err((a, b))
        }
    }).collect();
    let beacon_xs = ez.sensors.iter().map(|sd| &sd.beacon).filter(|p| p.y == y).map(|p| p.x).collect();
    (coalesced_ranges, beacon_xs)
}

fn count_excluded_in_ranges(ranges: &Vec<RangeInclusive<i64>>, beacons: &Vec<i64>, left: i64, right: i64) -> i64 {
    let mut sum = 0;
    for x in left..=right {
        if !beacons.contains(&x) {
            let mut excluded = false;
            for r in ranges {
                if x >= *r.start() && x <= *r.end() {
                    excluded = true;
                    break;
                }
            }
            if excluded {
                sum += 1;
            }
        }
    }
    sum
}

fn calc_candidate_beacon_positions(range: RangeInclusive<i64>, ez: &ExclusionZone) -> Vec<(i64,i64,i64)>{
    let mut candidates: Vec<(i64,i64,i64)> = Vec::new();
    for line in range {
        let (ranges, _) = calc_ranges_for_line(line, &ez);
        if ranges.len() > 1 {
            let x = *ranges[1].start() - 1;
            candidates.push((x, line, x * 4000000 + line));

        }
    }
    candidates
}

fn main() {
    let indata = fs::read_to_string("data/day15.txt").expect("No indata");
    let exclusion_zone = parse_indata(&indata);
    assert_eq!(23, exclusion_zone.sensors.len());
    let (ranges, beacons) = calc_ranges_for_line(2000000, &exclusion_zone);
    let xspan = ranges.iter()
        .fold(ranges[0].clone(), |a,x| {
            let xmin: i64 = *a.start().min(x.start());
            let xmax: i64 = *a.end().max(x.end());
            xmin..=xmax
        });
    let excl = count_excluded_in_ranges(&ranges, &beacons, *xspan.start(), *xspan.end());
    println!("Part1: {}", excl);

    let pos_to_test = calc_candidate_beacon_positions(0..=4000000, &exclusion_zone);
    println!("Part2: {:?}", pos_to_test);

}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_DATA: &'static str = indoc! {r#"
    Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    Sensor at x=9, y=16: closest beacon is at x=10, y=16
    Sensor at x=13, y=2: closest beacon is at x=15, y=3
    Sensor at x=12, y=14: closest beacon is at x=10, y=16
    Sensor at x=10, y=20: closest beacon is at x=10, y=16
    Sensor at x=14, y=17: closest beacon is at x=10, y=16
    Sensor at x=8, y=7: closest beacon is at x=2, y=10
    Sensor at x=2, y=0: closest beacon is at x=2, y=10
    Sensor at x=0, y=11: closest beacon is at x=2, y=10
    Sensor at x=20, y=14: closest beacon is at x=25, y=17
    Sensor at x=17, y=20: closest beacon is at x=21, y=22
    Sensor at x=16, y=7: closest beacon is at x=15, y=3
    Sensor at x=14, y=3: closest beacon is at x=15, y=3
    Sensor at x=20, y=1: closest beacon is at x=15, y=3
    "#
    };

    fn draw_ranges(ranges: &Vec<RangeInclusive<i64>>, beacons: &Vec<i64>, left: i64, right: i64) -> String {
        let mut line = String::new();
        for x in left..=right {
            if beacons.contains(&x) {
                line += "B";
            } else {
                let mut excluded = false;
                for r in ranges {
                    if x >= *r.start() && x <= *r.end() {
                        excluded = true;
                        break;
                    }
                }
                    line += if excluded {"#"} else {"."};
            }
        }
        line
    }
        
    #[test]
    fn test_part1() {
        let exclusion_zone = parse_indata(&TEST_DATA);
        assert_eq!(14, exclusion_zone.sensors.len());
        assert_eq!(SensorData { 
            sensor: Pos {x:2, y:18}, 
            beacon: Pos {x:-2, y:15},
        }, exclusion_zone.sensors[0]);
        assert_eq!(SensorData { 
            sensor: Pos {x:20, y:1}, 
            beacon: Pos {x:15, y:3},
        }, exclusion_zone.sensors[13]);
        assert_eq!(Pos {x: -2, y: 0}, exclusion_zone.top_left);
        assert_eq!(Pos {x: 25, y: 22}, exclusion_zone.bottom_right);

        let (ranges, beacons) = calc_ranges_for_line(10, &exclusion_zone);
        let line = draw_ranges(&ranges, &beacons, exclusion_zone.top_left.x, exclusion_zone.bottom_right.x);
        assert_eq!("####B######################.", line);
        assert_eq!(26, count_excluded_in_ranges(&ranges, &beacons, exclusion_zone.top_left.x, exclusion_zone.bottom_right.x));
    }

    #[test]
    fn test_part2() {
        let exclusion_zone = parse_indata(&TEST_DATA);
        let pos_to_test = calc_candidate_beacon_positions(0..=20, &exclusion_zone);
        assert_eq!(1, pos_to_test.len());
        assert_eq!((14,11,56000011), pos_to_test[0]);
    }


}
