use core::num::ParseIntError;
use serde_json::{self, json};
use jq_rs;
use regex::Regex;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Op {
    Eq,
    NotEq,
    GreaterThan,
    SmallerThan
} 

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Rule {
    json_element: String,
    op: Op,
    value: String,
}

impl Rule {
    pub fn parse(rule: &str) -> Result<Rule, &'static str> {
        let re = Regex::new(r"((\.{0,1}[a-z0-9\[\]]{1,}){0,}) (eq|!eq|>|<) ([0-9a-z ]{1,})")
                        .unwrap();
        
        let groups = re.captures(rule).unwrap();

        let op = match &groups[3] {
            "eq" => Op::Eq,
            "!eq" => Op::NotEq,
            ">" => Op::GreaterThan,
            "<" => Op::SmallerThan,
            _ => panic!("Not valid Operation")
        };
        
        let parsed_rule = Rule {
            json_element: groups[1].to_string(),
            op: op,
            value: groups[4].to_string(),
        };
        return match Rule::validate_rule(&parsed_rule) {
            Ok(_) => Ok(parsed_rule),
            Err(_) => Err("Operation does not suppot value passed"),
        };
    }


    // arg, this is terrible, forcing the convertion 
    // just to double check > and < are used correctly
    // arg, terrible, terrible
    fn validate_rule(rule: &Rule) -> Result<i32, ParseIntError> {
        return match rule.op {
            Op::SmallerThan => rule.value.parse::<i32>(),
            Op::GreaterThan => rule.value.parse::<i32>(),
            _ => Ok(0)
        };
    }

    pub fn should_exec(self: &Self, event: &str) -> bool {
        let mut event_value = str::replace(&self.walk_json(event).unwrap(), "\"", "");
        event_value = event_value.trim().to_string();
        
        return match self.op {
            Op::Eq => event_value == self.value,
            Op::NotEq => event_value != self.value,
            _ => self.check_op_numeric_condition(&event_value),
        };
    }

    fn check_op_numeric_condition(self: &Self, value: &str) -> bool {

        if let Ok(numeric_value) = value.parse::<i32>() {

            return match self.op {
                Op::SmallerThan => numeric_value < self.value.parse::<i32>().unwrap(),
                Op::GreaterThan => numeric_value > self.value.parse::<i32>().unwrap(),
                _ => false

            };
        } else {
            return false;
        }
    }

    fn walk_json(self: &Self, json: &str) -> Result<String, jq_rs::Error> {
        return jq_rs::run(&self.json_element, json);
    }

}

#[cfg(test)]
mod tests {
    use crate::rule::Rule;
    use crate::rule::Op;
    
    #[test]
    fn parse_rule() {
        let r = Rule::parse(".event.type eq success").unwrap();
        let expected = Rule { json_element: ".event.type".to_string(), op: Op::Eq, value: "success".to_string() };
        
        assert_eq!(expected, r);
    }

    #[test]
    fn should_exec() {
        let r = Rule::parse(".event.type eq success").unwrap();
        let json = "{\"event\": {\"type\": \"success\"}}";

        assert!(r.should_exec(json));

        let r = Rule::parse(".event.type !eq success").unwrap();
        let json = "{\"event\": {\"type\": \"false\"}}";

        assert!(r.should_exec(json));

        let r = Rule::parse(".event.id > 1").unwrap();
        let json = "{\"event\": {\"id\": \"5\"}}";

        assert!(r.should_exec(json));

        let r = Rule::parse(".event.id < 5").unwrap();
        let json = "{\"event\": {\"id\": \"1\"}}";

        assert!(r.should_exec(json));
    }
}