use std::collections::{HashMap, VecDeque};

use crate::solution::Solution;
use macros::return_type;

#[return_type(p1 = u32, p2 = u64)]
pub struct Day19;

struct Rule {
    variable: String,
    operator: char,
    value: u32,
    destination: String,
}

struct WorkflowManager {
    workflow_mapping: HashMap<String, Vec<Rule>>,
}

struct Input {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

struct InputBound {
    x: (u32, u32),
    m: (u32, u32),
    a: (u32, u32),
    s: (u32, u32),
}

impl WorkflowManager {
    fn new(wm: HashMap<String, Vec<Rule>>) -> WorkflowManager {
        WorkflowManager {
            workflow_mapping: wm,
        }
    }
    fn accept_input(&self, input: &Input, workflow: String) -> bool {
        if workflow == "A" {
            return true;
        }
        if workflow == "R" {
            return false;
        }
        if !self.workflow_mapping.contains_key(&workflow) {
            panic!("Cannot find rules for workflow: {}", workflow);
        }
        let current_rules = self.workflow_mapping.get(&workflow).unwrap();
        for rule in current_rules {
            if rule.accept(input) {
                return self.accept_input(input, rule.destination.clone());
            }
        }
        panic!("No usable rules found for workflow {}", workflow)
    }
    fn process_input(&self, input_data: &Vec<Input>) -> Vec<u32> {
        let mut ret = Vec::new();
        for input in input_data {
            if self.accept_input(input, String::from("in")) {
                ret.push(input.combined_params());
            }
        }
        ret
    }
    fn find_acceptable_variable_ranges(&self, init_bound: InputBound) -> Vec<InputBound> {
        let mut q: VecDeque<(String, InputBound)> = VecDeque::new();
        let mut acceptable_ranges: Vec<InputBound> = Vec::new();
        q.push_back((String::from("in"), init_bound));
        while let Some((workflow, mut current_range)) = q.pop_front() {
            if !current_range.is_valid() || workflow == "R" {
                continue;
            }
            if workflow == "A" {
                acceptable_ranges.push(current_range);
                continue;
            }
            if !self.workflow_mapping.contains_key(&workflow) {
                panic!("Cannot find rules for workflow: {}", workflow);
            }
            let current_rules = self.workflow_mapping.get(&workflow).unwrap();
            for (i, rule) in current_rules.iter().enumerate() {
                let new_range = rule.determine_new_range(&current_range);
                q.push_back((rule.destination.clone(), new_range));
                if i != current_rules.len() - 1 {
                    current_range = rule.determine_new_range_inverted(&current_range);
                }
            }
        }
        acceptable_ranges
    }
}

impl Rule {
    fn new(raw_rule: &str) -> Rule {
        let is_less_than = raw_rule.find('<').is_some();
        let parsed_raw_rule = raw_rule.split(['<', '>', ':']).collect::<Vec<_>>();
        if parsed_raw_rule.len() == 1 {
            Rule {
                variable: String::from("a"),
                operator: '<',
                value: u32::MAX,
                destination: String::from(parsed_raw_rule[0]),
            }
        } else {
            Rule {
                variable: String::from(parsed_raw_rule[0]),
                operator: if is_less_than { '<' } else { '>' },
                value: parsed_raw_rule[1].parse::<u32>().unwrap(),
                destination: String::from(parsed_raw_rule[2]),
            }
        }
    }
    fn accept(&self, input: &Input) -> bool {
        let verdict_value = match self.variable.as_str() {
            "a" => input.a,
            "x" => input.x,
            "m" => input.m,
            "s" => input.s,
            _ => {
                panic!("Invalid variable found in rule: {}", self.variable)
            }
        };
        match self.operator {
            '<' => verdict_value < self.value,
            '>' => verdict_value > self.value,
            _ => {
                panic!("Invalid operator found in rule: {}", self.operator)
            }
        }
    }
    fn determine_new_range_inverted(&self, old_range: &InputBound) -> InputBound {
        let mut new_range = InputBound {
            x: old_range.x,
            m: old_range.m,
            a: old_range.a,
            s: old_range.s,
        };
        if self.operator == '<' {
            match self.variable.as_str() {
                "x" => new_range.x.0 = new_range.x.0.max(self.value),
                "m" => new_range.m.0 = new_range.m.0.max(self.value),
                "a" => new_range.a.0 = new_range.a.0.max(self.value),
                "s" => new_range.s.0 = new_range.s.0.max(self.value),
                _ => panic!("invalid variable found: {}", self.variable),
            }
        } else if self.operator == '>' {
            match self.variable.as_str() {
                "x" => new_range.x.1 = new_range.x.1.min(self.value),
                "m" => new_range.m.1 = new_range.m.1.min(self.value),
                "a" => new_range.a.1 = new_range.a.1.min(self.value),
                "s" => new_range.s.1 = new_range.s.1.min(self.value),
                _ => panic!("invalid variable found: {}", self.variable),
            };
        } else {
            panic!("Invalid operator found: {}", self.operator);
        }
        new_range
    }

