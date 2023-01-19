use std::collections::HashMap;
use std::collections::HashSet;

pub fn part_1(input: &str, y_read: i32) -> u32 {
    let (sensors, beacons) = find_sensors_and_beacons(input);
    let (x_min, x_max) = find_extreme_x_values(&sensors);

    let mut positions_with_no_beacons = 0;
    for x in x_min..=x_max {
        if beacons.contains(&(x, y_read)) { continue }
        'inner: for (&sensor, &distance) in &sensors {
            if manhattan(sensor, (x, y_read)) <= distance {
                positions_with_no_beacons += 1;
                break 'inner
            }
        }
    }

    positions_with_no_beacons
}

fn find_sensors_and_beacons(input: &str) -> (HashMap<(i32, i32), u32>, HashSet<(i32, i32)>) {
    let mut sensors: HashMap<(i32, i32), u32> = HashMap::new();
    let mut beacons: HashSet<(i32, i32)> = HashSet::new();

    for line in input.lines() {
        let line = &line.replace(&[',', ':'][..], "")[..];
        let line_split = line.split(' ').collect::<Vec<&str>>();
        let mut positions = line_split.iter().filter(|&x| x.contains('='));
        let sensor = (positions.next().unwrap()[2..].parse::<i32>().unwrap(), positions.next().unwrap()[2..].parse::<i32>().unwrap());
        let beacon = (positions.next().unwrap()[2..].parse::<i32>().unwrap(), positions.next().unwrap()[2..].parse::<i32>().unwrap());
        let manhattan = manhattan(sensor, beacon);

        sensors.insert(sensor, manhattan);
        beacons.insert(beacon);
    }
    (sensors, beacons)
}

fn find_extreme_x_values(sensors: &HashMap<(i32, i32), u32>) -> (i32, i32) {
    let mut x_min = i32::MAX;
    let mut x_max = i32::MIN;

    for (&sensor, &distance) in sensors {
        let x_left = sensor.0 - distance as i32;
        if x_left < x_min { x_min = x_left }

        let x_right = sensor.0 + distance as i32;
        if x_right > x_max { x_max = x_right }
    }
    (x_min, x_max)
}

fn manhattan(a: (i32, i32), b: (i32, i32)) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

pub fn part_2(input: &str, max_coordinate: u32) -> u64 {
    const TUNING_CONST: i32 = 4000000;
    let (sensors, _) = find_sensors_and_beacons(input);

    let mut perimeters = HashSet::new();
    for (&sensor, &distance) in &sensors {
        perimeters.extend(find_and_filter_search_perimeter(sensor, distance, max_coordinate));
    }

    let perimeter_copy = perimeters.iter().map(Clone::clone).collect::<Vec<(i32, i32)>>();
    'outer: for perimeter_point in perimeter_copy {
        for (&sensor, &distance) in &sensors {
            if manhattan(sensor, perimeter_point) <= distance {
                perimeters.remove(&perimeter_point);
                continue 'outer
            }
        }
    }
    let &p = perimeters.iter().next().unwrap();
    (p.0 * TUNING_CONST + p.1) as u64
}

pub fn find_and_filter_search_perimeter(sensor: (i32, i32), distance: u32, max_coordinate: u32) -> Vec<(i32, i32)> {
    let max_coordinate = max_coordinate as i32;
    let perimeter: Vec<(i32, i32)> = find_search_perimeter(sensor, distance);
    perimeter.into_iter().filter(|&p| p.0 >= 0 && p.0 <= max_coordinate && p.1 >= 0 && p.1 <= max_coordinate).collect()
}

pub fn find_search_perimeter(sensor: (i32, i32), distance: u32) -> Vec<(i32, i32)> {
    let distance = distance as i32 + 1;
    let mut perimeter = Vec::new();
    for offset in 0..distance {
        perimeter.push((sensor.0 + offset, sensor.1 + distance - offset));
        perimeter.push((sensor.0 + offset, sensor.1 - distance + offset));
        perimeter.push((sensor.0 - distance + offset, sensor.1 - offset));
        perimeter.push((sensor.0 - distance + offset, sensor.1 + offset));
    }
    perimeter
}

#[test]
fn sample_input_part_1() {
    let input = include_str!("../input/sample_15.txt");
    assert_eq!(part_1(input, 10), 26)
}

#[test]
fn sample_input_part_2() {
    let input = include_str!("../input/sample_15.txt");
    assert_eq!(part_2(input, 20), 56000011)
}
