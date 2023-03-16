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
    /// Testing I u32 =? option u32
    fn example_1() {
        // Arrange
        let constraint = Constraint {
            left: Term::App(
                Box::new(Term::Meta("I".to_string())),
                Box::new(Term::Var("u32".to_string()))
            ),
            right: Term::App(
                Box::new(Term::Var("option".to_string())),
                Box::new(Term::Var("u32".to_string()))
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

    #[test]
    /// Testing I u32 =? option (option u32)
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

    #[test]
    /// Testing I (I u32) =? option (option u32)
    fn example_3() {
        // Arrange
        let constraint = Constraint {
            left: Term::App(
                Box::new(Term::Meta("I".to_string())),
                Box::new(
                    Term::App(
                        Box::new(Term::Meta("I".to_string())),
                        Box::new(Term::Var("u32".to_string()))
                    )
                )
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

    #[test]
    /// Testing I (L u32) =? option (option u32)
    fn example_4() {
        // Arrange
        let constraint = Constraint {
            left: Term::App(
                Box::new(Term::Meta("I".to_string())),
                Box::new(
                    Term::App(
                        Box::new(Term::Meta("L".to_string())),
                        Box::new(Term::Var("u32".to_string()))
                    )
                )
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
        println!("Number of solutions: {:#?}", context.solutions.borrow_mut().len());
        println!("Context: {:#?}", context);
    }

    #[test]
    /// Testing I u32 =? option u32 and
    /// I String =? option String
    fn example_5() {
        // Arrange
        let constraint_1 = Constraint {
            left: Term::App(
                Box::new(Term::Meta("I".to_string())),
                Box::new(Term::Var("u32".to_string()))
            ),
            right: Term::App(
                Box::new(Term::Var("option".to_string())),
                Box::new(Term::Var("u32".to_string()))
            )
        };

        let constraint_2 = Constraint {
            left: Term::App(
                Box::new(Term::Meta("I".to_string())),
                Box::new(Term::Var("String".to_string()))
            ),
            right: Term::App(
                Box::new(Term::Var("option".to_string())),
                Box::new(Term::Var("String".to_string()))
            )
        };

        let mut context = Context {
            typing_context: HashMap::from_iter([
                ("u32".to_string(), Type::Star),
                ("String".to_string(), Type::Star),
                ("option".to_string(), Type::Arrow(
                    Box::new(Type::Star),
                    Box::new(Type::Star)
                ))
            ]),
            substitutions: vec![],
            solutions: Rc::new(RefCell::new(vec![])),
        };
        // Act
        main_huet(&mut context, vec![constraint_1, constraint_2]);

        // Assert
        println!("Context: {:#?}", context);
    }

    #[test]
    /// Testing I u32 =? option u32 and
    /// I String =? option u32
    fn example_6() {
        // Arrange
        let constraint_1 = Constraint {
            left: Term::App(
                Box::new(Term::Meta("I".to_string())),
                Box::new(Term::Var("u32".to_string()))
            ),
            right: Term::App(
                Box::new(Term::Var("option".to_string())),
                Box::new(Term::Var("u32".to_string()))
            )
        };

        let constraint_2 = Constraint {
            left: Term::App(
                Box::new(Term::Meta("I".to_string())),
                Box::new(Term::Var("String".to_string()))
            ),
            right: Term::App(
                Box::new(Term::Var("option".to_string())),
                Box::new(Term::Var("u32".to_string()))
            )
        };

        let mut context = Context {
            typing_context: HashMap::from_iter([
                ("u32".to_string(), Type::Star),
                ("String".to_string(), Type::Star),
                ("option".to_string(), Type::Arrow(
                    Box::new(Type::Star),
                    Box::new(Type::Star)
                ))
            ]),
            substitutions: vec![],
            solutions: Rc::new(RefCell::new(vec![])),
        };
        // Act
        main_huet(&mut context, vec![constraint_1, constraint_2]);

        // Assert
        println!("Context: {:#?}", context);
    }

    #[test]
    /// Testing I u32 =? option u32 and
    /// I String =? option bool
    /// This case should fail as there should not be any solutions.
    fn example_7() {
        // Arrange
        let constraint_1 = Constraint {
            left: Term::App(
                Box::new(Term::Meta("I".to_string())),
                Box::new(Term::Var("u32".to_string()))
            ),
            right: Term::App(
                Box::new(Term::Var("option".to_string())),
                Box::new(Term::Var("u32".to_string()))
            )
        };

        let constraint_2 = Constraint {
            left: Term::App(
                Box::new(Term::Meta("I".to_string())),
                Box::new(Term::Var("String".to_string()))
            ),
            right: Term::App(
                Box::new(Term::Var("option".to_string())),
                Box::new(Term::Var("bool".to_string()))
            )
        };

        let mut context = Context {
            typing_context: HashMap::from_iter([
                ("u32".to_string(), Type::Star),
                ("String".to_string(), Type::Star),
                ("bool".to_string(), Type::Star),
                ("option".to_string(), Type::Arrow(
                    Box::new(Type::Star),
                    Box::new(Type::Star)
                ))
            ]),
            substitutions: vec![],
            solutions: Rc::new(RefCell::new(vec![])),
        };
        // Act
        main_huet(&mut context, vec![constraint_1, constraint_2]);

        // Assert
        println!("Context: {:#?}", context);
        assert_eq!(true, context.solutions.borrow_mut().is_empty());
    }

    #[test]
    /// Testing I u32 =? result u32 String
    fn example_8() {
        // Arrange
        let constraint = Constraint {
            left: Term::App(
                Box::new(Term::Meta("I".to_string())),
                Box::new(Term::Var("u32".to_string()))
            ),
            right: Term::App(
                Box::new(Term::App(
                    Box::new(Term::Var("result".to_string())),
                    Box::new(Term::Var("u32".to_string()))
                )),
                Box::new(Term::Var("String".to_string()))
            )
        };

        let mut context = Context {
            typing_context: HashMap::from_iter([
                ("u32".to_string(), Type::Star),
                ("String".to_string(), Type::Star),
                ("result".to_string(), Type::Arrow(
                    Box::new(Type::Star),
                    Box::new(Type::Arrow(
                        Box::new(Type::Star),
                        Box::new(Type::Star)
                    ))
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