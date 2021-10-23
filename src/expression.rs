use evalexpr::error::{EvalexprError, EvalexprResult};
use evalexpr::{eval_with_context, ContextWithMutableVariables, HashMapContext, Value};

pub fn run_expression<T: IntoIterator<Item = (String, String)>>(
    input: &str,
    vars: T,
) -> Result<bool, EvalexprError> {
    let mut ctx = HashMapContext::default();
    for (mut key, value) in vars {
        key.insert(0, '$');
        ctx.set_value(key, value.into())?;
    }
    let result = eval_with_context(input, &ctx);

    is_success(result)
}

pub fn is_success(res: EvalexprResult<Value>) -> Result<bool, EvalexprError> {
    match res {
        Err(EvalexprError::VariableIdentifierNotFound(_)) => Ok(false),
        Err(e) => Err(e),
        Ok(Value::Boolean(x)) => Ok(x),
        Ok(Value::Int(0)) => Ok(true),
        Ok(Value::Int(_)) => Ok(false),
        Ok(_) => Ok(false),
    }
}

#[test]
fn variable_substitution_works_variable_found_equal() {
    let vars = vec![(String::from("TEST_VAR"), String::from("testing"))];

    let res = run_expression("$TEST_VAR == \"testing\"", vars);

    assert_eq!(Ok(true), res);
}

#[test]
fn variable_substitution_works_variable_found_not_equal() {
    let vars = vec![(String::from("TEST_VAR"), String::from("testing"))];

    let res = run_expression("$TEST_VAR != \"testing\"", vars);

    assert_eq!(Ok(false), res);
}

#[test]
fn variable_substitution_works_not_found_variable_returns_false() {
    let vars = vec![];

    let res = run_expression("$TEST_VAR == \"testing\"", vars);

    assert_eq!(Ok(false), res);
}
