# 🧪 Test-Driven JSON Parser & Validator in Rust

Ein robuster, komplett **testgetrieben (TDD)** entwickelter JSON-Parser und Validator in Rust. Das Projekt wurde ohne externe Parsing-Bibliotheken implementiert, um das Zusammenspiel von sicherem Speicher-Handling (`&str`), der Transformation von Daten und striktem Fehler-Design in Rust zu meistern.

---
## 🎓 Projekt-Kontext & Lernziel

Dieses Projekt ist **eines meiner ersten Projekte in der Programmiersprache Rust**. Es ist bewusst im Rahmen eines intensiven Lernprozesses entstanden. 

* **Bewusster Verzicht auf Crates (Bibliotheken):** In produktiven Rust-Anwendungen gehört der Einsatz von `serde` und `serde_json` zum absoluten Standard. Für dieses Projekt habe ich **bewusst auf jegliche externen Bibliotheken verzichtet**. 
* **Das Ziel:** Ich wollte mich nicht auf fertige Magie verlassen, sondern die Kernkonzepte von Rust von Grund auf erzwingen und tiefgründig verstehen. Dazu gehören das strikte Ownership-Modell, das Arbeiten mit String-Slices (`&str`), Speicherallokation, die Fehlerbehandlung via `Result` und `Match` sowie das Schreiben von sauberen Unit-Tests.
---

## 🎯 Der TDD-Entwicklungszyklus (Red-Green-Refactor)

Dieses Projekt folgt strikt dem TDD-Paradigma. Jedes Feature und jede Validierungsregel wurde nach dem klassischen Dreischritt aufgebaut:
1. ***commits -> test:*** **🔴 Red:** Schreiben eines fehlschlagenden Integrationstests (z. B. für unvollständige Datentypen oder Altersgrenzen). 
2. ***commits -> feat:*** **🟢 Green:** Implementierung des minimal notwendigen Produktionscodes, bis `cargo test` erfolgreich durchläuft. 
3. ***commits -> refactor:*** **🔵 Refactor:** Modularisierung und Auslagerung des Codes in isolierte Prüffunktionen, abgesichert durch die bestehende Test-Suite.

Dank dieses Ansatzes ist die Pipeline zu 100 % regressionstestgesichert.

---

## 🚀 Die Validierungs-Pipeline

Der Eingabe-String durchläuft vier isolierte, testgesicherte Phasen, bevor er in die Zielstruktur überführt wird:

```text
  pruefe_basis_json_struktur()  ➔  Validiert {}, leere Eingaben & Hochkomma-Anzahl (Garantierte Basis)
               │
               ▼
  pruefe_pflichtfelder()        ➔  Schlägt fehl, wenn 'benutzer', 'email' oder 'alter' fehlen
               │
               ▼
  pruefe_email()                ➔  Validiert die formale Syntax der E-Mail (@ und .)
               │
               ▼
  pruefe_alter()                ➔  Isoliert das Alter, prüft Datentyp und erzwingt Altersgrenze (>= 18)
               │
               ▼
  [Zuweisung & Struct-Wandlung] ➔  Extraktion der finalen Werte via Whitespace- & Tabulator-Bereinigung
```
---

## 📋 Strikte Typisierung & Fehler-Katalog

### Die Zielstruktur
```rust
#[derive(Debug, PartialEq)]
pub struct BenutzerAnfrage {
    pub benutzer: String,
    pub email: String,
    pub alter: u32,
}
```

### Das Fehler-Enum (`FehlerValidierung`)
Der Parser nutzt das `Result`-Muster von Rust und wirft zu keinem Zeitpunkt Laufzeitfehler (`panic!`). Fehler werden präzise aufgeschlüsselt:
* `LeereJsonDatei` — Keine Daten übergeben.
* `FalschesJsonFormat` — Fehlerhafte äußere Struktur (z. B. fehlende `{}`).
* `KaputteJson` — Unausgeglichene Anführungszeichen.
* `FeldFehlt(String)` — Ein spezifisches Pflichtfeld fehlt im JSON.
* `EmailUngueltig` — E-Mail-Validierung fehlgeschlagen.
* `AlterZuJung` — Nutzer erfüllt die Altersgrenze von 18 Jahren nicht.
* `UngueltigerDatentyp(String)` — Wenn z. B. `"alter": "drei-und-zwanzig"` statt einer Zahl übergeben wird.

---

## 🛠️ Ausführung & Testumgebung

Da der Parser nativ geschrieben ist, benötigst du keine externen Crates.

### Tests ausführen (Der TDD-Kompass)
Lass alle geschriebenen Tests laufen, um die Integrität des Parsers zu prüfen:
```bash
cargo test
```
---
