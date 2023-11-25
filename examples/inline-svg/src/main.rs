use silkenweb::{elements::html::div, mount, prelude::ParentElement};
use silkenweb_inline_svg::{svg_dir, svg_file};

mod svg {
    super::svg_dir!("svg");
}

svg_file!("svg/test-image.svg");

fn main() {
    mount("app", div().children([test_image(), svg::test_image()]));
}