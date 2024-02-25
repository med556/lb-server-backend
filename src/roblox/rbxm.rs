use std::collections::HashMap;
use std::io::{Cursor, BufReader};
use rbx_binary;
use rbx_dom_weak::{WeakDom, Instance};
use rbx_types::Variant;
use crate::Backend;

fn search_for_classnames<'a>(dom: &'a WeakDom, classnames: &Vec<&str>, instances: &mut HashMap<Vec<&'a str>, &'a Instance>, mut names: Vec<&'a str>, instance: &'a Instance) {
    names.push(instance.name.as_str());
    for &child_ref in instance.children() {
        if let Some(instance) =  dom.get_by_ref(child_ref) {
            if classnames.contains(&instance.class.as_str()) {
                instances.insert(names.clone(), &instance);
            }

            search_for_classnames(dom, classnames, instances, names.clone(), &instance);
        }
    }
}

impl Backend {
    pub fn dom_from_bytes(&self, bytes: Vec<u8>) -> Result<WeakDom, Box<dyn std::error::Error>> {
        let cursor = Cursor::new(bytes);
        let buf_reader = BufReader::new(cursor);

        Ok(rbx_binary::from_reader(buf_reader)?)
    }

    pub fn dom_find_scripts<'a>(&'a self, dom: &'a WeakDom) -> HashMap<String, String> {
        let mut scripts: HashMap<String, String> = HashMap::new();

        let classnames: Vec<&str> = vec!["Script", "LocalScript", "ModuleScript"];
        let mut instances: HashMap<Vec<&str>, &Instance> = HashMap::new();
        for &instance_ref in dom.root().children() {
            if let Some(instance) = &dom.get_by_ref(instance_ref) {
                search_for_classnames(&dom, &classnames, &mut instances, Vec::new(), instance);
            }
        }

        for e in instances {
            let source = e.1.properties.get("Source").unwrap();
            match source {
                Variant::String(src) => {
                    let path = e.0.join(".");
                    scripts.insert(path, src.to_string());
                },
                _ => {}
            };
        }

        scripts
    }
}