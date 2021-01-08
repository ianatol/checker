use super::type_::Type;
use std::rc::Rc;
use std::collections::HashSet;
use super::expr::Expression;

pub fn type_of(expr: Rc<Expression>, ctx:&mut Vec<(String, Type)>) -> Result<Type, String> {
    match &*expr {
        Expression::True => Ok(Type::Bool),
        Expression::False => Ok(Type::Bool),
        Expression::If{guard, then_, else_} => {
            match type_of(guard.clone(), ctx) {
                Ok(Type::Bool) => {
                    let then_type = type_of(then_.clone(), ctx);
                    match then_type {
                        Ok(t) => {
                            let else_type = type_of(else_.clone(), ctx);
                            match else_type {
                                Ok(e) => {
                                    if is_subtype(e.clone(), t.clone()) {
                                        Ok(t.clone())
                                    }
                                    else if is_subtype(t.clone(), e.clone()){
                                        Ok(e.clone())
                                    }
                                    else {
                                        Err(format!("If statement expected then and else types to match."))
                                    }
                                },
                                Err(e) => return Err(e),
                            }
                        },
                        Err(e) => return Err(e),
                    }
                },
                Ok(_) => {
                    Err(format!("If statement expected Bool"))
                },
                Err(e) => return Err(e),
            }
        },
        Expression::Num(_) => Ok(Type::Int),
        Expression::Add {left, right} => {
            let left_type = type_of(left.clone(), ctx);
            match left_type {
                Ok(Type::Int) => {
                    let right_type = type_of(right.clone(), ctx);
                    match right_type {
                        Ok(Type::Int) => Ok(Type::Int),
                        Ok(_) => Err(format!("Add expected Int for right operand")),
                        Err(e) => return Err(e),
                    }
                    //match right type, if also int return int
                }
                Ok(_) => Err(format!("Add expected Int for left operand")),
                Err(e) => return Err(e),
            }
        },
        Expression::Sub {left, right} => {
            let left_type = type_of(left.clone(), ctx);
            match left_type {
                Ok(Type::Int) => {
                    let right_type = type_of(right.clone(), ctx);
                    match right_type {
                        Ok(Type::Int) => Ok(Type::Int),
                        Ok(_) => Err(format!("Sub expected Int for right operand")),
                        Err(e) => return Err(e),
                    }
                    //match right type, if also int return int
                }
                Ok(_) => Err(format!("Sub expected Int for left operand")),
                Err(e) => return Err(e),
            }
        },
        Expression::IsZero(term) => {
            let term_type = type_of(term.clone(), ctx);
            match term_type {
                Ok(Type::Int) => Ok(Type::Bool),
                Ok(_) => Err(format!("IsZero expected Int.")),
                Err(e) => return Err(e),
            }
        },
        Expression::Var{name} => {
            //Check context for name already
            let ctx_copy = ctx.clone();
            let gamma_iter = ctx_copy.iter();
            let found_iter = gamma_iter.filter(|(x, _y)| x == name);
            for (found_name, found_type) in found_iter.rev() {
                ctx.push((found_name.clone(), found_type.clone()));
                return Ok(found_type.clone())
            }
            //if not in context, error - free variable without a lambda type
            return Err(format!("Free variable without type in context or lambda"))
        },
        Expression::Abs{input_var, input_var_type, term} => {
            let temp_var = input_var.clone();
            //push input_var to context
            match &*temp_var {
                Expression::Var{name} => {
                    ctx.push((name.clone(), input_var_type.clone()))
                },
                _ => {
                    return Err(format!("Abs not given a variable type"))
                }
            }
            
            let term_type = type_of(term.clone(), ctx);
            match term_type {
                Ok(out_t) => Ok(Type::Function{input: Rc::new(input_var_type.clone()), output: Rc::new(out_t)}), //nervous about this line
                Err(e) => return Err(e),
            }
        },
        Expression::App{left, right} => {
            let left_type = type_of(left.clone(), ctx);
            match left_type {
                Ok(Type::Function{input: a, output: b}) => {
                    let right_type = type_of(right.clone(), ctx);
                    match right_type {
                        Ok(t) => {
                            if is_subtype(t.clone(), (*a).clone()) {
                                let return_type = (*b).clone();
                                Ok(return_type)
                            }
                            else {
                                Err(format!("t2 did not match input of t1"))
                            }
                        }
                        Err(e) => return Err(e)
                    }
                },
                Ok(_) => Err(format!("Expected t1 of type Function")),
                Err(e) => return Err(e),
            }
        },
        Expression::Variant{input, vars} => 
        {
            let input_type = type_of(input.1.clone(), ctx)?;
            for (label, t) in vars {
                if label == &input.0 {
                    if *t == input_type {
                        return Ok(Type::Variant{vars: vars.clone()})
                    }
                    else {
                        return Err(format!("Input variant value did not match type of corresponding label in variant"))
                    }
                }
            }
            Err(format!("Input variant did not have a matching label to any variant"))
        },
        Expression::Case{vars, cases} => {
            if vars.len() != cases.len() {
                return Err(format!("Improper number of cases to match variant"))
            }
            ctx.push((cases[0].0.clone(), vars[0].1.clone()));
            let case_expr_type = type_of(cases[0].1.clone(), ctx);
            for i in 0..vars.len() {
                ctx.push((cases[i].0.clone(), vars[i].1.clone()));
                if type_of(cases[i].1.clone(), ctx) != case_expr_type {
                    return Err(format!("Case expression types do not match"))
                }
            }
            Ok(case_expr_type?)
            //for each label in vars
            //add case var to context with label type
            //get type of case expression
            //if all case expressions are same type, return that
        },
        Expression::ValRecord{recs} => {
            let mut record:Vec<(String, Type)> = Vec::new();
            for (label, expr) in recs {
                record.push((label.to_string(), type_of(expr.clone(), ctx)?));
            }
            Ok(Type::Record{recs: record.clone()})
        },
        Expression::TypeRecord{recs} => {
            Ok(Type::Record{recs: recs.clone()})
        }
        Expression::Proj{expr, label} => {
            let rec = type_of(expr.clone(), ctx)?;
            match rec {
                Type::Record{recs} => {
                    for (name, type_) in recs {
                        if name == label.clone() {
                            return Ok(type_)
                        }
                    }
                    Err(format!("Label for projection not found in record"))
                }
                _ => {
                    Err(format!("Projection called on expression not of record type"))
                }
            }
        },
        Expression::TAbs{name, type_expr} => {
            let tabs_type = type_of(type_expr.clone(), ctx)?;
            ctx.push((name.clone(), Type::Variable{name: name.clone()}));
            Ok(Type::Universal{name: name.clone(),  tau: Rc::new(tabs_type)})
        },
        Expression::TApp{univ_term, input_type} => {
            let univ_term_type = type_of(univ_term.clone(), ctx)?;
            match univ_term_type {
                Type::Universal{name, tau} => {
                    return Ok(type_sub(&name, &input_type, (*tau).clone()))
                }
                _ => Err(format!("TApp given a non universal type for t1"))
            }
        },
        Expression::Pack{u, t, exis_type} => {
            let type_of_t = type_of((*t).clone(), ctx)?;
            match (**exis_type).clone() {
                Type::Existential{name, as_rec} => {
                    let exis_type_tau = type_sub(&name, &(*u).clone(), (*as_rec).clone());
                    if type_of_t == exis_type_tau {
                        return Ok((**exis_type).clone())
                    }
                },
                _ => return Err(format!("Tried to pack a non-existential type"))
            }
            Err(format!("Types of existential type interface and implementation did not match"))
        }
        Expression::Unpack{type_var, x, t1, t2} => {
            let type_var_name;
            match type_var {
                Type::Variable{name} => type_var_name = name,
                _ => return Err(format!("Unpack called with type_var not as var"))
            }
            //typecheck t1 - should be exis type
            let type_of_t1 = type_of((*t1).clone(), ctx)?;
            match type_of_t1 {
                Type::Existential{name, as_rec} => {
                    if type_var_name == &name {
                        ctx.push((x.clone(), (*as_rec).clone()));
                    }
                },
                _ => return Err(format!("Unpack called with t1 not as existential type"))
            }
            //check type_var matches name from t1
            //push x to context with tau from t1
            //typecheck t2
            let type_of_t2 = type_of((*t2).clone(), ctx)?;
            //return t2's type
            Ok(type_of_t2)
        }
    }
}

