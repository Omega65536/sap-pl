use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Formula {
    Variable(String),
    And(Vec<Formula>),
    Or(Vec<Formula>),
    Not(Box<Formula>),
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Formula::Variable(name) => write!(f, "{}", name),
            Formula::And(args) => write!(f, "({})", args.iter().map(|arg| format!("{}", arg)).collect::<Vec<String>>().join(" \u{2227} ")),
            Formula::Or(args) => write!(f, "({})", args.iter().map(|arg| format!("{}", arg)).collect::<Vec<String>>().join(" \u{2228} ")),
            Formula::Not(arg) => write!(f, "\u{00AC}{}", arg),
        }
    }
}
