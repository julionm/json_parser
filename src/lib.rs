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

fn parse_object(json: &str) -> (JsonValue, Option<&str>) {
    (JsonValue::Null, Some(json))
}

fn parse_array(json: &str) -> (JsonValue, Option<&str>) {
    (JsonValue::Null, Some(json))
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

    (JsonValue::String(String::from(&json[..idx])), Some(&json[idx+1..]))
}

fn parse(json: &str) -> (JsonValue, Option<&str>) {

    let first_char = json.get(..1);

    match first_char {
        Some("{") => parse_object(json),
        Some("[") => parse_array(json),
        Some("\"") => parse_string(&json[1..]),
        None => (JsonValue::Null, None),
        Some(ch) => (JsonValue::String(ch.into()), json.get(1..))
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_number() {
        // let res = JsonValue::Char()


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
    fn parse_array() {
        let s = "[1, 2, 3, 4]";

        assert_eq!(
            parse(s),
            (JsonValue::Array { values: vec![1, 2, 3, 4] }, Some(""))
        )
    }

}