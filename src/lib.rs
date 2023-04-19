extern crate core;


use crate::datatype::{Context, Problem, Solution};
use crate::r#match::match_;
use crate::simpl::simpl;
use crate::substs::problem_substitution;

mod datatype;
mod substs;
mod simpl;
mod r#match;
mod parse;
mod print;
mod util;
mod prioritization;

fn main_huet(context: &mut Context, problem: Problem) {
    let p_simpl = simpl(context.clone(), problem);

    if p_simpl.is_none() {
        return
    }

    let p_simpl = p_simpl.unwrap();

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
    use crate::datatype::{Context, Solution, SolutionSet};
    use crate::main_huet;
    use crate::util;
    use crate::parse::{parse_constraint, parse_problem, parse_term, parse_type};
    use crate::prioritization::{exhaustiveness, existence, generality, get_solution_from_solution_set, get_solution_from_solution_set_by_priority, ordering, simplicity};

    fn run(input: &str) -> SolutionSet {
        // Arrange
        let problem = parse_problem(input);

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

    fn run_with_priorities(input: &str) {
        // Arrange
        let problem = parse_problem(input);

        let mut context = generate_context_without_name_map();

        // Act
        main_huet(&mut context, problem.clone());

        // Assert
        println!("Problem: {}", problem);
        println!();
        println!("Number of solutions: {:#?}", context.solutions.borrow().len());
        let minimal = context.minimal_solutions();
        println!("Non-filtered solutions: {}", minimal);
        let filtered = get_solution_from_solution_set(minimal.clone());
        match filtered.clone() {
            Ok(solution) => println!("Filtered solutions: {}", solution),
            Err(solutions) => println!("Filtered solutions {}", solutions)
        }
    }

    fn run_with_priority_existence(input: &str) -> SolutionSet {
        // Arrange
        let problem = parse_problem(input);

        let mut context = generate_context_without_name_map();

        // Act
        main_huet(&mut context, problem.clone());

        // Assert
        println!("Problem: {}", problem);
        println!();
        println!("Number of solutions: {:#?}", context.solutions.borrow().len());
        let minimal = context.minimal_solutions();
        println!("Non-filtered solutions: {}", minimal);
        let filtered = get_solution_from_solution_set_by_priority(minimal.clone(), existence);
        println!("Number of filtered solutions: {:#?}", filtered.0.len());
        println!("Filtered solutions: {}", filtered);

        filtered
    }

    fn run_with_priority_generality(input: &str) -> SolutionSet {
        // Arrange
        let problem = parse_problem(input);

        let mut context = generate_context_without_name_map();

        // Act
        main_huet(&mut context, problem.clone());

        // Assert
        println!("Problem: {}", problem);
        println!();
        println!("Number of solutions: {:#?}", context.solutions.borrow().len());
        let minimal = context.minimal_solutions();
        println!("Non-filtered solutions: {}", minimal);
        let filtered = get_solution_from_solution_set_by_priority(minimal.clone(), generality);
        println!("Number of filtered solutions: {:#?}", filtered.0.len());
        println!("Filtered solutions: {}", filtered);

        filtered
    }

    fn run_with_priority_exhaustiveness(input: &str) -> SolutionSet {
        // Arrange
        let problem = parse_problem(input);

        let mut context = generate_context_without_name_map();

        // Act
        main_huet(&mut context, problem.clone());

        // Assert
        println!("Problem: {}", problem);
        println!();
        println!("Number of solutions: {:#?}", context.solutions.borrow().len());
        let minimal = context.minimal_solutions();
        println!("Non-filtered solutions: {}", minimal);
        let filtered = get_solution_from_solution_set_by_priority(minimal.clone(), exhaustiveness);
        println!("Number of filtered solutions: {:#?}", filtered.0.len());
        println!("Filtered solutions: {}", filtered);

        filtered
    }

    fn run_with_priority_ordering(input: &str) -> SolutionSet {
        // Arrange
        let problem = parse_problem(input);

        let mut context = generate_context_without_name_map();

        // Act
        main_huet(&mut context, problem.clone());

        // Assert
        println!("Problem: {}", problem);
        println!();
        println!("Number of solutions: {:#?}", context.solutions.borrow().len());
        let minimal = context.minimal_solutions();
        println!("Non-filtered solutions: {}", minimal);
        let filtered = get_solution_from_solution_set_by_priority(minimal.clone(), ordering);
        println!("Number of filtered solutions: {:#?}", filtered.0.len());
        println!("Filtered solutions: {}", filtered);

        filtered
    }

    fn run_with_priority_simplicity(input: &str) -> SolutionSet {
        // Arrange
        let problem = parse_problem(input);

        let mut context = generate_context_without_name_map();

        // Act
        main_huet(&mut context, problem.clone());

        // Assert
        println!("Problem: {}", problem);
        println!();
        println!("Number of solutions: {:#?}", context.solutions.borrow().len());
        let minimal = context.minimal_solutions();
        println!("Non-filtered solutions: {}", minimal);
        let filtered = get_solution_from_solution_set_by_priority(minimal.clone(), simplicity);
        println!("Number of filtered solutions: {:#?}", filtered.0.len());
        println!("Filtered solutions: {}", filtered);

        filtered
    }

    fn generate_context() -> Context {
        Context {
            typing_context: HashMap::from_iter([
                ("u32".to_string(), parse_type("*")),
                ("bool".to_string(), parse_type("*")),
                ("string".to_string(), parse_type("*")),
                ("unit".to_string(), parse_type("*")),
                ("result".to_string(), parse_type("* -> * -> *")),
                ("option".to_string(), parse_type("* -> *")),
                ("fn2".to_string(), parse_type("* -> * -> * -> *")),
                ("fn3".to_string(), parse_type("* -> * -> * -> * -> *")),
            ]),
            substitutions: vec![],
            solutions: Rc::new(RefCell::new(vec![])),
            name_map: HashMap::from_iter([
                ("I".to_string(), vec!["j".to_string()]),
                ("L".to_string(), vec!["m".to_string()]),
                ("P".to_string(), vec!["q".to_string(), "r".to_string()]),
                ("T".to_string(), vec!["u".to_string(), "v".to_string()]),
            ]),
        }
    }

    fn generate_context_without_name_map() -> Context {
        Context {
            typing_context: HashMap::from_iter([
                ("u32".to_string(), parse_type("*")),
                ("bool".to_string(), parse_type("*")),
                ("string".to_string(), parse_type("*")),
                ("unit".to_string(), parse_type("*")),
                ("result".to_string(), parse_type("* -> * -> *")),
                ("option".to_string(), parse_type("* -> *")),
                ("fn2".to_string(), parse_type("* -> * -> * -> *")),
                ("fn3".to_string(), parse_type("* -> * -> * -> * -> *")),
            ]),
            substitutions: vec![],
            solutions: Rc::new(RefCell::new(vec![])),
            name_map: HashMap::new(),
        }
    }

    #[test]
    fn example_1() {
        run("I u32 =? option u32");
    }

    #[test]
    fn example_2() {
        run("I u32 =? option (option u32)");
    }

    #[test]
    fn example_3() {
        run("I (I u32) =? option (option u32)");
    }

    #[test]
    fn example_4() {
        run("I (L u32) =? option (option u32)");
    }

    #[test]
    fn example_5() {
        run("I u32 =? option u32 ∧ I string =? option string");
    }

    #[test]
    fn example_6() {
        run("I u32 =? option u32 ∧ I string =? option u32");
    }

    #[test]
    /// This case should fail as there should not be any solutions.
    fn example_7() {
        let solutions = run("I u32 =? option u32 ∧ I string =? option bool");
        assert!(solutions.0.is_empty());
    }

    #[test]
    fn example_8() {
        run("I u32 =? result u32 string");
    }

    #[test]
    fn example_9() {
        run("I (I u32) =? option u32");
    }

    #[test]
    fn example_10() {
        run("P u32 u32 =? result u32 u32");
    }

    #[test]
    fn example_11() {
        run("I u32 =? result u32 u32");
    }

    #[test]
    fn example_12() {
        run("I u32 =? result string string");
    }

    #[test]
    fn example_13() {
        run("I u32 =? result u32 string");
    }

    #[test]
    fn example_14() {
        run("I (L u32) =? result u32 string");
    }

    #[test]
    fn example_15() {
        run("I (L u32) =? option (option u32) ∧ L (I u32) =? option (option u32)");
    }

    #[test]
    fn example_16() {
        run("P u32 u32 =? result u32 u32 ∧ T u32 u32 =? result u32 u32 ");
    }


    #[test]
    fn example_17() {
        run("P u32 u32 =? option u32 ∧ P bool bool =? option bool");
    }

    #[test]
    fn example_18() {
        run("P bool string =? result bool string ∧ P string bool =? result string bool");
    }

    #[test]
    fn example_19() {
        run("I u32 =? result u32 string ∧ I string =? result string string");
    }

    #[test]
    fn example_20() {
        run("I u32 u32 =? u32 ∧ L u32 =? u32 ∧ fn2 (I u32 u32) (L u32) unit =? fn2 u32 u32 unit");
    }

    #[test]
    fn example_21() {
        run("I u32 u32 =? u32 ∧ L u32 =? u32 ∧ fn3 (I u32 u32) (L u32) bool unit =? fn3 u32 u32 A unit");
    }

    #[test]
    fn example_22() {
        run("bool =? A");
    }

    #[test]
    fn example_23() {
        run("P u32 u32 =? result u32 u32 ∧ T u32 u32 =? result u32 u32 ∧ P bool bool =? result bool bool ∧ T bool bool =? result bool bool");
    }

    #[test]
    fn example_priority_existence_1() {
        run_with_priority_existence("I (L u32) =? option (option u32)");
    }

    #[test]
    fn example_priority_existence_2() {
        run_with_priority_existence("I (L (P u32)) =? option (option u32)");
    }

    #[test]
    fn example_priority_existence_3() {
        run_with_priority_existence("I (L u32) (P u32) =? result (option u32) (option u32)");
    }

    #[test]
    fn example_priority_generality_1() {
        run_with_priority_generality("I u32 bool =? result u32 bool");
    }

    #[test]
    fn example_priority_generality_2() {
        run_with_priority_generality("I u32 =? result u32 bool ∧ P u32 =? result u32 bool");
    }

    #[test]
    fn example_priority_generality_3() {
        run_with_priority_generality("I u32 bool =? fn3 u32 bool bool");
    }

    #[test]
    fn example_priority_generality_4() {
        run_with_priority_generality("I u32 u32 =? result u32 u32");
    }

    #[test]
    fn example_priority_exhaustiveness_1() {
        run_with_priority_exhaustiveness("I u32 bool =? option (result u32 bool)");
    }

    #[test]
    fn example_priority_exhaustiveness_2() {
        run_with_priority_exhaustiveness("I u32 bool =? option (result u32 bool) ∧ P u32 =? result u32 bool");
    }

    #[test]
    fn example_priority_exhaustiveness_3() {
        run_with_priority_exhaustiveness("I u32 u32 =? result u32 u32 ∧ P u32 bool =? result u32 bool");
    }

    #[test]
    fn example_priority_ordering_1() {
        run_with_priority_ordering("P u32 u32 =? result u32 u32 ∧ T u32 u32 =? result u32 u32 ∧ P bool bool =? result bool bool ∧ T bool bool =? result bool bool");
    }

    #[test]
    fn example_priority_ordering_2() {
        run_with_priority_ordering("P u32 u32 =? result u32 u32 ∧ T u32 u32 =? result u32 u32");
    }

    #[test]
    fn example_priority_ordering_3() {
        run_with_priority_ordering("I u32 u32 u32 =? fn3 u32 u32 u32");
    }

    #[test]
    fn example_priority_ordering_4() {
        run_with_priority_ordering("I u32 bool string =? result (fn2 u32 string) bool");
    }

    #[test]
    fn example_priority_simplicity_1() {
        run_with_priority_simplicity("P u32 u32 =? result u32 u32 ∧ P bool bool =? result bool bool");
    }

    #[test]
    fn example_priority_simplicity_2() {
        run_with_priority_simplicity("P u32 bool =? result u32 bool ∧ P bool bool =? result bool bool");
    }

    #[test]
    fn example_priority_simplicity_3() {
        run_with_priority_simplicity("I u32 bool =? fn3 u32 bool bool");
    }

    #[test]
    fn example_priority_1() {
        run_with_priorities("I (L u32) =? option (option u32)");
    }

    #[test]
    fn example_priority_2() {
        run_with_priorities("I (L (P u32)) =? option (option u32)");
    }

    #[test]
    fn example_priority_3() {
        run_with_priorities("I (L u32) (P u32) =? result (option u32) (option u32)");
    }

    #[test]
    fn example_priority_4() {
        run_with_priorities("I u32 bool =? result u32 bool");
    }

    #[test]
    fn example_priority_5() {
        run_with_priorities("I u32 =? result u32 bool ∧ P u32 =? result u32 bool");
    }

    #[test]
    fn example_priority_6() {
        run_with_priorities("I u32 bool =? fn3 u32 bool bool");
    }

    #[test]
    fn example_priority_7() {
        run_with_priorities("I u32 bool =? option (result u32 bool)");
    }

    #[test]
    fn example_priority_8() {
        run_with_priorities("I u32 bool =? option (result u32 bool) ∧ P u32 =? result u32 bool");
    }

    #[test]
    fn example_priority_9() {
        run_with_priorities("I u32 u32 =? result u32 u32 ∧ P u32 bool =? result u32 bool");
    }

    #[test]
    fn example_priority_10() {
        run_with_priorities("P u32 u32 =? result u32 u32 ∧ T u32 u32 =? result u32 u32 ∧ P bool bool =? result bool bool ∧ T bool bool =? result bool bool");
    }

    #[test]
    fn example_priority_11() {
        run_with_priorities("I u32 bool string =? result (fn2 u32 string) bool");
    }

    #[test]
    fn example_priority_12() {
        run_with_priorities("P u32 u32 =? result u32 u32 ∧ P bool bool =? result bool bool");
    }


    #[test]
    fn parse_and_print() {
        println!("{}   ", parse_term("λx:*. λy:*. N"));
        println!("{:#?}", parse_term("λx:*. λy:*. N"));
        println!("{}   ", parse_term("N O P E"));
        println!("{:#?}", parse_term("N O P E"));
        println!("{}   ", parse_term("N O (P E)"));
        println!("{:#?}", parse_term("N O (P E)"));
        println!("{}   ", parse_term("N (O P E)"));
        println!("{:#?}", parse_term("N (O P E)"));
        println!("{}   ", parse_term("N (λx:*. n)"));
        println!("{:#?}", parse_term("N (λx:*. n)"));
        println!("{}   ", parse_term("(λx:*. λy:*. n h) N"));
        println!("{:#?}", parse_term("(λx:*. λy:*. n h) N"));
        println!("{}   ", parse_term("(λx:*. (λy:*. n) h) N"));
        println!("{:#?}", parse_term("(λx:*. (λy:*. n) h) N"));
        println!("{}   ", parse_constraint("I u32 =? option (option u32)"));
        println!("{:#?}", parse_constraint("I u32 =? option (option u32)"));
        println!("{}   ", parse_problem("I u32 =? option u32 ∧ I string =? option bool"));
        println!("{:#?}", parse_problem("I u32 =? option u32 ∧ I string =? option bool"));
    }

    #[test]
    fn swap_count_for_simple_lists() {
        let list1 = vec![4,3,2,1];
        let list2 = vec![2,4,5,1,3];
        let res1 = util::amount_of_swaps_to_sort(list1);
        let res2 = util::amount_of_swaps_to_sort(list2);
        assert_eq!(res1, 2);
        assert_eq!(res2, 3);

    }
}
