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
    UngueltigerDatentyp(String),
    Testzustand,
}

pub fn print_testdatei() {
    println!("testdatei print aus lib#########>\n{}", TEST_JSON);
}

impl BenutzerAnfrage {
    //schlüssel als globale variablen setzten
    //da keys sich nicht so einfach aus struct isolieren lassen -> weil compiliert
    //einzigste variante -> eigenes macro schreiben!
    //TODO -> macro generieren -> wichtige lektion zum lernen!!!
    pub const KEY_BENUTZER: &'static str = "benutzer";
    pub const KEY_EMAIL: &'static str = "email";
    pub const KEY_ALTER: &'static str = "alter";

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

        //flags setzten
        //erst über felder gehen und danach anhand von flags erst den fehler werfen
        //diese variante gefällt mir garnicht
        //TODO -> bessere logik überlegen
        let mut hat_benutzer_key = false;
        let mut hat_email_key = false;
        let mut hat_alter_key = false;
        //println!("pflichfeldertests");
        let pflichtfelder = json_inhalt.split(",");
        //println!("{pflichtfelder:?}");
        for pf in pflichtfelder {
            //println!("{}", pf);
            if pf.contains(Self::KEY_BENUTZER) {
                hat_benutzer_key = true;
            } else if pf.contains(Self::KEY_EMAIL) {
                hat_email_key = true;
            } else if pf.contains(Self::KEY_ALTER) {
                hat_alter_key = true;
            }
        }

        if !hat_benutzer_key {
            return Err(FehlerValidierung::FeldFehlt(Self::KEY_BENUTZER.to_string()));
        }
        if !hat_email_key {
            return Err(FehlerValidierung::FeldFehlt(Self::KEY_EMAIL.to_string()));
        }
        if !hat_alter_key {
            return Err(FehlerValidierung::FeldFehlt(Self::KEY_ALTER.to_string()));
        }

        //standardfehler während der entwicklung
        Err(FehlerValidierung::Testzustand)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eingabe_von_leerer_datei_wirft_fehler() {
        let ergebnis = BenutzerAnfrage::parse_und_validiere_json("");
        assert_eq!(ergebnis, Err(FehlerValidierung::LeereJsonDatei));
        let ergebnis = BenutzerAnfrage::parse_und_validiere_json("   ");
        assert_eq!(ergebnis, Err(FehlerValidierung::LeereJsonDatei));
    }

    #[test]
    fn test_pruefe_json_auf_oeffnende_schliessende_klammer() {
        let ergebnis = BenutzerAnfrage::parse_und_validiere_json("ohne klammern");
        assert_eq!(ergebnis, Err(FehlerValidierung::FalschesJsonFormat));
    }

    #[test]
    fn test_inhalt_der_json_ist_unfug() {
        let ergebnis = BenutzerAnfrage::parse_und_validiere_json("{+da5-i5t_k@uderwel$ch~}");
        assert_eq!(ergebnis, Err(FehlerValidierung::KaputteJson));
    }

    #[test]
    fn test_fehlendes_pflichtfeld_benutzer() {
        let json = r#"{
            "email": "foobar@rab.dd",
            "alter": 23
        }"#;
        let ergebnis = BenutzerAnfrage::parse_und_validiere_json(json);
        assert_eq!(
            ergebnis,
            Err(FehlerValidierung::FeldFehlt("benutzer".to_string()))
        );
    }

    #[test]
    fn test_fehlendes_pflichtfeld_email() {
        let json = r#"{
            "benutzer": "rainer_zufall",
            "alter": 23
        }"#;
        let ergebnis = BenutzerAnfrage::parse_und_validiere_json(json);
        assert_eq!(
            ergebnis,
            Err(FehlerValidierung::FeldFehlt("email".to_string()))
        );
    }

    #[test]
    fn test_fehlendes_pflichtfeld_alter() {
        let json = r#"{
            "email": "foobar@rab.dd",
            "benutzer": "rainer_zufall" 
        }"#;
        let ergebnis = BenutzerAnfrage::parse_und_validiere_json(json);
        assert_eq!(
            ergebnis,
            Err(FehlerValidierung::FeldFehlt("alter".to_string()))
        );
    }

    #[test]
    fn test_alter_hat_falschen_datentyp_string() {
        let test_json = r#"{
            "email": "foobar@rab.dd",
            "benutzer": "rainer_zufall", 
            "alter": "drei-und-zwanzig" 
        }"#;
        let ergebnis = BenutzerAnfrage::parse_und_validiere_json(test_json);
        assert_eq!(
            ergebnis,
            Err(FehlerValidierung::UngueltigerDatentyp("alter".to_string()))
        );
    }

    #[test]
    fn test_email_ohne_at_gibt_fehler() {
        let test_json = r#"{
            "email": "foobarrab.dd",
            "benutzer": "rainer_zufall", 
            "alter": "23" 
        }"#;
        let ergebnis = BenutzerAnfrage::parse_und_validiere_json(test_json);
        assert_eq!(ergebnis, Err(FehlerValidierung::EmailUngueltig));
    }

    #[test]
    fn test_email_ohne_punkt_gibt_fehler() {
        let test_json = r#"{
            "email": "foobar@rabdd",
            "benutzer": "rainer_zufall", 
            "alter": "23" 
        }"#;
        let ergebnis = BenutzerAnfrage::parse_und_validiere_json(test_json);
        assert_eq!(ergebnis, Err(FehlerValidierung::EmailUngueltig));
    }
}
