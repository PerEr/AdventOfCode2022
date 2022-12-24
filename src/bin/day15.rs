use std::{fs};
use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
struct Pos {
    x: i32,
    y:i32,
}

#[derive(Debug, PartialEq, Clone)]
struct SensorData {
    sensor: Pos,
    beacon: Pos,
}

fn parse_indata(indata: &str) -> Vec<SensorData> {
    Regex::new(r".* x=([-]?\d+), y=([-]?\d+): .* x=([-]?\d+), y=([-]?\d+)")
        .unwrap()
        .captures_iter(indata)
        .map(|cap| {
            SensorData {
                sensor: Pos {x: cap[1].parse().unwrap(), y: cap[2].parse().unwrap()},
                beacon: Pos {x: cap[3].parse().unwrap(), y: cap[4].parse().unwrap()},
         }
        })
        .collect()
}


fn main() {
    let indata = fs::read_to_string("data/day15.txt").expect("No indata");
    let sensor_data = parse_indata(&indata);
    assert_eq!(23, sensor_data.len());
    println!("{:?}", sensor_data);
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

    #[test]
    fn test_part1() {
        let sensor_data = parse_indata(&TEST_DATA);
        assert_eq!(14, sensor_data.len());
        assert_eq!(SensorData { 
            sensor: Pos {x:2, y:18}, 
            beacon: Pos {x:-2, y:15},
        }, sensor_data[0]);
        assert_eq!(SensorData { 
            sensor: Pos {x:20, y:1}, 
            beacon: Pos {x:15, y:3},
        }, sensor_data[13]);
    }


}