pub fn is_subtype(a: Type, b: Type) -> bool {
        if a == b {
            return true
        }
        else {
            match (a,b) {
                (Type::Record{recs: rec_a}, Type::Record{recs: rec_b}) => {
                    //Check that b's labels are an actual set subset of a's labels
                    let mut a_labels:HashSet<String> = HashSet::new();
                    let mut b_labels:HashSet<String> = HashSet::new();
                    for (label, _) in rec_a.clone() {
                        a_labels.insert(label);
                    }
                    for (label, _) in rec_b.clone() {
                        b_labels.insert(label);
                    }
                    if b_labels.is_subset(&a_labels) {
                        for (b_label, b_t) in rec_b {
                            for (a_label, a_t) in &rec_a {
                                if *a_label == b_label {
                                    if !is_subtype(a_t.clone(), b_t.clone()) {
                                        return false
                                    }
                                }
                            }
                            //find label in rec_a and get its type: a_t
                            //is_subtype(a_t, b_t) 
                        }
                    }
                    else {
                        return false
                    }
                    //For each label in b, check that the corresponding a-label-type is a subtype of the b-label-type
                    //If all that succeeds then return true 😎
                    return true
                },
                (Type::Function{input: a_in, output: a_out}, Type::Function{input: b_in, output: b_out}) => {
                    //check that b_in is a subtype of a_in 
                    //check that a_out is a subtype of b_out
                    return is_subtype((*b_in).clone(), (*a_in).clone()) && is_subtype((*a_out).clone(), (*b_out).clone())
                }
                (_, _) => {
                    return false
                }
            }
        }
}

