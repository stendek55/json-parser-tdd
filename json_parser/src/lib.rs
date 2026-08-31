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

    pub fn pruefe_basis_json_struktur(eingabe: &str) -> Result<&str, FehlerValidierung> {
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

        Ok(json_string)
    }

    pub fn pruefe_pflichtfelder(eingabe: &str) -> Result<&str, FehlerValidierung> {
        //flags setzten
        //erst über felder gehen und danach anhand von flags erst den fehler werfen
        //diese variante gefällt mir garnicht
        //TODO -> bessere logik überlegen
        let mut hat_benutzer_key = false;
        let mut hat_email_key = false;
        let mut hat_alter_key = false;
        //println!("pflichfeldertests");
        let pflichtfelder = eingabe.split(",");
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

        Ok(eingabe)
    }

    pub fn pruefe_email(eingabe: &str) -> Result<&str, FehlerValidierung> {
        // email auf @ und . prüfen
        let email_muster = format!("\"{}\"", Self::KEY_EMAIL);

        if let Some(key_pos) = eingabe.find(&email_muster) {
            let text_ab_email = &eingabe[key_pos..];

            if let Some(doppelpunkt_pos) = text_ab_email.find(':') {
                let text_ab_wert = &text_ab_email[doppelpunkt_pos + 1..];
                let bereinigter_text = text_ab_wert.trim_start();

                // prüfen ob mit " beginnt UND schneidet es gleichzeitig ab!
                if let Some(email_inhalt_start) = bereinigter_text.strip_prefix('"') {
                    // suchen das schließende anführungszeichen der mail-adresse
                    if let Some(ende_pos) = email_inhalt_start.find('"') {
                        // die reine mail-adresse
                        let email_adresse = &email_inhalt_start[..ende_pos];

                        // prüfen '@'
                        if !email_adresse.contains('@') {
                            return Err(FehlerValidierung::EmailUngueltig);
                        }

                        // prüfen auf '.'
                        if !email_adresse.contains('.') {
                            return Err(FehlerValidierung::EmailUngueltig);
                        }
                    }
                }
            }
        }

        Ok(eingabe)
    }

    pub fn pruefe_alter(eingabe: &str) -> Result<u32, FehlerValidierung> {
        let mut gutes_alter: u32 = 0;
        // datentyp vom alter prüfen
        // das ergibt genau den text: "alter"
        let such_muster = format!("\"{}\"", Self::KEY_ALTER);

        // suchen nach diesem key im JSON-string
        if let Some(key_pos) = eingabe.find(&such_muster) {
            // alles abschneiden was VOR dem suchmuster kam
            let text_ab_alter = &eingabe[key_pos..];

            // suchen den doppelpunkt der nach dem "alter" kommt
            if let Some(doppelpunkt_pos) = text_ab_alter.find(':') {
                // schneidet alles vor dem doppelpunkt ab
                let text_ab_wert = &text_ab_alter[doppelpunkt_pos + 1..];

                if let Some(zeilenumbruch_pos) = text_ab_wert.find("\n") {
                    // nimmt alles bis zum zeilenumbruch
                    let alter_wert = &text_ab_wert[..zeilenumbruch_pos];
                    // entscheidung hier für die match version
                    // anstatt des kürzeren if let (wo nur erfolögsfall)
                    // weil so auch der fehlerfall zur verfügung steht und nochmals genauer auf
                    // typmatching geschautz werden kann
                    match alter_wert.trim().parse::<u32>() {
                        Ok(alter) => {
                            println!("alter korrekter wert u8 und ist {}", alter);
                            if alter < 18 {
                                return Err(FehlerValidierung::AlterZuJung);
                            }
                            gutes_alter = alter;
                        }
                        Err(_) => {
                            println!("gein gültiger u8 wert");
                            return Err(FehlerValidierung::UngueltigerDatentyp(
                                Self::KEY_ALTER.to_string(),
                            ));
                        }
                    }
                }

                // entfernt leerzeichen und zeilenumbrüche am anfang
                let bereinigter_text = text_ab_wert.trim_start();

                // typ-PRÜFUNG
                // wenn der wert mit anführungszeichen " beginnt ist es ein string
                if bereinigter_text.starts_with('"') {
                    // fehlermeldung rausgebene
                    return Err(FehlerValidierung::UngueltigerDatentyp(
                        Self::KEY_ALTER.to_string(),
                    ));
                }
            }
        }

        Ok(gutes_alter)
    }
    //hauptfunktion die entwickelt werden soll
    pub fn parse_und_validiere_json(eingabe: &str) -> Result<BenutzerAnfrage, FehlerValidierung> {
        let json_inhalt = Self::pruefe_basis_json_struktur(eingabe)?;
        let json_mit_feldern = Self::pruefe_pflichtfelder(json_inhalt)?;
        let json_ok = Self::pruefe_email(json_mit_feldern)?;
        let alter_ok = Self::pruefe_alter(json_ok)?;

        let mut extrahierter_benutzer = String::new();
        let mut extrahierte_email = String::new();
        let extrahiertes_alter = alter_ok;

        // über geprüft korrektes json gehen und final die benötigten werte auslösen
        for feld in json_ok.split(',') {
            if let Some((key, val)) = feld.split_once(':') {
                // zur sicherheit explizit alles angeben was entfernt werden soll
                let zeichen_entfernen = &['"', '{', '}', '\n', '\t', '\r', ' '][..];
                let sauberer_key = key.trim().trim_matches(zeichen_entfernen);
                let sauberer_wert = val.trim().trim_matches(zeichen_entfernen);

                match sauberer_key {
                    Self::KEY_BENUTZER => extrahierter_benutzer = sauberer_wert.to_string(),
                    Self::KEY_EMAIL => extrahierte_email = sauberer_wert.to_string(),
                    _ => {}
                }
            }
        }

        // final das struct befüllen und zurückgeben
        Ok(Self {
            benutzer: extrahierter_benutzer,
            email: extrahierte_email,
            alter: extrahiertes_alter,
        })
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

    #[test]
    fn test_gibt_fehler_wenn_alter_zu_jung_unter_18() {
        let test_json = r#"{
            "email": "unter18@zujung.de",
            "benutzer": "rainer_zufall", 
            "alter": 17 
        }"#;
        let ergebnis = BenutzerAnfrage::parse_und_validiere_json(test_json);
        assert_eq!(ergebnis, Err(FehlerValidierung::AlterZuJung));
    }

    #[test]
    fn test_gibt_fehler_wenn_alter_zu_jung_extremfall_0() {
        let test_json = r#"{
            "email": "baby@schlaf.de",
            "benutzer": "rainer_zufall", 
            "alter": 0 
        }"#;
        let ergebnis = BenutzerAnfrage::parse_und_validiere_json(test_json);
        assert_eq!(ergebnis, Err(FehlerValidierung::AlterZuJung));
    }

    #[test]
    fn test_gibt_fehler_wenn_alter_dezimalzahl() {
        let test_json = r#"{
            "email": "ahoi@welt.dd",
            "benutzer": "rainer_zufall", 
            "alter": 23.42 
        }"#;
        let ergebnis = BenutzerAnfrage::parse_und_validiere_json(test_json);
        assert_eq!(
            ergebnis,
            Err(FehlerValidierung::UngueltigerDatentyp("alter".to_string()))
        );
    }
}
