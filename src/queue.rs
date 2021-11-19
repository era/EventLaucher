
use std::collections::HashMap;
extern crate yaml_rust;
use yaml_rust::{YamlLoader, Yaml};
use std::fs;

use crate::rule;

#[derive(Debug, Clone)]
pub struct Consumer {
    pub rule: rule::Rule,
    pub exec: String,
    queue_name: String
}


pub fn from_file(file: &str) -> Option<HashMap<String, Vec<Consumer>>> {
    let yaml = fs::read_to_string(file).ok()?;
    return from_str(&yaml);
}

fn from_str(yaml: &str) -> Option<HashMap<String, Vec<Consumer>>> {
    let mut consumers: HashMap<String, Vec<Consumer>> = HashMap::new();

    let config = parse_yaml(yaml).into_iter().next().unwrap(); // wroooong

    for k in config[0].as_hash().iter() {
        for (key, entries) in k.iter() {
            let mut vec = Vec::<Consumer>::new();

            let (key, entries) = match (key, entries) {
                (Yaml::String(key), Yaml::Array(entries)) => (key, entries),
                (_, _) => return None
            };

            for item in entries {

                let (rule, exec) = match (&item["rule"], &item["exec"]) {
                    (Yaml::String(rule), Yaml::String(exec)) => (rule, exec),
                    (_, _) => return None
                };

                vec.push(Consumer {
                    rule: rule::Rule::parse(&rule).unwrap(), // WROONG
                    exec: exec.to_string(),
                    queue_name: key.to_string(),
                });
            }
            consumers.insert(key.to_string(), vec);
        }
    }
    return Some(consumers);
}

fn parse_yaml(yaml: &str) -> Option<Vec<Yaml>> {
    if let Ok(result) = yaml_rust::YamlLoader::load_from_str(yaml) {
        return Some(result);
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::queue;
    use crate::rule;
    use std::collections::HashMap;

    #[test]
    fn parse_yaml() {
        let s = r#"
my_cool_queue:
    -
        rule: ".event.type eq error"
        exec: "python3 slack_message.py ${event}"
    -
        rule: ".event.type eq success"
        exec: "python3 send_email_boss.py ${event}"

another_queue:
    -
        rule: ".event.type eq error"
        exec: "python3 slack_message.py ${event}"
    -
        rule: ".event.type eq success"
        exec: "python3 send_email_boss.py ${event}"
            "#;
        
    
        if let Some(config) = queue::from_str(s) {
            if let Some(queue) = config.get("my_cool_queue") {
                assert_eq!(2, queue.len());
                if let Some(item) = queue.get(0) {
                    assert_eq!("python3 slack_message.py ${event}", item.exec);
                    assert_eq!(rule::Rule::parse(".event.type eq error").unwrap(), item.rule);
                } else {
                    assert!(false);
                }
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }

}