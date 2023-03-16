extern crate core;

use crate::datatype::{Context, Problem, Solution};
use crate::r#match::match_;
use crate::simpl::simpl;
use crate::substs::problem_substitution;

mod datatype;
mod substs;
mod simpl;
mod r#match;


fn main_huet(context: &mut Context, problem: Problem) {
    let mut p_simpl = simpl(context.clone(), problem);

    if p_simpl.is_none() {
        return
    }

    let mut p_simpl = p_simpl.unwrap();

    println!("p_simpl: {:#?}", p_simpl);

    if p_simpl.is_empty() {
        context.solutions.borrow_mut().push(context.substitutions.clone());
        return;
    }

    let constraint = p_simpl.pop();
    if constraint.is_none() {
        return
    }
    let constraint = constraint.unwrap();
    let substitution_set = match_(context.clone(), constraint.clone());

    p_simpl.push(constraint);
    for substitution in substitution_set {
        let new_problem = problem_substitution(p_simpl.clone(), substitution.clone());
        let mut substs_for_context = context.substitutions.clone();
        substs_for_context.push(substitution.clone());

        let mut new_context = Context {
            typing_context: context.typing_context.clone(),
            substitutions: substs_for_context,
            solutions: context.solutions.clone(),
        };
        println!("new_context substitutions: {:#?}", new_context.substitutions);
        println!("new_problem: {:#?}", new_problem);
        main_huet(&mut new_context, new_problem);
    }
}


mod tests {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;
    use crate::datatype::{Constraint, Context, Term, Type};
    use crate::main_huet;

    #[test]
    fn example_2() {
        // Arrange
        let constraint = Constraint {
            left: Term::App(
                Box::new(Term::Meta("I".to_string())),
                Box::new(Term::Var("u32".to_string()))
            ),
            right: Term::App(
                Box::new(Term::Var("option".to_string())),
                Box::new(Term::App(
                    Box::new(Term::Var("option".to_string())),
                    Box::new(Term::Var("u32".to_string()))
                ))
            )
        };

        let mut context = Context {
            typing_context: HashMap::from_iter([
                ("u32".to_string(), Type::Star),
                ("option".to_string(), Type::Arrow(
                    Box::new(Type::Star),
                    Box::new(Type::Star)
                ))
            ]),
            substitutions: vec![],
            solutions: Rc::new(RefCell::new(vec![])),
        };
        // Act
        main_huet(&mut context, vec![constraint]);

        // Assert
        println!("Context: {:#?}", context);
    }
}