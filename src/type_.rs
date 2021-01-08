use std::rc::Rc;

#[derive(PartialEq,Debug,Clone)]
pub enum Type {
   Bool,
   Int,
   Function {input: Rc<Type>, output: Rc<Type>},
   Unit,
   Variant {vars: Vec<(String, Type)>},
   Record {recs: Vec<(String, Type)>},
   Universal {name: String, tau: Rc<Type>},
   Existential{name: String, as_rec: Rc<Type>},
   Variable{name: String},
}
