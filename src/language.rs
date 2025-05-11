pub trait Lang: Sized {
    type Val;
    type Ctx: Default;
    type Err;

    fn eval(self, ctx: &mut Self::Ctx) -> Result<Self::Val, Self::Err>;
}

pub trait LangComposition<Frag: LangFragment<Self>>: Lang {
    fn wrap(e: Frag) -> Self;

    fn wrap_val(v: Frag::Val) -> Self::Val;
    fn unwrap_val(v: Self::Val) -> Result<Frag::Val, Self::Err>;

    fn ctx(ctx: &mut Self::Ctx) -> &mut Frag::Ctx;

    fn wrap_err(err: Frag::Err) -> Self::Err;
}

pub trait LangFragment<L: LangComposition<Self>>: Sized {
    type Val;
    type Ctx: Default;
    type Err;

    fn eval(self, ctx: &mut L::Ctx) -> Result<L::Val, L::Err>;
}

#[macro_export]
macro_rules! compose_lang {
    ($expr:ident, $val:ident, $ctx:ident, $err:ident, [$($fragment:path as $id:ident),+ $(,)?]) => {
        use crate::language::*;
        #[allow(non_camel_case_types)]
        #[derive(Clone, Debug)]
        pub enum $expr {
            $($id(Box<$fragment>), )+
        }

        #[allow(non_camel_case_types)]
        #[derive(Clone, Debug)]
        pub enum $val {
            $($id(Box<<$fragment as LangFragment<$expr>>::Val>), )+
        }

        #[allow(non_camel_case_types)]
        #[derive(Debug)]
        pub enum $err {
            TypeMismatch(&'static str, $val),
            $($id(Box<<$fragment as LangFragment<$expr>>::Err>), )+
        }

        #[derive(Clone, Debug, Default)]
        pub struct $ctx {
            $($id: <$fragment as LangFragment<$expr>>::Ctx, )+
        }

        impl Lang for $expr {
            type Val = $val;
            type Ctx = $ctx;
            type Err = $err;

            fn eval(self, ctx: &mut Self::Ctx) -> Result<Self::Val, Self::Err> {
                match self {
                    $($expr::$id(e) => <$fragment as LangFragment<$expr>>::eval(*e, ctx), )+
                }
            }
        }

        $(
            impl From<$fragment> for $expr {
                fn from(v: $fragment) -> Self {
                    Self::$id(Box::new(v))
                }
            }
        )+

        $(
            impl LangComposition<$fragment> for $expr {
                fn wrap(e: $fragment) -> Self {
                    $expr::$id(Box::new(e))
                }

                fn wrap_val(v: <$fragment as LangFragment<$expr>>::Val) -> Self::Val {
                    $val::$id(Box::new(v))
                }
                fn unwrap_val(v: Self::Val) -> Result<<$fragment as LangFragment<$expr>>::Val, Self::Err> {
                    match v {
                        $val::$id(v) => Ok(*v),
                        _ => Err($err::TypeMismatch(stringify!($id), v)),
                    }
                }

                fn ctx(ctx: &mut Self::Ctx) -> &mut <$fragment as LangFragment<$expr>>::Ctx {
                    &mut ctx.$id
                }

                fn wrap_err(err: <$fragment as LangFragment<$expr>>::Err) -> Self::Err {
                    $err::$id(Box::new(err))
                }
            }
        )+
    }
}
