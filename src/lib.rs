extern crate core;


use crate::datatype::{Context, Problem, Solution};
use crate::parse::{parse_constraint, parse_problem, parse_term};
use crate::r#match::match_;
use crate::simpl::simpl;
use crate::substs::problem_substitution;

mod datatype;
mod substs;
mod simpl;
mod r#match;
mod parse;
mod print;


#[test]
fn calculator1() {
    println!("{:#?}", parse_term("λx:*. λy:*. N").print());
    println!("{:#?}", parse_term("λx:*. λy:*. N"));
    println!("{:#?}", parse_term("N O P E").print());
    println!("{:#?}", parse_term("N O P E"));
    println!("{:#?}", parse_term("N O (P E)").print());
    println!("{:#?}", parse_term("N O (P E)"));
    println!("{:#?}", parse_term("N (O P E)").print());
    println!("{:#?}", parse_term("N (O P E)"));
    println!("{:#?}", parse_term("N (λx:*. n)").print());
    println!("{:#?}", parse_term("N (λx:*. n)"));
    println!("{:#?}", parse_term("(λx:*. λy:*. n h) N").print());
    println!("{:#?}", parse_term("(λx:*. λy:*. n h) N"));
    println!("{:#?}", parse_term("(λx:*. (λy:*. n) h) N").print());
    println!("{:#?}", parse_term("(λx:*. (λy:*. n) h) N"));
    println!("{:#?}", parse_constraint("I u32 =? option (option u32)").print());
    println!("{:#?}", parse_constraint("I u32 =? option (option u32)"));
    println!("{:#?}", parse_problem("I u32 =? option u32 ∧ I string =? option bool").print());
    println!("{:#?}", parse_problem("I u32 =? option u32 ∧ I string =? option bool"));
}


fn main_huet(context: &mut Context, problem: Problem) {
    let p_simpl = simpl(context.clone(), problem);

    if p_simpl.is_none() {
        return
    }

    let mut p_simpl = p_simpl.unwrap();

    if p_simpl.0.is_empty() {
        context.solutions.borrow_mut().push(Solution(context.substitutions.clone()));
        return;
    }

    let constraint = p_simpl.0[0].clone();

    let substitution_set = match_(context.clone(), constraint);

    for substitution in substitution_set {
        let new_problem = problem_substitution(p_simpl.clone(), substitution.clone());
        let mut substs_for_context = context.substitutions.clone();
        substs_for_context.push(substitution.clone());

        let mut new_context = Context {
            typing_context: context.typing_context.clone(),
            substitutions: substs_for_context,
            solutions: context.solutions.clone(),
            name_map: context.name_map.clone(),
        };

        main_huet(&mut new_context, new_problem);
    }
}


mod tests {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;
    use crate::datatype::{Context};
    use crate::main_huet;
    use crate::parse::{parse_problem, parse_type};

    macro_rules! test_harness {
        ($l:literal) => {
            {
                // Arrange
                let problem = parse_problem($l);

                let mut context = generate_context();

                // Act
                main_huet(&mut context, problem.clone());

                // Assert
                println!("Problem: {}", problem);
                println!();
                println!("Number of solutions: {:#?}", context.solutions.borrow().len());
                let minimal = context.minimal_solutions();
                println!("Context: {}", minimal);
                minimal
            }
        };
    }

    fn generate_context() -> Context {
        Context {
            typing_context: HashMap::from_iter([
                ("u32".to_string(), parse_type("*")),
                ("bool".to_string(), parse_type("*")),
                ("string".to_string(), parse_type("*")),
                ("result".to_string(), parse_type("* -> * -> *")),
                ("option".to_string(), parse_type("* -> *")),
            ]),
            substitutions: vec![],
            solutions: Rc::new(RefCell::new(vec![])),
            name_map: HashMap::from_iter([
                ("I".to_string(), vec!["j".to_string()]),
                ("L".to_string(), vec!["m".to_string()]),
                ("P".to_string(), vec!["q".to_string(), "r".to_string()]),
            ]),
        }
    }

    #[test]
    fn example_1() {
        test_harness!("I u32 =? option u32");
    }

    #[test]
    fn example_2() {
        test_harness!("I u32 =? option (option u32)");
    }

    #[test]
    fn example_3() {
        test_harness!("I (I u32) =? option (option u32)");
    }

    #[test]
    fn example_4() {
        test_harness!("I (L u32) =? option (option u32)");
    }

    #[test]
    fn example_5() {
        test_harness!("I u32 =? option u32 ∧ I string =? option string");
    }

    #[test]
    fn example_6() {
        test_harness!("I u32 =? option u32 ∧ I string =? option u32");
    }

    #[test]
    /// This case should fail as there should not be any solutions.
    fn example_7() {
        let solutions = test_harness!("I u32 =? option u32 ∧ I string =? option bool");
        assert!(solutions.0.is_empty());
    }

    #[test]
    fn example_8() {
        test_harness!("I u32 =? result u32 string");
    }

    #[test]
    fn example_9() {
        test_harness!("I (I u32) =? option u32");
    }

    #[test]
    fn example_10() {
        test_harness!("P u32 u32 =? result u32 u32");
    }

    #[test]
    fn example_11() {
        test_harness!("I u32 =? result u32 u32");
    }

    #[test]
    fn example_12() {
        test_harness!("I u32 =? result string string");
    }

    #[test]
    fn example_13() {
        test_harness!("I u32 =? result u32 string");
    }

    #[test]
    fn example_14() {
        test_harness!("I (L u32) =? result u32 string");
    }
}