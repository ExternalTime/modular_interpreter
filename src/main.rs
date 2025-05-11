mod language;
mod bool;
mod lexical_lambda;
mod record;

compose_lang!{Expr, Val, Ctx, Err, [
    bool::BoolExpr<Expr> as bool,
    lexical_lambda::LamExpr<Expr> as lambda,
    record::RecordExpr<Expr> as record,
]}

fn main() {
    let literal = Expr::from(bool::BoolExpr::Literal(true));
    let record = Expr::from(record::RecordExpr::Make(vec![("foo".to_owned(), literal)]));
    let access = Expr::from(record::RecordExpr::Field(record, "bar".to_owned()));
    println!("{:?}", access.eval(&mut Default::default()));
}
