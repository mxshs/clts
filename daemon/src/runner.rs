use std::collections::HashMap;
use std::io::Write;

use super::parser::{
    Node,
    Pair,
};

pub struct Runner {
    meta: HashMap<String, String>,
    mapping: HashMap<String, HashMap<String, String>>
}

impl Runner {
    pub fn new(nodes: Vec<Node>) -> Runner {
        let mut runner = Runner{
            meta: HashMap::new(),
            mapping: HashMap::new(),
        };

        for node in nodes {
            if let Node::Group(group) = node {
                for mapping in group.pairs {
                    runner.add_mapping(&group.name, mapping);
                }
            } else {
                panic!("unexpected non-group node");
            }
        }

        runner
    }

    fn add_mapping(&mut self, group_name: &String, mapping: Pair) {
        self.meta.insert(mapping.from.clone(), group_name.clone());

        if self.mapping.get_mut(group_name).is_none() {
            self.mapping.insert(group_name.clone(), HashMap::new());
        }
            
        self.mapping.get_mut(group_name).unwrap().insert(mapping.from, mapping.to);
    }

    pub fn map(&self, input: Vec<String>) -> Vec<String> {
        assert!(!input.is_empty());

        let group = &self.meta[input.first().unwrap()];

        let map = &self.mapping[group];

        let mut res = Vec::new();

        let mut offset = 0;
        
        while offset < input.len() {
            if let Some(val) = map.get(input.get(offset).unwrap()) {
                res.push(val.to_string());
                offset += 1;
            } else {
                break
            }
        }

        if offset < input.len() {
            res.append(&mut input[offset..].to_vec());
        }

        res
    }

    pub fn write(&self, path: &str) {
        let mut f = std::fs::OpenOptions::new().append(true).open(path).unwrap();
        let mut aliases = Vec::new();
        let mut expansions = Vec::new();

        for group in &self.mapping {
            let al = self.convert(Vec::new(), group.0);
            aliases.extend_from_slice(&al);

            for seq in al {
                println!("{}", seq.concat());
                if seq.len() > 0 {
                    expansions.push(self.map(seq))
                }
            }
        }

        let mut offset = 0;
        for idx in 0..(aliases.len() - 1) {
            if !aliases[idx].is_empty() {
                f.write_all(format!("alias {} {}", aliases[idx].concat(), expansions[offset].concat()).as_bytes()).unwrap();
                offset += 1;
            }
        }
    }

    fn convert(&self, base: Vec<String>, group_name: &String) -> Vec<Vec<String>> {
        let mut aliases = Vec::new();
        aliases.push(base.clone());

        for char in self.mapping[group_name].clone() {
            if !base.contains(&char.0) {
                let mut temp = base.clone();
                temp.push(char.0);
                let al = self.convert(temp, group_name);
                aliases.extend(al);
            }
        }

        aliases
    }
}

