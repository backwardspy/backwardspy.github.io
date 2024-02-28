use std::collections::HashMap;

use tera::{Error, Value};

pub(crate) fn lines(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let value = value
        .as_str()
        .ok_or_else(|| Error::msg("expected string value"))?;
    let lines = value.lines().collect::<Vec<_>>();
    Ok(lines.into())
}
