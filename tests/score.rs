extern crate reudh_parse;

use reudh_parse::{parse, Etym};

#[test]
fn validate_score() {
    let etym = Etym::new("test word", "test definition");
    let result = parse(&etym);
    assert!(result.is_ok());
}
