use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug};
use std::rc::Rc;
use std::str::FromStr;
use std::string::ToString;
use std::sync::atomic::{AtomicU32, Ordering};
use crate::datatype::Term::{Abs, App, Var};
use crate::datatype::Type::Star;
use crate::substs::{beta_reduce, term_substitution};
use crate::util::amount_of_swaps_to_sort;

const PLACEHOLDER: &'static str = "placeholder";

#[derive(Clone, PartialEq, Debug)]
pub enum Term {
    Meta(String),
    Var(String),
    Abs(String, Type, Box<Term>),
    App(Box<Term>, Box<Term>)
}

#[derive(Clone, PartialEq, Debug)]
pub enum Type {
    Star,
    Arrow(Box<Type>, Box<Type>)
}

#[derive(Clone, PartialEq, Debug)]
pub struct Context {
    pub typing_context: HashMap<String, Type>,
    pub substitutions: Vec<Substitution>,
    pub solutions: Rc<RefCell<Vec<Solution>>>,
    pub name_map: HashMap<String, Vec<String>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Constraint {
    pub left: Term,
    pub right: Term
}

#[derive(Clone, PartialEq, Debug)]
pub struct Substitution {
    pub name: String,
    pub with: Term
}

#[derive(Clone, PartialEq, Debug)]
pub struct Problem(pub Vec<Constraint>);

#[derive(Clone, PartialEq, Debug)]
pub struct Solution(pub Vec<Substitution>);

#[derive(Clone, PartialEq, Debug)]
pub struct SolutionSet(pub Vec<Solution>);


impl Term {
    pub fn is_rigid(&self) -> bool {
        !matches!(self, Term::Meta(_))
    }

    pub fn split(&self) -> (Term, Term, Vec<Term>) {
        let mut current = self.clone();
        let mut external_abstractors_builder = Term::Var(PLACEHOLDER.to_string());
        let mut reverse_abstractors_builder = Term::Var(PLACEHOLDER.to_string());
        let mut arguments_builder = vec![];

        // Extract external abstractors
        while let Term::Abs(s, typ, term) = current {
            current = *term;
            external_abstractors_builder = Term::Abs(s, typ, Box::new(external_abstractors_builder));
        }

        while let Term::Abs(s, typ, term) = external_abstractors_builder {
            external_abstractors_builder = *term;
            reverse_abstractors_builder = Term::Abs(s, typ, Box::new(reverse_abstractors_builder));
        }


        // Extract arguments
        while let Term::App(t1, t2) = current {
            arguments_builder.push(*t2);
            current = *t1;
        }
        arguments_builder.reverse();

        // Extract head
        match current {
            Term::Meta(_) | Term::Var(_) => (reverse_abstractors_builder, current, arguments_builder),
            Term::Abs(_, _, _) => panic!("The term is not in eta-normal form"),
            Term::App(_, _) => unreachable!()
        }
    }

    pub fn combine(&self, bindings: Term) -> Term {
        let substs = Substitution { name: PLACEHOLDER.to_string(), with: self.clone() };
        term_substitution(bindings, substs)
    }

    pub fn get_name(&self) -> String {
        match self {
            Term::Meta(s) | Term::Var(s) => s.clone(),
            _ => panic!("Can not get name for Term::Abs or Term::App")
        }
    }

    pub fn equal_in_context(&self, other: &Term, context: &HashMap<String, Type>) -> bool {
        match (self, other) {
            (Term::Var(s1), Term::Var(s2)) if s1 == s2 => {
                context.get(s1).is_some()
            }
            (_, _) => false
        }
    }

    pub fn binding_index(&self, bindings: &Term) -> Option<usize> {
        let mut current = bindings;
        let mut depth = 0;
        let mut last_seen_index = None;
        while let Term::Abs(s, _, term) = current {
            current = term;
            if s == &self.get_name() {
                last_seen_index = Some(depth);
            }
            depth += 1;
        }
        last_seen_index.map(|index| depth - index)
    }

    pub fn number_of_constants(&self, bounded: HashSet<String>) -> usize {

        match self {
            Var(s) => {
                if bounded.contains(s) {
                    0
                } else {
                    1
                }
            }
            Abs(s, _, inner) => {
                let mut new_bounded = bounded.clone();
                new_bounded.insert(s.clone());
                inner.number_of_constants(new_bounded)
            }
            App(a, call_arg) if matches!(**a, Var(_)) => {
                call_arg.number_of_constants(bounded)
            }
            App(callee, call_arg) => {
                callee.number_of_constants(bounded.clone()) + call_arg.number_of_constants(bounded)
            }
            Term::Meta(..) => 0
        }
    }

    pub fn number_of_unique_params(&self, bounded: HashSet<String>) -> HashSet<String> {

        match self {
            Var(s) => {
                if bounded.contains(s) {
                    let mut set = HashSet::new();
                    set.insert(s.clone());
                    set
                } else {
                    HashSet::new()
                }
            }
            Abs(s, _, inner) => {
                let mut new_bounded = bounded.clone();
                new_bounded.insert(s.clone());
                inner.number_of_unique_params(new_bounded)
            }
            App(a, call_arg) if matches!(**a, Var(_)) => {
                call_arg.number_of_unique_params(bounded)
            }
            App(callee, call_arg) => {
                &callee.number_of_unique_params(bounded.clone()) | &call_arg.number_of_unique_params(bounded)
            }
            Term::Meta(..) => HashSet::new()
        }
    }

