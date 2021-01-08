use std::rc::Rc;
use super::type_::Type;

use super::expr::Expression;

pub fn true_e() -> Rc<Expression> {
   Rc::new(Expression::True)
}

pub fn false_e() -> Rc<Expression> {
   Rc::new(Expression::False)
}

pub fn num_e(n: i32) -> Rc<Expression> {
   Rc::new(Expression::Num(n))
}

pub fn if_e(guard: Rc<Expression>, then_: Rc<Expression>, else_: Rc<Expression>)
   -> Rc<Expression> {
   Rc::new(Expression::If {guard, then_, else_,})
}

pub fn add_e(left: Rc<Expression>, right: Rc<Expression>) -> Rc<Expression> {
   Rc::new(Expression::Add {left, right})
}

pub fn sub_e(left: Rc<Expression>, right: Rc<Expression>) -> Rc<Expression> {
   Rc::new(Expression::Sub {left, right})
}

pub fn is_zero_e(exp: Rc<Expression>) -> Rc<Expression> {
   Rc::new(Expression::IsZero(exp))
}

pub fn var_e(name: String) -> Rc<Expression> {
   Rc::new(Expression::Var {name})
}

pub fn abs_e(input_var: Rc<Expression>, input_var_type: Type, term: Rc<Expression>) -> Rc<Expression> {
   Rc::new(Expression::Abs {input_var, input_var_type, term})
}

pub fn app_e(left: Rc<Expression>, right: Rc<Expression>) -> Rc<Expression> {
   Rc::new(Expression::App {left, right})
}

pub fn variant_e(input: (String, Rc<Expression>), vars: Vec<(String, Type)>) -> Rc<Expression> {
   Rc::new(Expression::Variant {input, vars})
}

pub fn case_e(vars: Vec<(String, Type)>, cases: Vec<(String, Rc<Expression>)>) -> Rc<Expression> {
   Rc::new(Expression::Case {vars, cases})
}

pub fn val_record_e(recs: Vec<(String, Rc<Expression>)>) -> Rc<Expression> {
   Rc::new(Expression::ValRecord {recs})
}

pub fn type_record_e(recs: Vec<(String, Type)>) -> Rc<Expression> {
   Rc::new(Expression::TypeRecord {recs})
}

pub fn proj_e(expr: Rc<Expression>, label: String) -> Rc<Expression> {
   Rc::new(Expression::Proj{expr, label})
}

pub fn tabs_e(name: String, type_expr: Rc<Expression>) -> Rc<Expression> {
   Rc::new(Expression::TAbs{name, type_expr})
}

pub fn tapp_e(univ_term: Rc<Expression>, input_type: Type) -> Rc<Expression> {
   Rc::new(Expression::TApp{univ_term, input_type})
}

pub fn pack_e(u: Rc<Type>, t: Rc<Expression>, exis_type: Rc<Type>) -> Rc<Expression> {
   Rc::new(Expression::Pack{u, t, exis_type})
}

pub fn unpack_e(type_var: Type, x: String, t1: Rc<Expression>, t2: Rc<Expression>) -> Rc<Expression> {
   Rc::new(Expression::Unpack{type_var, x, t1, t2})
}

pub fn bool_t() -> Type {
   return Type::Bool
}

pub fn int_t() -> Type {
   return Type::Int
}

pub fn fun_t(input: Type, output: Type) -> Type {
   return Type::Function{input: Rc::new(input), output: Rc::new(output)}
}

pub fn unit_t() -> Type {
   return Type::Unit
}

pub fn variant_t(vars: Vec<(String, Type)>) -> Type {
   return Type::Variant{vars: vars}
}

pub fn record_t(recs: Vec<(String, Type)>) -> Type {
   return Type::Record{recs: recs}
}

pub fn exis_t(name: String, as_rec: Type) -> Type {
   return Type::Existential{name: name, as_rec: Rc::new(as_rec)}
}

pub fn univ_t(name: String, tau: Type) -> Type {
   return Type::Universal{name: name, tau: Rc::new(tau)}
}

pub fn var_t(name: String) -> Type {
   return Type::Variable{name: name}
}


#[cfg(test)]
mod tests {
   use super::{*,Expression::*};

   #[test]
   fn num_eq() {
      assert_eq!(*num_e(0), Num(0));
   }

   #[test]
   fn true_eq() {
      assert_eq!(*true_e(), True);
   }

   #[test]
   fn false_eq() {
      assert_eq!(*false_e(), False);
   }

   #[test]
   fn if_eq() {
      assert_eq!(
         *if_e(true_e(), num_e(0), num_e(1)),
         If {guard: true_e(), then_: num_e(0), else_: num_e(1)});
   }

   #[test]
   fn add_eq() {
      assert_eq!(
         *add_e(num_e(9), num_e(4)),
         Add {left: num_e(9), right: num_e(4)});
   }

   #[test]
   fn sub_eq() {
      assert_eq!(
         *sub_e(num_e(9), num_e(4)),
         Sub {left: num_e(9), right: num_e(4)});
   }

   #[test]
   fn is_zero_eq() {
      assert_eq!(
         *is_zero_e(num_e(4)),
         IsZero(num_e(4)));
   }
}
