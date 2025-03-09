use std::path::PathBuf;

pub fn get_template_path() -> PathBuf {
  let base_path_result = std::env::current_dir();

  let base_path = match base_path_result {
    Ok(base_path) => base_path,
    Err(error) => panic!("Error retrieving current dir path: {error:?}"),
  };

  let template_path = base_path.join("src/templates");

  return template_path;
}
