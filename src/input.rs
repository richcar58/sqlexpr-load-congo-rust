use serde::{Deserialize, Serialize};
use sqlexpr_congo_rust::RuntimeValue;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExpressionTest {
    pub expr: String,
    pub value_map: HashMap<String, serde_json::Value>,
}

impl ExpressionTest {
    /// Convert the JSON value_map to a HashMap of RuntimeValues
    /// suitable for passing to sqlexpr_rust::evaluate
    pub fn to_runtime_map(&self) -> Result<HashMap<String, RuntimeValue>, String> {
        self.value_map
            .iter()
            .map(|(key, val)| {
                json_value_to_runtime(val)
                    .map(|runtime_val| (key.clone(), runtime_val))
            })
            .collect()
    }
}

/// Load expression tests from a JSON file
pub fn load_tests(path: &str) -> Result<Vec<ExpressionTest>, anyhow::Error> {
    let content = fs::read_to_string(path)?;
    let tests: Vec<ExpressionTest> = serde_json::from_str(&content)?;
    Ok(tests)
}

/// Convert a serde_json::Value to a RuntimeValue
fn json_value_to_runtime(val: &serde_json::Value) -> Result<RuntimeValue, String> {
    match val {
        serde_json::Value::Null => Ok(RuntimeValue::Null),
        serde_json::Value::Bool(b) => Ok(RuntimeValue::Boolean(*b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(RuntimeValue::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(RuntimeValue::Float(f))
            } else {
                Err(format!("Unsupported number format: {}", n))
            }
        }
        serde_json::Value::String(s) => Ok(RuntimeValue::String(s.clone())),
        _ => Err(format!("Unsupported JSON value type: {:?}", val)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_to_runtime_null() {
        let val = serde_json::Value::Null;
        let result = json_value_to_runtime(&val).unwrap();
        matches!(result, RuntimeValue::Null);
    }

    #[test]
    fn test_json_to_runtime_bool() {
        let val = serde_json::json!(true);
        let result = json_value_to_runtime(&val).unwrap();
        matches!(result, RuntimeValue::Boolean(true));
    }

    #[test]
    fn test_json_to_runtime_integer() {
        let val = serde_json::json!(42);
        let result = json_value_to_runtime(&val).unwrap();
        matches!(result, RuntimeValue::Integer(42));
    }

    #[test]
    fn test_json_to_runtime_float() {
        let val = serde_json::json!(3.14);
        let result = json_value_to_runtime(&val).unwrap();
        matches!(result, RuntimeValue::Float(_));
    }

    #[test]
    fn test_json_to_runtime_string() {
        let val = serde_json::json!("hello");
        let result = json_value_to_runtime(&val).unwrap();
        matches!(result, RuntimeValue::String(_));
    }
}