pub fn type_sub(var_name: &String, input_type: &Type, tau: Type) -> Type {
    let subbed_type;
    //match tau
    match tau {
        Type::Bool => subbed_type = Type::Bool,
        Type::Int => subbed_type = Type::Int,
        Type::Unit => subbed_type = Type::Unit,
        Type::Function{input, output} => {
            subbed_type = Type::Function{
                input: Rc::new(type_sub(var_name, input_type, (*input).clone())),
                output: Rc::new(type_sub(var_name, input_type, (*output).clone()))};
        }
        Type::Variant{vars} => {
            let mut new_vars:Vec<(String, Type)> = Vec::new();
            for (vars_name, var_type) in vars {
                new_vars.push((vars_name, type_sub(var_name, input_type, var_type)));
            }
            subbed_type = Type::Variant{vars: new_vars};
        }
        Type::Record{recs} => {
            let mut new_recs:Vec<(String, Type)> = Vec::new();
            for (rec_name, rec_type) in recs {
                new_recs.push((rec_name, type_sub(var_name, input_type, rec_type)));
            }
            subbed_type = Type::Record{recs: new_recs};
        }
        Type::Universal{name, tau} => {
            subbed_type = Type::Universal {
                name: name,
                tau: Rc::new(type_sub(var_name, input_type, (*tau).clone()))
            }
        }
        Type::Existential{name, as_rec} => {
            subbed_type = Type::Existential {
                name: name,
                as_rec: Rc::new(type_sub(var_name, input_type, (*as_rec).clone()))
            }
        }
        Type::Variable{name} => {
            if name == *var_name {
                subbed_type = (*input_type).clone()
            }
            else {
                subbed_type = Type::Variable{name: name}
            }
        }
    }
    return subbed_type
    //if base type (unit, bool, int), return the same
    //if variable type, check name and replace with input_type if same
    //for anything with nested types, recurse
}