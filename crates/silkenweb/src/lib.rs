//! A library for building reactive single page web apps
//!
//! # Quick Start
//!
//! The best way to get started is to look at the examples. You'll need [trunk]
//! to run them. For example, to run [hello-world]:
//!
//! ```bash
//! cd examples/hello-world
//! trunk serve --open
//! ```
//!
//! - [hello-world] is a minimal example
//! - [counter] is a minimal interactive example
//! - [todomvc] is an example of a simple app
//!
//! For a more complete introduction, see
//! [Learning Silkenweb With Entirely Too Many Counters](https://silkenweb.netlify.app/)
//!
//! [trunk]: https://trunkrs.dev/
//! [hello-world]: https://github.com/silkenweb/silkenweb/tree/main/examples/hello-world
//! [counter]: https://github.com/silkenweb/silkenweb/tree/main/examples/counter
//! [todomvc]: https://github.com/silkenweb/silkenweb/tree/main/examples/todomvc

pub use silkenweb_dom::{
    after_render, element_list, mount, render_updates, tag, unmount, Builder, DomElement, Element,
    ElementBuilder,
};
pub use silkenweb_html::elements;
pub use silkenweb_reactive::{accumulators, clone, memo, signal};
