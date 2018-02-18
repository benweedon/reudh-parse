mod node;
mod result;

pub use node::Node;
pub use result::{Error, Result};

#[derive(Debug)]
pub struct Etym {
    word: String,
    definition: String,
}

impl Etym {
    pub fn new(word: String, definition: String) -> Etym {
        Etym {
            word: word,
            definition: definition,
        }
    }

    pub fn from_slices(word: &str, definition: &str) -> Etym {
        Etym {
            word: word.to_owned(),
            definition: definition.to_owned(),
        }
    }
}

pub fn parse(etym: Etym) -> Result<Node> {
    Ok(Node::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_works() {
        let etym = Etym::from_slices("word", "definition");
        let result = parse(etym);
        assert!(result.is_ok());
    }
}
