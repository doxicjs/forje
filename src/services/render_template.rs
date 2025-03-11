use crate::utils::tera_filters::register_filters;
use tera::{Context, Tera};

pub fn render_template(path: &str, data: &Context) {
  let mut tera = Tera::new("src/templates/**/*.tera").unwrap();
  register_filters(&mut tera);

  println!("{}", tera.render(path, data).unwrap());
}
