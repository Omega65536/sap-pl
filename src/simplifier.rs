use crate::formula::Formula;

pub fn simplify(formula: &Formula) -> Formula {
    match &formula {
        Formula::Variable(_) => formula.clone(),
        Formula::And(args) => {
            let mut simpl_args = Vec::new();
            for arg in args {
                let simpl_arg = simplify(arg);
                simpl_args.push(simpl_arg);
            }
            Formula::And(simpl_args)
        },
        Formula::Or(args) => {
            let mut simpl_args = Vec::new();
            for arg in args {
                let simpl_arg = simplify(arg);
                simpl_args.push(simpl_arg);
            }
            Formula::Or(simpl_args)
        },
        Formula::Not(arg) => {
            match &**arg {
                Formula::Variable(_) => Formula::Not(Box::new(*arg.clone())),
                Formula::And(args) => {
                    let mut simpl_neg_args = Vec::new();
                    for arg in args {
                        let neg_arg = Formula::Not(Box::new(arg.clone()));
                        let simpl_neg_arg = simplify(&neg_arg);
                        simpl_neg_args.push(simpl_neg_arg);
                    }
                    Formula::Or(simpl_neg_args)
                },
                Formula::Or(args) => {
                    let mut simpl_neg_args = Vec::new();
                    for arg in args {
                        let neg_arg = Formula::Not(Box::new(arg.clone()));
                        let simpl_neg_arg = simplify(&neg_arg);
                        simpl_neg_args.push(simpl_neg_arg);
                    }
                    Formula::And(simpl_neg_args)
                },
                Formula::Not(arg) => {
                    simplify(arg)
                } 
            }
        },
    }
}
