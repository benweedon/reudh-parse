extern crate petgraph;

mod graph;
mod result;

pub use graph::Graph;
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

pub fn parse<'a>(_etym: &Etym<'a>) -> Result<Graph<'a>> {
    Ok(Graph::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_works() {
        let etym = Etym::new("word", "definition");
        let result = parse(&etym);
        assert!(result.is_ok());
    }
}
