pub trait Node {
    fn token_literal(&self) -> String;
    fn node_type(&self) -> Type;
    fn to_string(&self) -> String;
}

pub enum Type {
    Block,
    Expr,
}
