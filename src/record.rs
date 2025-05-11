use super::language::*;

#[derive(Clone, Debug)]
pub struct UnknownField<T>(Vec<(String, T)>, String);

#[derive(Clone, Debug)]
pub enum RecordExpr<T> {
    Make(Vec<(String, T)>),
    Field(T, String),
}

impl<L: LangComposition<Self>> LangFragment<L> for RecordExpr<L> {
    type Val = Vec<(String, L::Val)>;
    type Ctx = ();
    type Err = UnknownField<L::Val>;

    fn eval(self, ctx: &mut L::Ctx) -> Result<L::Val, L::Err> {
        match self {
            Self::Make(vec) => {
                let mut res = Vec::with_capacity(vec.len());
                for (field, e) in vec {
                    res.push((field, e.eval(ctx)?));
                }
                Ok(L::wrap_val(res))
            },
            Self::Field(record, field) => {
                let mut record = L::unwrap_val(record.eval(ctx)?)?;
                let i = record.iter().enumerate().find_map(|(i, (f, _))| (f == &field).then_some(i));
                match i {
                    Some(i) => Ok(record.swap_remove(i).1),
                    None => Err(L::wrap_err(UnknownField(record, field))),
                }
            }
        }
    }
}
