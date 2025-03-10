use std::collections::HashMap;

use heck::{
  ToKebabCase, ToLowerCamelCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase,
  ToUpperCamelCase,
};
use tera::{Result, Tera, Value, from_value, to_value};

// Snake Cases
fn shouty_snake_case_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
  let input = from_value::<String>(value.clone()).map_err(|_| {
    tera::Error::msg("Invalid input for shouty_snake_case filter; expected a string")
  })?;
  Ok(to_value(input.to_shouty_snake_case()).unwrap())
}

fn snake_case_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
  let input = from_value::<String>(value.clone())
    .map_err(|_| tera::Error::msg("Invalid input for snake_case filter; expected a string"))?;
  Ok(to_value(input.to_snake_case()).unwrap())
}

// Kebab Cases
fn shouty_kebab_case_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
  let input = from_value::<String>(value.clone()).map_err(|_| {
    tera::Error::msg("Invalid input for shouty_kebab_case filter; expected a string")
  })?;
  Ok(to_value(input.to_shouty_kebab_case()).unwrap())
}

fn kebab_case_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
  let input = from_value::<String>(value.clone())
    .map_err(|_| tera::Error::msg("Invalid input for kebab_case filter; expected a string"))?;
  Ok(to_value(input.to_kebab_case()).unwrap())
}

// Camel Cases
fn upper_camelcase_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
  let input = from_value::<String>(value.clone())
    .map_err(|_| tera::Error::msg("Invalid input for upper_camelcase filter; expected a string"))?;
  Ok(to_value(input.to_upper_camel_case()).unwrap())
}

fn lower_camelcase_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
  let input = from_value::<String>(value.clone())
    .map_err(|_| tera::Error::msg("Invalid input for lower_camelcase filter; expected a string"))?;
  Ok(to_value(input.to_lower_camel_case()).unwrap())
}

pub fn register_filters(tera: &mut Tera) {
  // Snake Cases
  tera.register_filter("to_up_snake", shouty_snake_case_filter);
  tera.register_filter("to_snake", snake_case_filter);
  // Kebab Cases
  tera.register_filter("to_up_kebab", shouty_kebab_case_filter);
  tera.register_filter("to_kebab", kebab_case_filter);
  // Camel Cases
  tera.register_filter("to_up_camel", upper_camelcase_filter);
  tera.register_filter("to_camel", lower_camelcase_filter);
}
