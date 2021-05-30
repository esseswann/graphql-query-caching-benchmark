use graphql_parser::query::{parse_query, Document};

fn main() {
    println!("Hello, world!");
}

pub fn parse(query: &'static str) -> Document<'static, &'static str> {
    parse_query::<&str>(query).unwrap()
}

pub fn cached_parse(query: &'static str) -> Document<'static, &'static str> {
    parse_query::<&str>(query).unwrap()
}