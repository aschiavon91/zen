use rustler::{Atom, Env, Term};

use crate::{atoms, value};

#[rustler::nif(schedule = "DirtyCpu")]
pub fn evaluate_expression<'a>(
    env: Env<'a>,
    expression: &str,
    ctx: Term<'a>,
) -> Result<Term<'a>, Atom> {
    let context = value::to_value(&ctx, env).or(Err(atoms::unsupported_type()))?;

    let result = zen_expression::evaluate_expression(expression, &context)
        .or(Err(atoms::execution_error()))?;

    value::from_value(result, env).or(Err(atoms::invalid_result()))
}

#[rustler::nif(schedule = "DirtyCpu")]
pub fn evaluate_unary_expression<'a>(
    env: Env<'a>,
    expression: &str,
    ctx: Term<'a>,
) -> Result<bool, Atom> {
    let context = value::to_value(&ctx, env).or(Err(atoms::unsupported_type()))?;

    let result = zen_expression::evaluate_unary_expression(expression, &context)
        .or(Err(atoms::execution_error()))?;

    Ok(result)
}
