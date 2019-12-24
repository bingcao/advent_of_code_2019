use num::Integer;
use std::collections::HashSet;
use std::fmt;

struct Planet {
    position: (i32, i32, i32),
    velocity: (i32, i32, i32),
}
impl fmt::Debug for Planet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "pos=<x={}, y={}, z={}> vel=<x={}, y={}, z={}>",
            self.position.0,
            self.position.1,
            self.position.2,
            self.velocity.0,
            self.velocity.1,
            self.velocity.2
        )
    }
}

impl Planet {
    fn new(planet_str: &str) -> Self {
        let new_str = &planet_str[1..planet_str.len() - 1];
        let positions = new_str.split(", ").collect::<Vec<&str>>();
        let position = (
            positions[0][2..].parse::<i32>().unwrap(),
            positions[1][2..].parse::<i32>().unwrap(),
            positions[2][2..].parse::<i32>().unwrap(),
        );

        Planet {
            position,
            velocity: (0, 0, 0),
        }
    }

    fn energy(&self) -> i32 {
        (self.position.0.abs() + self.position.1.abs() + self.position.2.abs())
            * (self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs())
    }

    fn gravity(&mut self, other: &Self) {
        if self.position.0 < other.position.0 {
            self.velocity.0 += 1
        } else if self.position.0 > other.position.0 {
            self.velocity.0 -= 1
        }
        if self.position.1 < other.position.1 {
            self.velocity.1 += 1
        } else if self.position.1 > other.position.1 {
            self.velocity.1 -= 1
        }
        if self.position.2 < other.position.2 {
            self.velocity.2 += 1
        } else if self.position.2 > other.position.2 {
            self.velocity.2 -= 1
        }
    }

    fn velocity(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
    }
}

fn apply_gravity(planets: &mut Vec<Planet>) {
    for i in 0..planets.len() - 1 {
        let (a, b) = planets.split_at_mut(i + 1);
        let planet_a = a.get_mut(i).unwrap();
        for j in 0..b.len() {
            let planet_b = b.get_mut(j).unwrap();
            planet_a.gravity(&planet_b);
            planet_b.gravity(&planet_a);
        }
    }
}

fn apply_velocity(planets: &mut Vec<Planet>) {
    planets.iter_mut().for_each(|p| p.velocity());
}

fn print_info(planets: &Vec<Planet>, step: u128) {
    println!("After {} steps:", step);
    for planet in planets {
        println!("{:?}", planet);
    }
    println!("");
}

pub fn run_galaxy(planets_str: &str, num_steps: u128) -> i32 {
    let mut planets = planets_str
        .split("\n")
        .map(|s| Planet::new(s))
        .collect::<Vec<Planet>>();
    print_info(&planets, 0);
    for i in 0..num_steps {
        apply_gravity(&mut planets);
        apply_velocity(&mut planets);
        if (i + 1) % 20 == 0 {
            print_info(&planets, i + 1);
        }
    }
    planets.iter().map(|p| p.energy()).sum()
}

fn get_keys(planets: &Vec<Planet>) -> [[i32; 8]; 3] {
    let mut xs = [0; 8];
    let mut ys = [0; 8];
    let mut zs = [0; 8];
    for (i, planet) in planets.iter().enumerate() {
        xs[2 * i] = planet.position.0;
        ys[2 * i] = planet.position.1;
        zs[2 * i] = planet.position.2;
        xs[2 * i + 1] = planet.velocity.0;
        ys[2 * i + 1] = planet.velocity.1;
        zs[2 * i + 1] = planet.velocity.2;
    }
    [xs, ys, zs]
}

pub fn find_reset(planets_str: &str) -> u128 {
    let mut planets = planets_str
        .split("\n")
        .map(|s| Planet::new(s))
        .collect::<Vec<Planet>>();
    print_info(&planets, 0);

    let mut seen_x = HashSet::new();
    let mut seen_y = HashSet::new();
    let mut seen_z = HashSet::new();
    let keys = get_keys(&planets);
    seen_x.insert(keys[0]);
    seen_y.insert(keys[1]);
    seen_z.insert(keys[2]);

    let mut x_period = 0;
    let mut y_period = 0;
    let mut z_period = 0;

    let mut step = 1;
    while x_period == 0 || y_period == 0 || z_period == 0 {
        apply_gravity(&mut planets);
        apply_velocity(&mut planets);

        let keys = get_keys(&planets);
        if x_period == 0 && seen_x.contains(&keys[0]) {
            x_period = step;
        } else {
            seen_x.insert(keys[0]);
        }
        if y_period == 0 && seen_y.contains(&keys[1]) {
            y_period = step;
        } else {
            seen_y.insert(keys[1]);
        }

        if z_period == 0 && seen_z.contains(&keys[2]) {
            z_period = step;
        } else {
            seen_z.insert(keys[2]);
        }

        if step % 10000 == 0 {
            print_info(&planets, step);
        }
        step += 1;
    }

    println!("x: {:?}, y: {:?}, z: {:?}", x_period, y_period, z_period,);

    x_period.lcm(&y_period).lcm(&z_period)
}
