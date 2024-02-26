extern crate rustler;

mod atoms;
mod errors;
mod expression;
mod value;

rustler::init!(
    "Elixir.ZenEngine",
    [
        expression::evaluate_expression,
        expression::evaluate_unary_expression,
    ]
);
