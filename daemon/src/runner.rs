use std::collections::HashMap;

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
                panic!("unexpected non-group expression");
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
            let temp = &input[offset..];
            res.append(&mut temp.to_vec());
        }

        res
    }
}

