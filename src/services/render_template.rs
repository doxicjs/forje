use tera::{Context, Tera};

use crate::services::get_template_path::get_template_path;
use crate::utils::tera_filters::register_filters;

pub fn render_template(path: &str, data: &Context) {
  let template_pattern = get_template_path().into_os_string().into_string().unwrap() + "**/*.tera";

  let mut tera = Tera::new(&template_pattern).unwrap();
  register_filters(&mut tera);

  println!("{}", tera.render(path, data).unwrap());
}
