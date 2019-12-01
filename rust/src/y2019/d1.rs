#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass_fuel() {
        assert_eq!(mass_fuel(&12), 2);
        assert_eq!(mass_fuel(&14), 2);
        assert_eq!(mass_fuel(&1969), 654);
        assert_eq!(mass_fuel(&100756), 33583);
    }

    #[test]
    fn test_recursive_mass_fuel() {
        assert_eq!(recursive_mass_fuel(&14), 2);
        assert_eq!(recursive_mass_fuel(&1969), 966);
        assert_eq!(recursive_mass_fuel(&100756), 50346);
    }
}

fn parse_input(masses: &str) -> Vec<i32> {
    return masses
        .to_string()
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
}

fn mass_fuel(mass: &i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel > 0 {
        return fuel;
    } else {
        return 0;
    }
}

fn naive_fuel(masses: &Vec<i32>) -> i32 {
    return masses.iter().map(|x| mass_fuel(x)).sum();
}

fn recursive_mass_fuel(mass: &i32) -> i32 {
    let fuel = mass_fuel(mass);
    if fuel > 0 {
        return fuel + recursive_mass_fuel(&fuel);
    } else {
        return fuel;
    }
}

fn recursive_fuel(masses: &Vec<i32>) -> i32 {
    return masses.iter().map(|x| recursive_mass_fuel(x)).sum();
}

pub fn run(masses: &str) {
    let parsed = parse_input(masses);
    println!("{:?}", naive_fuel(&parsed));
    println!("{:?}", recursive_fuel(&parsed));
}
