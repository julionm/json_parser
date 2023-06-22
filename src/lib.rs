#![allow(dead_code)]

use std::collections::HashMap;

#[derive(PartialEq, Debug)]
enum JsonValue<'a> {
    Array {
        values: Vec<JsonValue<'a>>
    },
    Number(String),
    Object(HashMap<String, JsonValue<'a>>),
    Text(&'a str),
    Char(char),
    Boolean(bool),
    Null,
    Undefined,
    String(String)
}

struct JsonNode<'a> {
    value: Option<JsonValue<'a>>,
}

fn parse_object(_json: &str) -> (JsonValue, Option<&str>) {
    unimplemented!("parse_object function not implemented yet")
}

fn parse_array(_json: &str) -> (JsonValue, Option<&str>) {
    unimplemented!("parse_array function not implemented yet")
}

fn parse_string(json: &str) -> (JsonValue, Option<&str>) {

    let mut chars_iter = json.chars();

    let mut idx: usize = 0;

    while let Some(ch) = chars_iter.next() {
        if ch == '"' {
           break; 
        }
        idx += 1;
    }

    println!("{}", json);
    println!("{}", idx);

    (JsonValue::String(String::from(&json[..idx])), Some(&json[idx+1..]))
}

fn parse_numeric(json: &str) -> (JsonValue, Option<&str>) {

    let mut chars_iter = json.chars();
    let mut idx: usize = 0;

    while let Some(ch) = chars_iter.next() {
        if char_is_numeric(&ch) {
            idx += 1;
        } else {
            break;
        }
    }

    (
        JsonValue::Number(String::from(&json[..idx])),
        if idx == json.len() {
            None
        } else {
            Some(&json[idx+1..])
        }
    )
}

fn char_is_numeric(c: &char) -> bool {
    match *c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
        _ => false
    }
}

fn str_is_numeric(c: &&str) -> bool {
    match *c {
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => true,
        _ => false
    }
}

fn parse(json: &str) -> (JsonValue, Option<&str>) {

    let first_char = json.get(..1);

    match first_char {
        Some("{") => parse_object(json),
        Some("[") => parse_array(json),

        // * WORKS!!
        Some("\"") => parse_string(&json[1..]),

        Some(ch) if str_is_numeric(&ch) => parse_numeric(json),

        None => (JsonValue::Null, None),

        // ! this'll change
        Some(ch) => (JsonValue::String(ch.into()), json.get(1..))
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_number() {
        let s = "12345";

        assert_eq!(
            parse(s),
            (JsonValue::Number("12345".into()), None)
        )
    }

    #[test]
    fn parse_string() {

        let s = "\"teste\"";

        assert_eq!(
            parse(s),
            (JsonValue::String(String::from("teste")), Some(""))
        );
    }

    #[test]
    fn parse_string_with_numbers() {
        let s = "\"12teste12\"";

        assert_eq!(
            parse(s),
            (JsonValue::String(String::from("12teste12")), Some(""))
        );
    }

    #[test]
    fn parse_array() {
        let s = "[1, 2, 3, 4]";

        assert_eq!(
            parse(s),
            (
                JsonValue::Array { 
                    values: vec![
                        JsonValue::Number("1".into()),
                        JsonValue::Number("2".into()),
                        JsonValue::Number("3".into()),
                        JsonValue::Number("4".into())
                    ]
                },
                Some("")
            )
        )
    }

}