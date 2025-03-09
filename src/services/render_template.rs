use tera::{Context, Tera};

use crate::services::get_template_path::get_template_path;

pub fn render_template(path: String, data: Context) {
  let fetch_path = get_template_path()
    .join(path)
    .into_os_string()
    .into_string()
    .unwrap();

  let mut tera = Tera::default();

  tera
    .add_template_file(fetch_path, Some("template"))
    .unwrap();

  println!("{}", tera.render("template", &data).unwrap());
}
