use std::rc::Rc;
use super::type_::Type;

#[derive(PartialEq,Debug)]
pub enum Expression {
   True,
   False,
   If {guard: Rc<Expression>, then_: Rc<Expression>, else_: Rc<Expression>},
   Num (i32),
   Add {left: Rc<Expression>, right: Rc<Expression>},
   Sub {left: Rc<Expression>, right: Rc<Expression>},
   IsZero (Rc<Expression>),
   Var{name: String},
   Abs{input_var: Rc<Expression>, input_var_type: Type, term: Rc<Expression>},
   App{left: Rc<Expression>, right: Rc<Expression>},
   Variant{input: (String, Rc<Expression>), vars: Vec<(String, Type)>},
   Case{vars: Vec<(String, Type)>, cases: Vec<(String, Rc<Expression>)>}, //vars: name, type; cases: var, resulting expression
   ValRecord{recs: Vec<(String, Rc<Expression>)>},
   TypeRecord{recs: Vec<(String, Type)>},
   Proj{expr: Rc<Expression>, label: String},
   TAbs{name: String, type_expr: Rc<Expression>},
   TApp{univ_term: Rc<Expression>, input_type: Type},
   Pack{u: Rc<Type>, t: Rc<Expression>, exis_type: Rc<Type>},
   Unpack{type_var: Type, x: String, t1: Rc<Expression>, t2: Rc<Expression>}
}  

