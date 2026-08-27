use json_parser::*;

fn main() {
    println!("ahoi zum tdd-projekt");
    let json_string = include_str!("tests/fixtures/valid.json");

    println!("testdatei JSON----->\n{}", json_string);

    print_testdatei();
    
    let error_wenn_leer = parse_und_validiere_json("");
    println!("result bei leerer json-datei----->\n{:#?}", error_wenn_leer);

    //ausgaben zum debuggen
    let _ = parse_und_validiere_json(json_string);
}
