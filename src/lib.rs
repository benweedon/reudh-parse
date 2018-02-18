mod node;
mod result;

pub use node::Node;
pub use result::{Error, Result};

#[derive(Debug)]
pub struct Etym<'a> {
    word: &'a str,
    definition: &'a str,
}

impl<'a> Etym<'a> {
    pub fn new(word: &'a str, definition: &'a str) -> Etym<'a> {
        Etym {
            word: word,
            definition: definition,
        }
    }
}

pub fn parse<'a>(etym: &Etym<'a>) -> Result<Node<'a>> {
    Ok(Node::new(etym.word))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_works() {
        let etym = Etym::new("word", "definition");
        let result = parse(&etym);
        assert!(result.is_ok());
        let node = result.unwrap();
        assert_eq!(node.name, etym.word);
        assert_eq!(node.parents.borrow().len(), 0);
        assert_eq!(node.children.borrow().len(), 0);
    }
}
