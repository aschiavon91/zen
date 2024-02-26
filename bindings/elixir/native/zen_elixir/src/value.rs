use rustler::{types::tuple::get_tuple, Encoder, Env, Term, TermType};
use serde_json::Value;
use std::collections::HashMap;

use crate::errors::TypeEncodingError;

pub fn from_value(value: Value, env: Env) -> Result<Term, TypeEncodingError> {
    if value.is_i64() {
        value.as_i64().map_or(
            Err(TypeEncodingError::InvalidInteger),
            |v| Ok(v.encode(env)),
        )
    } else if value.is_u64() {
        value.as_u64().map_or(
            Err(TypeEncodingError::InvalidInteger),
            |v| Ok(v.encode(env)),
        )
    } else if value.is_f64() {
        value
            .as_f64()
            .map_or(Err(TypeEncodingError::InvalidFloat), |v| Ok(v.encode(env)))
    } else if value.is_boolean() {
        value.as_bool().map_or(
            Err(TypeEncodingError::InvalidBoolean),
            |v| Ok(v.encode(env)),
        )
    } else if value.is_string() {
        Ok(value.as_str().encode(env))
    } else if value.is_null() {
        Ok(value.as_null().encode(env))
    } else if value.is_array() {
        let mut errors: Vec<TypeEncodingError> = vec![];

        let term = value
            .as_array()
            .expect("is_array returned true but could not encode as array.")
            .into_iter()
            .map(|v| from_value(v.to_owned(), env).map_or(None, Some))
            .filter_map(|r| r.ok_or(|e| errors.push(e)).ok())
            .fold(Term::list_new_empty(env), |acc, value| {
                acc.list_prepend(value)
            });

        if !errors.is_empty() {
            return Err(TypeEncodingError::InvalidArrayItem);
        }

        term.list_reverse()
            .map_or(Err(TypeEncodingError::InvalidArray), Ok)
    } else if value.is_object() {
        match value.as_object() {
            Some(encoded) => {
                let mut pairs: Vec<(Term, Term)> = vec![];
                let mut errors: Vec<TypeEncodingError> = vec![];
                encoded.iter().for_each(|(k, v)| {
                    match (k.encode(env), from_value(v.clone(), env)) {
                        (k, Ok(v)) => pairs.push((k, v)),
                        (_, Err(e)) => errors.push(e),
                    }
                });

                if !errors.is_empty() {
                    return Err(TypeEncodingError::InvalidMapItem);
                }
                Term::map_from_pairs(env, &pairs).map_or(Err(TypeEncodingError::InvalidMap), Ok)
            }
            None => Err(TypeEncodingError::InvalidMap),
        }
    } else {
        Err(TypeEncodingError::UnsupportedType)
    }
}

pub fn to_value<'a>(term: &Term<'a>, env: Env<'a>) -> Result<Value, TypeEncodingError> {
    match Term::get_type(*term) {
        TermType::Binary => term
            .decode::<String>()
            .map(Value::from)
            .or(Err(TypeEncodingError::InvalidBinary)),

        TermType::Atom => term
            .decode::<bool>()
            .map(Value::from)
            .or_else(|_| {
                if *term == rustler::types::atom::nil().to_term(env) {
                    Ok(Value::from(()))
                } else {
                    term.atom_to_string().map(Value::from)
                }
            })
            .or(Err(TypeEncodingError::InvalidAtom)),

        TermType::Integer => term
            .decode::<i64>()
            .map(Value::from)
            .or(Err(TypeEncodingError::InvalidInteger)),

        TermType::Float => term
            .decode::<i64>()
            .map(Value::from)
            .or_else(|_| term.decode::<f64>().map(Value::from))
            .or(Err(TypeEncodingError::InvalidFloat)),

        TermType::Tuple => match get_tuple(*term) {
            Ok(decoded) => {
                let mut errors: Vec<TypeEncodingError> = vec![];

                let items: Vec<_> = decoded
                    .iter()
                    .map(|item| to_value(&item, env).map_or(None, Some))
                    .filter_map(|r| r.ok_or(|e| errors.push(e)).ok())
                    .collect();

                if !errors.is_empty() {
                    Err(TypeEncodingError::InvalidTupleItem)
                } else {
                    Ok(Value::from_iter(items))
                }
            }
            Err(_) => Err(TypeEncodingError::InvalidTuple),
        },

        TermType::List => match term.decode::<Vec<Term>>() {
            Ok(decoded) => {
                let mut errors = vec![];

                let result: Vec<Value> = decoded
                    .into_iter()
                    .map(|item| to_value(&item, env))
                    .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
                    .collect();

                if !errors.is_empty() {
                    return Err(TypeEncodingError::InvalidArrayItem);
                }

                Ok(Value::from_iter(result))
            }
            Err(_) => Err(TypeEncodingError::InvalidArray),
        },

        TermType::Map => match term.decode::<HashMap<Term, Term>>() {
            Ok(decoded) => {
                let mut errors: Vec<TypeEncodingError> = vec![];

                let items: HashMap<String, Value> = decoded
                    .iter()
                    .map(|(k, v)| match Term::get_type(*k) {
                        TermType::Binary => match (k.decode::<String>(), to_value(&v, env)) {
                            (Ok(decoded_key), Ok(decoded_value)) => {
                                Some((decoded_key, decoded_value))
                            }
                            _ => None,
                        },
                        TermType::Atom => match (k.atom_to_string(), to_value(&v, env)) {
                            (Ok(decoded_key), Ok(decoded_value)) => {
                                Some((decoded_key, decoded_value))
                            }
                            _ => None,
                        },
                        _ => None,
                    })
                    .filter_map(|r| r.ok_or(|e| errors.push(e)).ok())
                    .collect();

                if !errors.is_empty() {
                    return Err(TypeEncodingError::InvalidMapItem);
                }

                serde_json::to_value(&items).map_or(Err(TypeEncodingError::InvalidMap), Ok)
            }
            Err(_) => Err(TypeEncodingError::InvalidMap),
        },

        TermType::Pid => Err(TypeEncodingError::UnsupportedType),
        TermType::Port => Err(TypeEncodingError::UnsupportedType),
        TermType::Ref => Err(TypeEncodingError::UnsupportedType),
        TermType::Fun => Err(TypeEncodingError::UnsupportedType),
        TermType::Unknown => Err(TypeEncodingError::UnsupportedType),
    }
}
