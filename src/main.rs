// eigene lib einbinden
use json_parser::*;

fn main() {
    println!("ahoi zum tdd-projekt");
    // einbinden einer gültigen test-JSON
    let json_string = include_str!("tests/fixtures/valid.json");

    // aufruf einer funktion aus der lib
    //print_testdatei();

    let error_wenn_leer = BenutzerAnfrage::parse_und_validiere_json("");
    println!("result bei leerer json-datei----->\n{:#?}", error_wenn_leer);

    //ausgaben zum debuggen
    let ergebnis_struktur = BenutzerAnfrage::parse_und_validiere_json(json_string);

    println!("####################################################################");
    println!("#############----- FINALE -----#####################################");
    println!("####################################################################");
    println!("{:#?}", ergebnis_struktur);
}
