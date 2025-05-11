use super::language::*;

#[derive(Clone, Debug)]
pub enum BoolExpr<T> {
    Literal(bool),
    If(T, T, T),
    And(T, T),
    Or(T, T),
    Not(T),
}

impl<L: LangComposition<Self>> LangFragment<L> for BoolExpr<L> {
    type Val = bool;
    type Ctx = ();
    type Err = std::convert::Infallible;

    fn eval(self, ctx: &mut L::Ctx) -> Result<L::Val, L::Err> {
        match self {
            Self::Literal(b) => Ok(L::wrap_val(b)),
            Self::If(cond, t, e) => {
                let cond = L::unwrap_val(cond.eval(ctx)?)?;
                match cond { true => t, false => e }.eval(ctx)
            },
            Self::And(b1, b2) => {
                let b1 = L::unwrap_val(b1.eval(ctx)?)?;
                Ok(L::wrap_val(b1 && L::unwrap_val(b2.eval(ctx)?)?))
            },
            Self::Or(b1, b2) => {
                let b1 = L::unwrap_val(b1.eval(ctx)?)?;
                Ok(L::wrap_val(b1 || L::unwrap_val(b2.eval(ctx)?)?))
            },
            Self::Not(b) => Ok(L::wrap_val(!L::unwrap_val(b.eval(ctx)?)?)),
        }
    }
}
