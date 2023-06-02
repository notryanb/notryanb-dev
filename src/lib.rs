pub fn parse_num(num: &str) -> Result<u32, NumParseError> {
    if num.len() <= 1 {
        return *&num[..].parse::<u32>().map_err(|_| NumParseError::InvalidDecimal);
    }

    match num[..2].as_ref() {
        "0x" =>  u32::from_str_radix(&num[2..], 16).map_err(|_| NumParseError::InvalidHex),
        "0o" =>  u32::from_str_radix(&num[2..], 8).map_err(|_| NumParseError::InvalidOctal),
        "0b" =>  u32::from_str_radix(&num[2..], 2).map_err(|_| NumParseError::InvalidBinary),
        _ => *&num[..].parse::<u32>().map_err(|_| NumParseError::InvalidDecimal),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumParseError {
    InvalidHex,
    InvalidOctal,
    InvalidBinary,
    InvalidDecimal,
}

impl std::error::Error for NumParseError {}

impl std::fmt::Display for NumParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NumParseError::InvalidHex=> write!(f, "invalid hex number"),
            NumParseError::InvalidOctal=> write!(f, "invalid octal number"),
            NumParseError::InvalidBinary=> write!(f, "invalid binary number"),
            NumParseError::InvalidDecimal=> write!(f, "invalid decimal number"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_hex() {
        let res = parse_num("0x4b4b4b4b");
        assert_eq!(res, Ok(1263225675));
    }
    
    #[test]
    fn err_hex() {
        let res = parse_num("0xb0rp");
        assert_eq!(res, Err(NumParseError::InvalidHex));
    }
    
    #[test]
    fn parses_octal() {
        let res = parse_num("0o666");
        assert_eq!(res, Ok(438));
    }
    
    #[test]
    fn err_octal() {
        let res = parse_num("0o888");
        assert_eq!(res, Err(NumParseError::InvalidOctal));
    }
    
    #[test]
    fn parses_binary() {
        let res = parse_num("0b01001011");
        assert_eq!(res, Ok(75));
    }
    
    #[test]
    fn err_binary() {
        let res = parse_num("0b23");
        assert_eq!(res, Err(NumParseError::InvalidBinary));
    }
    
    #[test]
    fn parses_decimal() {
        let res = parse_num("8675309");
        assert_eq!(res, Ok(8675309));
    }
    
    #[test]
    fn err_decimal() {
        let res = parse_num("pretend_to_be_a_number");
        assert_eq!(res, Err(NumParseError::InvalidDecimal));
    }
}
