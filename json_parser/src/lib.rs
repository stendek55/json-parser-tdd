//json testdatei global einbinden
pub const TEST_JSON: &str = include_str!("tests/fixtures/valid.json");

//zielstruktur für eine gültige json
#[derive(Debug, PartialEq)]
pub struct BenutzerAnfrage {
    pub benutzer: String,
    pub email: String,
    pub alter: u32,
}

//mögliche fehler die bei wandlung eintreten könn
#[derive(Debug, PartialEq)]
pub enum FehlerValidierung {
    LeereJsonDatei,
    FalschesJsonFormat,
    KaputteJson,
    FeldFehlt(String),
    EmailUngueltig,
    AlterZuJung,
    Testzustand,
}

pub fn print_testdatei() {
    println!("testdatei print aus lib#########>\n{}", TEST_JSON);
}

//hauptfunktion die entwickelt werden soll
pub fn parse_und_validiere_json(eingabe: &str) -> Result<BenutzerAnfrage, FehlerValidierung> {
    //entfernt whitespaces und steuerzeichen
    let json_string = eingabe.trim();

    //prüfen ob inhalt in datei existiert
    if json_string.is_empty() {
        return Err(FehlerValidierung::LeereJsonDatei);
    } 

    //gibt es öffnende und abschließende klammern {}
    if !json_string.starts_with("{") || !json_string.ends_with("}") {
        return Err(FehlerValidierung::FalschesJsonFormat);
    }

    //inhalt zwischen { und } herauslösen
    let json_inhalt = &json_string[1..&json_string.len() - 1].trim();
    //sind die anführungszeichen ausgeglichen -> gerade anzahl von "
    let anzahl_hochkommas = json_inhalt.chars().filter(|&c| c == '"').count();
    //println!("summe hochkommas:{anzahl_hochkommas}");
    //inhalt muss mit " beginnen -> name für ersten key
    if anzahl_hochkommas % 2 != 0 || !json_inhalt.starts_with('"') {
        return Err(FehlerValidierung::KaputteJson);
    }


    //standardfehler während der entwicklung
    Err(FehlerValidierung::Testzustand)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eingabe_von_leerer_datei_wirft_fehler() {
        let ergebnis = parse_und_validiere_json("");
        assert_eq!(ergebnis, Err(FehlerValidierung::LeereJsonDatei));
        let ergebnis = parse_und_validiere_json("   ");
        assert_eq!(ergebnis, Err(FehlerValidierung::LeereJsonDatei));
    }

    #[test]
    fn test_pruefe_json_auf_oeffnende_schliessende_klammer() {
        let ergebnis = parse_und_validiere_json("ohne klammern");
        assert_eq!(ergebnis, Err(FehlerValidierung::FalschesJsonFormat));
    }

    #[test]
    fn test_inhalt_der_json_ist_unfug() {
        let ergebnis = parse_und_validiere_json("{+da5-i5t_k@uderwel$ch~}");
        assert_eq!(ergebnis, Err(FehlerValidierung::KaputteJson));
    }
}