    pub fn number_of_params(&self, bounded: HashSet<String>) -> usize {

        match self {
            Var(_) => {
                1
            }
            Abs(s, _, inner) => {
                let mut new_bounded = bounded.clone();
                new_bounded.insert(s.clone());
                inner.number_of_params(new_bounded)
            }

            App(callee, call_arg) => {
                &callee.number_of_params(bounded.clone()) + &call_arg.number_of_params(bounded)
            }
            Term::Meta(..) => 0
        }
    }

    pub fn number_of_swaps(&self, bounded: HashSet<String>) -> Vec<usize> {

        match self {
            Var(s) => {
                if bounded.contains(s) {
                    Vec::from([s.parse::<usize>().unwrap()])
                } else {
                    Vec::new()
                }
            }
            Abs(s, _, inner) => {
                let mut new_bounded = bounded.clone();
                new_bounded.insert(s.clone());
                inner.number_of_swaps(new_bounded)
            }
            App(a, call_arg) if matches!(**a, Var(_)) => {
                call_arg.number_of_swaps(bounded)
            }
            App(callee, call_arg) => {
                let mut first = callee.number_of_swaps(bounded.clone());
                first.append(&mut call_arg.number_of_swaps(bounded));
                first
            }
            Term::Meta(..) => Vec::new()
        }
    }
}

impl Constraint {
    pub fn is_rigid_rigid(&self) -> bool {
        let (_, l_head, _) = self.left.split();
        let (_, r_head, _) = self.right.split();

        l_head.is_rigid() && r_head.is_rigid()
    }
}

pub fn generate_fresh_var() -> String {
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    format!("{:?}", COUNTER.fetch_add(1, Ordering::Relaxed))
}

impl Solution {
    pub fn minimize(self, name_map: &HashMap<String, Vec<String>>) -> Solution {
        let mut originals =
            self.0.iter()
                .filter(|substitution| u32::from_str(&substitution.name).is_err())
                .cloned()
                .collect::<Vec<_>>();

        for sub in self.0 {
            for original in &mut originals {
                original.with = term_substitution(original.with.clone(), sub.clone())
            }
        }

        for original in &mut originals {
            if let Some(list) = name_map.get(&original.name) {
                let mut builder = original.with.clone();
                for element in list {
                    builder = beta_reduce(App(Box::new(builder), Box::new(Var(element.to_string()))))
                }

                for element in list.iter().rev() {
                    builder = Abs(element.to_string(), Star, Box::new(builder))
                }

                original.with = builder;
            }
        }

        Solution(originals)
    }

    pub fn name_map(self, name_map: &HashMap<String, Vec<String>>) -> Solution {
        let mut originals =
            self.0.iter()
                .filter(|substitution| u32::from_str(&substitution.name).is_err())
                .cloned()
                .collect::<Vec<_>>();

        for original in &mut originals {
            if let Some(list) = name_map.get(&original.name) {
                let mut builder = original.with.clone();
                for element in list {
                    builder = beta_reduce(App(Box::new(builder), Box::new(Var(element.to_string()))))
                }

                for element in list.iter().rev() {
                    builder = Abs(element.to_string(), Star, Box::new(builder))
                }

                original.with = builder;
            }
        }

        Solution(originals)
    }

    pub fn number_of_constants(&self) -> usize {
        self.0.iter().map(|a| a.number_of_constants()).sum()
    }

    pub fn number_of_unique_params(&self) -> usize {
        self.0.iter().map(|a| a.number_of_unique_params()).sum()
    }

    pub fn number_of_params(&self) -> usize {
        self.0.iter().map(|a| a.number_of_params()).sum()
    }

    pub fn number_of_swaps(&self) -> usize {
        self.0.iter().map(|a| a.number_of_swaps()).sum()
    }
}

impl Substitution {
    pub fn number_of_unique_params(&self) -> usize {
        self.with.number_of_unique_params(HashSet::new()).len()
    }

    pub fn number_of_params(&self) -> usize {
        self.with.number_of_params(HashSet::new())
    }

    pub fn number_of_constants(&self) -> usize {
        self.with.number_of_constants(HashSet::new())
    }

    pub fn number_of_swaps(&self) -> usize {
        let list = self.with.number_of_swaps(HashSet::new());
        amount_of_swaps_to_sort(list)
    }
}

impl Context {
    pub fn minimal_solutions(&self) -> SolutionSet {
        SolutionSet(self.solutions.borrow().iter().cloned().map(|solution| solution.minimize(&self.name_map)).collect())
    }

    pub fn minimal_solutions_without_name_map(&self) -> SolutionSet {
        SolutionSet(self.solutions.borrow().iter().cloned().map(|solution| solution.minimize(&HashMap::new())).collect())
    }
}



#[test]
fn test_split() {
    // Arrange
    let input = Term::Abs("x1".to_string(), Type::Star,
                          Box::new(Term::Abs("x2".to_string(), Type::Star,
                                       Box::new(Term::App(
                                           Box::new(Term::App(
                                               Box::new(Term::Var("h".to_string())),
                                               Box::new(Term::Var("e1".to_string()))
                                           )),
                                           Box::new(Term::Var("e2".to_string()))
                                       )))));

    let head = Term::Var("h".to_string());
    let external_abstractors = Term::Abs("x1".to_string(), Type::Star,
                                         Box::new(
                                             Term::Abs("x2".to_string(),
                                                            Type::Star,
                                                            Box::new(Term::Var(PLACEHOLDER.to_string())))));
    let arguments = vec![Term::Var("e1".to_string()), Term::Var("e2".to_string())];
    let expected = (external_abstractors, head,  arguments);

    // Act
    let actual = input.split();

    // Assert
    assert_eq!(actual, expected);
}
