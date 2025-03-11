use crate::{TEMPLATES_DIR, utils::tera_filters::register_filters};
use lazy_static::lazy_static;
use tera::{Context, Tera};

lazy_static! {
  pub static ref TEMPLATE_ENGINE: Tera = {
    let mut tera = Tera::default();
    register_filters(&mut tera);
    for entry in TEMPLATES_DIR.find("**/*.tera").unwrap() {
      let current_path = entry.path().display().to_string();

      let _ = tera.add_raw_template(
        &current_path,
        TEMPLATES_DIR
          .get_file(&current_path)
          .unwrap()
          .contents_utf8()
          .unwrap(),
      );
    }
    tera
  };
}

pub fn render_template(path: &str, data: &Context) {
  println!("{}", TEMPLATE_ENGINE.render(path, data).unwrap());
}
