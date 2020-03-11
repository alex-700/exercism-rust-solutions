use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

type OpType = dyn Fn(i32, i32) -> i32 + Sync;

lazy_static! {
    static ref NAME_TO_OP: HashMap<&'static str, Box<OpType>> = {
        let mut ans = HashMap::new();
        ans.insert("plus", Box::new(|a, b| a + b) as Box<OpType>);
        ans.insert("minus", Box::new(|a, b| a - b) as Box<OpType>);
        ans.insert("divided by", Box::new(|a, b| a / b) as Box<OpType>);
        ans.insert("multiplied by", Box::new(|a, b| a * b) as Box<OpType>);
        ans
    };
    static ref ALL_OPS: String = {
        let mut ans = String::new();
        for x in NAME_TO_OP.keys() {
            if !ans.is_empty() {
                ans.push('|');
            }
            ans.push_str(x);
        }
        ans
    };
    static ref MAIN_REGEX: Regex = Regex::new(&format!(
        r"^What is (-?\d+)((?: (?:{}) (?:-?\d+))*)\?$",
        *ALL_OPS
    ))
    .unwrap();
    static ref OPS_REGEX: Regex =
        Regex::new(&format!(r"(?P<operation>{}) (?P<operand>-?\d+)", *ALL_OPS)).unwrap();
}

pub fn answer(command: &str) -> Option<i32> {
    MAIN_REGEX.captures(command).map(|c| {
        let first_number = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let operations = c.get(2).unwrap();
        OPS_REGEX
            .captures_iter(operations.as_str())
            .fold(first_number, |acc, caps| {
                let operand = caps
                    .name("operand")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
                NAME_TO_OP
                    .get(caps.name("operation").unwrap().as_str())
                    .unwrap()(acc, operand)
            })
    })
}
