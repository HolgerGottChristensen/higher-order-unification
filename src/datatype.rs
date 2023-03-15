use std::collections::HashMap;

pub enum Term {
    Meta(String),
    Var(String),
    Abs(String, Type, Box<Term>),
    App(Box<Term>, Box<Term>)
}

pub enum Type {
    Star,
    Arrow(Box<Type>, Box<Type>)
}

pub struct Context {
    pub typing_context: HashMap<String, Type>,
    pub substitutions: Vec<Substitution>,
    pub solutions: Vec<Solution>
}

pub struct Constraint {
    pub left: Term,
    pub right: Term
}

pub struct Substitution {
    pub name: String,
    pub with: Term
}

pub type Problem = Vec<Constraint>;
pub type Solution = Vec<Substitution>;

impl Term {
    pub fn is_rigid(&self) -> bool {
        !matches!(self, Term::Meta(_))
    }

    pub fn split(&self) -> (Term, Term, Vec<Term>) {
        todo!()
    }

    pub fn combine(&self, bindings: Term) -> Term {
        todo!()
    }

    pub fn equal_in_context(&self, other: &Term, context: &HashMap<String, Type>) -> bool {
        match (self, other) {
            (Term::Var(s1), Term::Var(s2)) if s1 == s2 => {
                context.get(s1).is_some()
            }
            (_, _) => false
        }
    }

    pub fn binding_index(&self, bindings: Term) -> Option<usize> {
        todo!()
    }
}

impl Constraint {
    pub fn is_rigid_rigid(&self) -> bool {
        let (_, l_head, _) = self.left.split();
        let (_, r_head, _) = self.right.split();

        l_head.is_rigid() && r_head.is_rigid()
    }
}