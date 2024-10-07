use thiserror::Error;

pub type Atom = (String, usize);
pub type Molecule = Vec<Atom>;

#[derive(Error, Debug)]
pub enum ParseError {
    // variants
}

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    todo!();
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::{Molecule, parse_molecule};
    
    macro_rules! assert_parse {
        ($formula:expr, $expected:expr, $name:ident) => {
            #[test]
            fn $name() {
                super::assert_parse($formula, &$expected, "");
            }
        }
    }

    mod molecules {
        assert_parse!("H", [("H",1)], hydrogen);
        assert_parse!("O2", [("O",2)], oxygen);
        assert_parse!("H2O", [("H",2),("O",1)], water);
        assert_parse!("Mg(OH)2", [("Mg",1),("O",2),("H",2)], magnesium_hydroxide);
        assert_parse!("K4[ON(SO3)2]2", [("K",4),("O",14),("N",2),("S",4)], fremys_salt);
    }

    #[test]
    fn errors() {
        assert_fail("pie", "Not a valid molecule");
        assert_fail("Mg(OH", "Mismatched parenthesis");
        assert_fail("Mg(OH}2", "Mismatched parenthesis");
    }

    fn assert_fail(formula: &str, msg: &str) {
        let result = parse_molecule(formula);
        assert!(result.is_err(), "expected {} {:?} to fail, got {:?}", msg, formula, result.unwrap());
    }

    fn assert_parse(formula: &str, expected: &[(&str, usize)], _mst: &str) {
        let mut expected = expected.into_iter()
        .map(|&(name, usize)| (name.to_owned(), usize))
        .collect::<Molecule>();
        let result = parse_molecule(formula);
        assert!(result.is_ok(), "expected {:?} to pass, got {:?}", formula, result);
        let mut actual = result.unwrap();
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }
}
