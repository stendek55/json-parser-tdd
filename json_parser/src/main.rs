use json_parser::*;

fn main() {
    println!("ahoi zum tdd-projekt");
    let json_string = include_str!("tests/fixtures/valid.json");

    println!("testdatei JSON----->\n{}", json_string);

    print_testdatei();
}
