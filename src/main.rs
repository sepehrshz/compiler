use semantic::Sem;

pub(crate) mod lexial;
pub(crate) mod semantic;
pub(crate) mod syntax;
pub(crate) mod token;

const TEST_IN: &str = include_str!("./../tests/test.c");

fn main() {
    let mut sem = Sem::new(TEST_IN).unwrap();
    sem.parser();
}
