use std::collections::{HashMap, HashSet};
use std::cell::RefCell;
use std::fmt;
use std::rc::{Rc, Weak};

struct Planet {
    children: RefCell<Vec<Rc<Planet>>>,
    parents: RefCell<Vec<Weak<Planet>>>,
    name: String,
}
impl fmt::Debug for Planet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parent_names: Vec<String> = self.parents.borrow().iter().map(|weak| weak.upgrade().unwrap().name.clone()).collect(); 
        let child_names: Vec<String> = self.children.borrow().iter().map(|child| child.name.clone()).collect();
        write!(
            f, 
            "Point {{ name: {}, parents: {:?}, children: {:?} }}", 
            self.name, 
            parent_names,
            child_names
        )
    }
}
impl Planet {
    fn new(name: &String) -> Planet {
        Planet { 
            name: name.clone(), 
            parents: RefCell::new(vec![]), 
            children: RefCell::new(vec![]),
        }
    }

    fn add_child(&self, child: &Rc<Planet>) {
        self.children.borrow_mut().push(Rc::clone(child));
    }

    fn add_parent(&self, parent: &Rc<Planet>) {
        self.parents.borrow_mut().push(Rc::downgrade(parent));
    }

    fn compute_orbits(&self, count_map: &HashMap<String, usize>) -> usize {
        let num_directs = self.children.borrow().len();
        let num_indirects = self.children.borrow().iter().fold(0, |acc, child| acc + count_map.get(&child.name).unwrap());
        num_directs + num_indirects
    }
}

fn init_planets(orbits: &[&str]) -> HashMap<String, RefCell<Rc<Planet>>> {
    let mut name_to_planets: HashMap<String, RefCell<Rc<Planet>>> = HashMap::new();
    for orbit in orbits {
        let orbit: Vec<&str> = orbit.split(")").collect();
        let child = String::from(orbit[0]);
        let parent = String::from(orbit[1]);

        if !name_to_planets.contains_key(&child) {
            let child_default = RefCell::new(Rc::new(Planet::new(&child)));
            name_to_planets.insert(child.clone(), child_default); 
        }
        if !name_to_planets.contains_key(&parent) {
            let parent_default = RefCell::new(Rc::new(Planet::new(&parent)));
            name_to_planets.insert(parent.clone(), parent_default); 
        }

        let child_planet = name_to_planets.get(&child).unwrap();
        let parent_planet = name_to_planets.get(&parent).unwrap();

        child_planet.borrow_mut().add_parent(&parent_planet.borrow());
        parent_planet.borrow_mut().add_child(&child_planet.borrow());
    }
    name_to_planets
}


pub fn calc_checksum(orbits: &[&str]) -> usize {
    let name_to_planets = init_planets(orbits);
    let mut horizon: Vec<String> = name_to_planets.iter()
                                     .filter(|(_name, planet)| planet.borrow().children.borrow().len() == 0)
                                     .map(|(name, _planet)| name.clone())
                                     .collect();
    let mut count_map: HashMap<String, usize> = HashMap::new();
    loop {
        let mut new_horizon: Vec<String> = vec![];

        if horizon.len() == 0 {
            break
        }

        for name in &horizon {
            let planet = name_to_planets.get(name).unwrap().borrow(); 
            let count = planet.compute_orbits(&count_map);
            count_map.insert(name.clone(), count);
            let parent_names: Vec<String> = planet.parents.borrow().iter().map(|p| p.upgrade().unwrap().name.clone()).collect();
            new_horizon.extend(parent_names);
        }
        horizon = new_horizon;
    }

    count_map.values().sum()
}


pub fn calc_transfers(orbits: &[&str]) -> usize {
    let name_to_planets = init_planets(orbits);
    let mut horizon: Vec<String> = name_to_planets.get("YOU")
                                    .unwrap()
                                    .borrow()
                                    .children
                                    .borrow()
                                    .iter()
                                    .map(|c| c.name.clone())
                                    .collect();
    let mut path_len = 0;
    let mut seen = HashSet::new();
    loop {
        let mut new_horizon: Vec<String> = vec![];

        if horizon.len() == 0 {
            panic!("Couldn't find path");
        }

        for name in &horizon {
            if name == "SAN" {
                return path_len - 1
            }
            let planet = name_to_planets.get(name).unwrap().borrow(); 
            let next_parents: Vec<String> = planet.parents.borrow()
                                        .iter()
                                        .map(|p| p.upgrade().unwrap().name.clone())
                                        .filter(|name| !seen.contains(name))
                                        .collect();
            let next_children: Vec<String> = planet.children.borrow()
                                        .iter()
                                        .map(|c| c.name.clone())
                                        .filter(|name| !seen.contains(name))
                                        .collect();
            let next = [&next_parents[..], &next_children[..]].concat();
            for name in &next {
                seen.insert(name.clone());
            }
            new_horizon.extend(next);
        }
        horizon = new_horizon;
        path_len += 1;
    }
}
