use syn::{Ident, Expr};

pub struct OracleArg {
    pub ident: Ident,
    pub distrib: Ident,
    pub distrib_args: Vec<Expr>,
}

