use lalrpop_util::lalrpop_mod;
use crate::datatype::{Constraint, Problem, Term, Type};

lalrpop_mod!(parser);

pub fn parse_problem(s: &str) -> Problem {
    parser::ProblemParser::new().parse(s).unwrap()
}

pub fn parse_constraint(s: &str) -> Constraint {
    parser::ConstraintParser::new().parse(s).unwrap()
}

pub fn parse_term(s: &str) -> Term {
    parser::TermParser::new().parse(s).unwrap()
}

pub fn parse_type(s: &str) -> Type {
    parser::TypeParser::new().parse(s).unwrap()
}