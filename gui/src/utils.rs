#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstructionsHex {
    bytes_as_hex: Vec<String>,
}

impl From<&[u8]> for InstructionsHex {
    fn from(bytes: &[u8]) -> Self {
        let bytes_as_hex = bytes.iter().map(|b| format!("{b:02X}")).collect();
        Self { bytes_as_hex }
    }
}

impl std::fmt::Display for InstructionsHex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.bytes_as_hex.join(" "))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_bytes() {
        let bytes: &[u8] = &[222, 173, 190, 239];
        let insns_hex = InstructionsHex::from(bytes);

        assert_eq!(
            vec![
                String::from("DE"),
                String::from("AD"),
                String::from("BE"),
                String::from("EF"),
            ],
            insns_hex.bytes_as_hex
        );
    }
}