    fn determine_new_range(&self, old_range: &InputBound) -> InputBound {
        let mut new_range = InputBound {
            x: old_range.x,
            m: old_range.m,
            a: old_range.a,
            s: old_range.s,
        };
        if self.operator == '<' {
            match self.variable.as_str() {
                "x" => new_range.x.1 = new_range.x.1.min(self.value - 1),
                "m" => new_range.m.1 = new_range.m.1.min(self.value - 1),
                "a" => new_range.a.1 = new_range.a.1.min(self.value - 1),
                "s" => new_range.s.1 = new_range.s.1.min(self.value - 1),
                _ => panic!("invalid variable found: {}", self.variable),
            };
        } else if self.operator == '>' {
            match self.variable.as_str() {
                "x" => new_range.x.0 = new_range.x.0.max(self.value + 1),
                "m" => new_range.m.0 = new_range.m.0.max(self.value + 1),
                "a" => new_range.a.0 = new_range.a.0.max(self.value + 1),
                "s" => new_range.s.0 = new_range.s.0.max(self.value + 1),
                _ => panic!("invalid variable found: {}", self.variable),
            }
        } else {
            panic!("Invalid operator found: {}", self.operator);
        }
        new_range
    }
}

impl Input {
    fn new(raw_line: &str) -> Input {
        let temp = raw_line
            .split(['{', '}', ','])
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        let mut ret = Input {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };
        for elem in temp {
            let raw_entry = elem.split('=').collect::<Vec<_>>();
            match raw_entry[0] {
                "x" => ret.x = raw_entry[1].parse().unwrap(),
                "m" => ret.m = raw_entry[1].parse().unwrap(),
                "a" => ret.a = raw_entry[1].parse().unwrap(),
                "s" => ret.s = raw_entry[1].parse().unwrap(),
                _ => {
                    panic!("Invalid variable found: {}", raw_entry[0])
                }
            }
        }
        ret
    }
    fn combined_params(&self) -> u32 {
        self.a + self.x + self.m + self.s
    }
}

impl InputBound {
    fn is_valid(&self) -> bool {
        self.x.0 <= self.x.1 && self.m.0 <= self.m.1 && self.a.0 <= self.a.1 && self.s.0 <= self.s.1
    }
    fn count_valid_combinations(&self) -> u64 {
        ((self.x.1 - self.x.0 + 1) as u64)
            * ((self.m.1 - self.m.0 + 1) as u64)
            * ((self.a.1 - self.a.0 + 1) as u64)
            * ((self.s.1 - self.s.0 + 1) as u64)
    }
}

impl Day19 {
    fn parse_workflow_data(raw_data: &str) -> (String, Vec<Rule>) {
        let splitted_data = raw_data.split(['{', '}']).collect::<Vec<_>>();
        (
            String::from(splitted_data[0]),
            splitted_data[1]
                .split(',')
                .map(Rule::new)
                .collect::<Vec<_>>(),
        )
    }
}

impl Solution<u32, u64> for Day19 {
    fn part_one<'a>(lines_it: impl Iterator<Item = &'a str>) -> u32 {
        let lines = lines_it.collect::<Vec<_>>();
        let mut workflow_rules: HashMap<String, Vec<Rule>> = HashMap::new();
        let mut workflow_ended = false;
        let mut input_data: Vec<Input> = Vec::new();
        for line in lines {
            if line.is_empty() {
                workflow_ended = true;
                continue;
            }
            if !workflow_ended {
                let (k, v) = Self::parse_workflow_data(line);
                workflow_rules.insert(k, v);
            } else {
                input_data.push(Input::new(line));
            }
        }
        let workflow_manager = WorkflowManager::new(workflow_rules);
        workflow_manager.process_input(&input_data).iter().sum()
    }
    fn part_two<'a>(lines_it: impl Iterator<Item = &'a str>) -> u64 {
        let lines = lines_it.collect::<Vec<_>>();
        let mut workflow_rules: HashMap<String, Vec<Rule>> = HashMap::new();
        let mut workflow_ended = false;
        let mut input_data: Vec<Input> = Vec::new();
        for line in lines {
            if line.is_empty() {
                workflow_ended = true;
                continue;
            }
            if !workflow_ended {
                let (k, v) = Self::parse_workflow_data(line);
                workflow_rules.insert(k, v);
            } else {
                input_data.push(Input::new(line));
            }
        }
        let workflow_manager = WorkflowManager::new(workflow_rules);
        workflow_manager
            .find_acceptable_variable_ranges(InputBound {
                x: (1, 4000),
                m: (1, 4000),
                a: (1, 4000),
                s: (1, 4000),
            })
            .iter()
            .map(|range| range.count_valid_combinations())
            .sum()
    }
}
