//json testdatei global einbinden
pub const TEST_JSON: &str = include_str!("tests/fixtures/valid.json");

pub fn print_testdatei() {
    println!("testdatei print aus lib#########>\n{}", TEST_JSON);
}

#[cfg(test)]
mod tests {
    use super::*;

}
