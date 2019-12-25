use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
struct Element {
    amount: u64,
    name: String,
}
impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.amount, self.name)
    }
}
impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Element {}
impl Element {
    fn new(amount: u64, name: String) -> Self {
        Element { amount, name }
    }

    fn from_string(element_str: &str) -> Self {
        let element_vec = element_str.split(" ").collect::<Vec<&str>>();
        let amount = element_vec[0].parse().unwrap();
        let name = element_vec[1].into();
        Element::new(amount, name)
    }

    fn add(&self, amount: u64) -> Self {
        Element {
            amount: self.amount + amount,
            name: self.name.clone(),
        }
    }
}
impl Hash for Element {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

struct Reaction {
    els: Vec<Element>,
    result: Element,
}
impl Reaction {
    fn new(reaction_str: &str) -> Self {
        let reaction_vec = reaction_str.split(" => ").collect::<Vec<&str>>();
        let left = reaction_vec[0];
        let right = reaction_vec[1];
        Reaction {
            els: left
                .split(", ")
                .map(|el_str| Element::from_string(el_str))
                .collect(),
            result: Element::from_string(right),
        }
    }
}

fn get_graph_map(reactions_str: &str) -> HashMap<Element, Vec<Element>> {
    let reactions = reactions_str
        .split("\n")
        .map(|reaction_str| Reaction::new(reaction_str))
        .collect::<Vec<Reaction>>();

    let mut graph_map = reactions
        .into_iter()
        .map(|r| (r.result, r.els))
        .collect::<HashMap<Element, Vec<Element>>>();
    graph_map.insert(Element::from_string("1 ORE"), vec![]);
    graph_map
}

fn get_ore_for_fuel(
    graph_map: &mut HashMap<Element, Vec<Element>>,
    fuel_needed: u64,
    debug: bool,
) -> u64 {
    /*
     * 1. Need: [1 FUEL]; Produced: {}
     *      a. For 1 FUEL, get 1 FUEL -> 7A, 1E => Need: [7 A, 1 E]; Produced: {FUEL: 1}
     * 2. Need: [7 A, 1 E]; Produced: {FUEL: 1}
     *      a. For 7A, get 10 A -> 10 ORE => Need: [10 ORE]; Produced: {FUEL: 1, A: 10}, Extra:
     *          {A: 3}
     *      b. For 1E, get 1 E -> 7A, 1D => Need: [10 ORE, 7A, 1D]; Produced: {FUEL: 1, A: 10,
     *          1E}, Extra: {A: 3}
     * 3. Need: [10 ORE, 7A, 1D]; Produced: {FUEL: 1, A: 10, E: 1}; Extra: {A: 3}
     *      a. For 10 ORE, get 1 ORE -> [] => Need: []; Produced: {FUEL: 1, A: 10, E: 1, ORE: 10},
     *          Extra: {A: 3}
     */
    let mut needed: HashSet<Element> = vec![Element::new(fuel_needed, String::from("FUEL"))]
        .into_iter()
        .collect();
    let mut produced: HashMap<String, u64> = HashMap::new();
    let mut extra: HashMap<String, u64> = HashMap::new();
    let mut i = 0;
    while needed.len() > 0 {
        if debug {
            println!(
                "{}. Need: {:?}; Produced: {:?}, Extra: {:?}",
                i, needed, produced, extra
            );
        }

        let mut new_needed: HashSet<Element> = HashSet::new();
        for need in needed.iter() {
            let reagents_entry = graph_map.entry(need.clone());
            let produced_amount = reagents_entry.key().amount;
            let reagents = match reagents_entry {
                Entry::Occupied(e) => e.get().clone(),
                _ => panic!("Could not find element {:?} in graph", need),
            };
            if debug {
                println!("\t{:?} => {} {}", reagents, produced_amount, need.name);
            }

            let produced_entry = produced.entry(need.name.clone()).or_insert(0);
            let extra_entry = extra.entry(need.name.clone()).or_insert(0);

            let mut needed_amount = need.amount;
            if needed_amount <= *extra_entry {
                *extra_entry -= needed_amount;
                continue;
            }
            needed_amount -= *extra_entry;
            *extra_entry = 0;

            let num_reactions = divide_and_round_up(needed_amount, produced_amount);
            if debug {
                println!(
                    "\tFor {}: produced {} and need {}, so running {} rxs",
                    need.name, produced_amount, needed_amount, num_reactions
                );
            }
            *produced_entry += produced_amount * num_reactions;
            *extra_entry += num_reactions * produced_amount - needed_amount;
            for reagent in reagents.into_iter() {
                let needed_amount = reagent.amount * num_reactions;
                if let Some(r) = new_needed.get(&reagent).cloned() {
                    let new_r = r.add(needed_amount);
                    new_needed.replace(new_r);
                } else {
                    new_needed.insert(Element::new(needed_amount, reagent.name));
                }
            }
            if debug {
                println!(
                    "\tNeed: {:?}, Produced: {:?}, Extra: {:?}",
                    new_needed, produced, extra
                );
                println!("");
            }
        }
        needed = new_needed;
        i += 1;
    }
    *produced.get("ORE").unwrap()
}

fn divide_and_round_up(a: u64, b: u64) -> u64 {
    (a + (b - 1)) / b
}

pub fn get_ore_and_fuel(reactions_str: &str) -> (u64, u64) {
    let mut graph_map = get_graph_map(reactions_str);
    let debug = env::var_os("DEBUG").is_some();
    if debug {
        println!("{:?}", graph_map);
    }

    let ore_for_one = get_ore_for_fuel(&mut graph_map, 1, debug);
    let mut search_min = 0;
    let mut search_max = 10000;
    let mut highest_ore = 0;
    let mut fuel_for_ore = 0;
    loop {
        println!("Searching between {} and {}", search_min, search_max);
        let est = (search_max + search_min) / 2;
        let ore = get_ore_for_fuel(&mut graph_map, est, debug);
        println!("{} ore creates {} fuel", ore, est);

        if ore > 1_000_000_000_000 {
            search_max = est;
        } else {
            if ore > highest_ore {
                fuel_for_ore = est;
                highest_ore = ore;
            } else {
                break;
            }
            if est == search_max || est + 1 == search_max {
                search_max *= 2;
            }
            search_min = est;
        }
    }

    (ore_for_one, fuel_for_ore)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_part_1 {
        use super::*;

        // #[test]
        // fn test_1() {
        //     let reactions_str = "10 ORE => 10 A
        // 1 ORE => 1 B
        // 7 A, 1 B => 1 C
        // 7 A, 1 C => 1 D
        // 7 A, 1 D => 1 E
        // 7 A, 1 E => 1 FUEL";
        //     let result = get_ore(reactions_str);
        //     assert_eq!(result, 31);
        // }

        // #[test]
        // fn test_2() {
        //     let reactions_str = "9 ORE => 2 A
        // 8 ORE => 3 B
        // 7 ORE => 5 C
        // 3 A, 4 B => 1 AB
        // 5 B, 7 C => 1 BC
        // 4 C, 1 A => 1 CA
        // 2 AB, 3 BC, 4 CA => 1 FUEL";
        //     let result = get_ore(reactions_str);
        //     assert_eq!(result, 165);
        // }

        #[test]
        fn test_3() {
            let reactions_str = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
            let result = get_ore_and_fuel(reactions_str);
            assert_eq!(result, (13312, 82892753));
        }

        #[test]
        fn test_4() {
            let reactions_str = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";
            let result = get_ore_and_fuel(reactions_str);
            assert_eq!(result, (180697, 5586022));
        }

        #[test]
        fn test_5() {
            let reactions_str = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
            let result = get_ore_and_fuel(reactions_str);
            assert_eq!(result, (2210736, 460664));
        }
    }
}
