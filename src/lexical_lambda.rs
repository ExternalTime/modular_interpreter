use super::language::*;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct UnknownIdentifier(pub String);

#[derive(Clone, Debug)]
struct Node<V> {
    ident: String,
    value: V,
    next: Option<Rc<Node<V>>>,
}

#[derive(Debug)]
pub struct Context<V>(Option<Rc<Node<V>>>);

impl<V> Clone for Context<V> {
    fn clone(&self) -> Self {
        Self(self.0.as_ref().map(Rc::clone))
    }
}

impl<V> Default for Context<V> {
    fn default() -> Self {
        Self(None)
    }
}

impl<V> Context<V> {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn cons(&self, ident: String, value: V) -> Self {
        Self(Some(Rc::new(Node {
            ident,
            value,
            next: self.0.clone(),
        })))
    }

    pub fn get(&self, ident: &str) -> Result<&V, UnknownIdentifier> {
        let mut next = &self.0;
        while let Some(node) = next {
            if node.ident == ident {
                return Ok(&node.value);
            }
            next = &node.next;
        }
        Err(UnknownIdentifier(ident.to_owned()))
    }
}

#[derive(Clone, Debug)]
pub struct Lam<E, V> {
    ctx: Context<V>,
    ident: String,
    body: E,
}

#[derive(Clone, Debug)]
pub enum LamExpr<E> {
    Lam(String, E),
    Var(String),
    App(E, E),
}

impl<L: LangComposition<Self>> LangFragment<L> for LamExpr<L>
where
    L::Val: Clone,
{
    type Val = Lam<L, L::Val>;
    type Ctx = Context<L::Val>;
    type Err = UnknownIdentifier;

    fn eval(self, ctx: &mut L::Ctx) -> Result<L::Val, L::Err> {
        match self {
            Self::Lam(ident, body) => Ok(L::wrap_val(Lam { ctx: L::ctx(ctx).clone(), ident, body })),
            Self::Var(ident) => L::ctx(ctx).get(&ident).cloned().map_err(L::wrap_err),
            Self::App(f, x) => {
                let Lam { ctx: tmp, ident, body } = L::unwrap_val(f.eval(ctx)?)?;
                let x = x.eval(ctx)?;
                let mut tmp = tmp.cons(ident, x);
                std::mem::swap(&mut tmp, L::ctx(ctx));
                let res = body.eval(ctx);
                std::mem::swap(&mut tmp, L::ctx(ctx));
                res
            },
        }
    }
}
