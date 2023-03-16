extern crate core;

use crate::datatype::{Context, Problem, Solution};
use crate::r#match::match_;
use crate::simpl::simpl;
use crate::substs::problem_substitution;

mod datatype;
mod substs;
mod simpl;
mod r#match;


fn main_huet(mut context: Context, problem: Problem) {
    println!("Hello world!");
    let mut p_simpl = simpl(context.clone(), problem);

    if p_simpl.is_none() {
        return
    }

    let mut p_simpl = p_simpl.unwrap();

    if p_simpl.is_empty() {
        context.solutions.push(context.substitutions.clone());
        return;
    }

    let constraint = p_simpl.pop();
    if constraint.is_none() {
        return
    }
    let substitution_set = match_(context.clone(), constraint.unwrap());

    for substitution in substitution_set {
        let new_problem = problem_substitution(p_simpl.clone(), substitution.clone());
        let mut substs_for_context = context.substitutions.clone();
        substs_for_context.push(substitution.clone());

        let new_context = Context {
            typing_context: context.typing_context.clone(),
            substitutions: substs_for_context,
            solutions: context.solutions.clone(),
        };
        main_huet(new_context, new_problem);
    }
}