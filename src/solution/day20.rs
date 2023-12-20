use num::integer;
use std::collections::{HashMap, VecDeque};

use crate::solution::Solution;
use macros::return_type;

#[return_type(p1 = u64, p2 = u64)]
pub struct Day20;

struct Broadcaster {
    input: Vec<PulseType>,
}

struct FlipFlop {
    last_pulse: PulseType,
    is_on: bool,
}

struct Conjunction {
    /// true if most recent pulse is false
    last_pulse: HashMap<String, PulseType>,
}

enum ModuleComponent {
    Broadcaster(Broadcaster),
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

struct Module {
    inner: ModuleComponent,
    destination: Vec<String>,
}

impl Module {
    fn receive_input(&mut self, module_from: &String, input: &PulseType) {
        match &mut self.inner {
            ModuleComponent::Conjunction(conjunction) => {
                conjunction
                    .last_pulse
                    .insert(module_from.clone(), input.clone());
            }
            ModuleComponent::FlipFlop(flipflop) => {
                flipflop.last_pulse = input.clone();
                match input {
                    &PulseType::HIGH => {}
                    &PulseType::LOW => {
                        flipflop.is_on = !flipflop.is_on;
                    }
                }
            }
            ModuleComponent::Broadcaster(b) => {
                b.input.push(input.clone());
            }
        }
    }
    fn current_output(&mut self) -> Option<PulseType> {
        match &mut self.inner {
            ModuleComponent::Conjunction(c) => {
                if c.last_pulse.values().all(|x| x == &PulseType::HIGH) {
                    Some(PulseType::LOW)
                } else {
                    Some(PulseType::HIGH)
                }
            }
            ModuleComponent::FlipFlop(f) => match f.last_pulse {
                PulseType::HIGH => None,
                PulseType::LOW => {
                    if f.is_on {
                        Some(PulseType::HIGH)
                    } else {
                        Some(PulseType::LOW)
                    }
                }
            },
            ModuleComponent::Broadcaster(b) => {
                if b.input.is_empty() {
                    None
                } else {
                    let ret = Some(b.input[0]);
                    b.input.clear();
                    ret
                }
            }
        }
    }
    fn reset(&mut self) {
        match &mut self.inner {
            ModuleComponent::Conjunction(c) => {
                let keys = c.last_pulse.keys().map(|x| x.clone()).collect::<Vec<_>>();
                for key in keys {
                    c.last_pulse.insert(key, PulseType::LOW);
                }
            }
            ModuleComponent::FlipFlop(f) => {
                f.is_on = false;
            }
            ModuleComponent::Broadcaster(b) => {
                b.input.clear();
            }
        }
    }
}

#[derive(Clone, PartialEq, Copy)]
enum PulseType {
    HIGH,
    LOW,
}

struct Machine {
    modules: Vec<(String, Module)>,
    module_index: HashMap<String, usize>,
}

impl Machine {
    fn new() -> Machine {
        Machine {
            modules: Vec::new(),
            module_index: HashMap::new(),
        }
    }
    fn add_module(&mut self, module_name: String, module: Module) {
        self.module_index
            .insert(module_name.clone(), self.modules.len());
        self.modules.push((module_name, module));
    }
    fn build_connection(&mut self) {
        for i in 0..self.modules.len() {
            for dest in self.modules[i].1.destination.clone().iter() {
                let current_module_name = self.modules[i].0.clone();
                if let Some(dest_module_index) = self.module_index.get(dest) {
                    match &mut self.modules[*dest_module_index].1.inner {
                        ModuleComponent::Conjunction(c) => {
                            c.last_pulse.insert(current_module_name, PulseType::LOW);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    fn has_module(&self, module_name: &str) -> bool {
        self.module_index.contains_key(&String::from(module_name))
    }
    fn push_button(&mut self) -> (u64, u64) {
        let mut num_low = 1;
        let mut num_high = 0;
        let mut active: VecDeque<(String, PulseType, String)> = VecDeque::new();
        for (name, _) in self.modules.iter_mut() {
            if name == "broadcaster" {
                active.push_back((String::new(), PulseType::LOW, name.clone()));
            }
        }
        while let Some((source_module, pulse_type, dest_module)) = active.pop_front() {
            let i = *self.module_index.get(&dest_module).unwrap();
            self.modules[i].1.receive_input(&source_module, &pulse_type);
            let module_output = self.modules[i].1.current_output();
            if module_output.is_none() {
                continue;
            }
            match module_output.unwrap() {
                PulseType::HIGH => num_high += self.modules[i].1.destination.len() as u64,
                PulseType::LOW => num_low += self.modules[i].1.destination.len() as u64,
            }
            let current_module_name = self.modules[i].0.clone();
            for dest in self.modules[i].1.destination.clone().iter() {
                if let Some(dest_module_index) = self.module_index.get(dest) {
                    active.push_back((
                        current_module_name.clone(),
                        module_output.unwrap().clone(),
                        self.modules[*dest_module_index].0.clone(),
                    ));
                }
            }
        }
        (num_low, num_high)
    }
    fn push_until_node_sends_signal(&mut self, module: &String, pulse: &PulseType) -> u64 {
        let mut num_pushes = 0;
        loop {
            num_pushes += 1;
            let mut active: VecDeque<(String, PulseType, String)> = VecDeque::new();
            for (name, _) in self.modules.iter_mut() {
                if name == "broadcaster" {
                    active.push_back((String::new(), PulseType::LOW, name.clone()));
                }
            }
            while let Some((source_module, pulse_type, dest_module)) = active.pop_front() {
                let i = *self.module_index.get(&dest_module).unwrap();
                self.modules[i].1.receive_input(&source_module, &pulse_type);
                let module_output = self.modules[i].1.current_output();
                if module_output.is_none() {
                    continue;
                }
                if dest_module.eq(module) && module_output.unwrap() == *pulse {
                    return num_pushes;
                }
                let current_module_name = self.modules[i].0.clone();
                for dest in self.modules[i].1.destination.clone().iter() {
                    if let Some(dest_module_index) = self.module_index.get(dest) {
                        active.push_back((
                            current_module_name.clone(),
                            module_output.unwrap().clone(),
                            self.modules[*dest_module_index].0.clone(),
                        ));
                    }
                }
            }
        }
    }
    fn reset(&mut self) {
        for (_, module) in self.modules.iter_mut() {
            module.reset();
        }
    }
}

impl Day20 {
    fn parse_module(line: &str) -> (String, Module) {
        let splitted_line = line.split(" -> ").collect::<Vec<_>>();
        match splitted_line[0] {
            "broadcaster" => (
                String::from(splitted_line[0]),
                Module {
                    destination: splitted_line[1]
                        .split(", ")
                        .map(String::from)
                        .collect::<Vec<_>>(),
                    inner: ModuleComponent::Broadcaster(Broadcaster { input: Vec::new() }),
                },
            ),
            _ => {
                let is_flipflop = splitted_line[0].find('%').is_some();
                let module_name = String::from(&splitted_line[0][1..]);
                (
                    module_name,
                    Module {
                        destination: splitted_line[1]
                            .split(", ")
                            .map(String::from)
                            .collect::<Vec<_>>(),
                        inner: if is_flipflop {
                            ModuleComponent::FlipFlop(FlipFlop {
                                is_on: false,
                                last_pulse: PulseType::LOW,
                            })
                        } else {
                            ModuleComponent::Conjunction(Conjunction {
                                last_pulse: HashMap::new(),
                            })
                        },
                    },
                )
            }
        }
    }
}

impl Solution<u64, u64> for Day20 {
    fn part_one<'a>(lines: impl Iterator<Item = &'a str>) -> u64 {
        let mut machine = Machine::new();
        for line in lines {
            let (module_name, module) = Self::parse_module(line);
            machine.add_module(module_name, module);
        }
        machine.build_connection();
        let mut num_high_output = 0u64;
        let mut num_low_output = 0u64;
        for _iter in 0..1000 {
            let (num_low, num_high) = machine.push_button();
            num_high_output += num_high;
            num_low_output += num_low;
        }
        num_high_output * num_low_output
    }
    fn part_two<'a>(lines: impl Iterator<Item = &'a str>) -> u64 {
        let mut machine = Machine::new();
        for line in lines {
            let (module_name, module) = Self::parse_module(line);
            machine.add_module(module_name, module);
        }
        machine.build_connection();
        let special_modules = ["dl", "ns", "bh", "vd"];

        // Sample case does not have "rx" module
        if special_modules
            .iter()
            .any(|module| !machine.has_module(module))
        {
            return u64::MAX;
        }

        special_modules.into_iter().fold(1u64, |acc, module_name| {
            machine.reset();
            integer::lcm(
                acc,
                machine.push_until_node_sends_signal(&String::from(module_name), &PulseType::HIGH),
            )
        })
    }
}
