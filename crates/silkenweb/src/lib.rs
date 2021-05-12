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
    after_render, animation, element_list, local_storage, mount, render_updates, session_storage,
    tag, tag_in_namespace, unmount, Builder, DomElement, Element, ElementBuilder,
};
pub use silkenweb_html::elements;
pub use silkenweb_reactive::{accumulators, clone, memo, signal};

pub mod router {
    //! URL based routing.
    //!
    //! Get the URL with [`url()`], and set it however you want to. For example:
    //! - with an anchor element like `<a href="/some/link">Some link</a>`
    //! - with [`set_url_path`].
    //!
    //! # Example
    //!
    //! ```no_run
    //! # use silkenweb::{
    //! #     elements::{button, div, p},
    //! #     mount, router,
    //! # };
    //! div()
    //!     .child(
    //!         button()
    //!             .on_click(|_, _| router::set_url_path("/route_1"))
    //!             .text("Go to route 1"),
    //!     )
    //!     .child(
    //!         button()
    //!             .on_click(|_, _| router::set_url_path("/route_2"))
    //!             .text("Go to route 2"),
    //!     )
    //!     .child(p().text(router::url().map(|url| format!("URL Path is: {}", url.pathname()))));
    //! ```
    pub use silkenweb_dom::router::{set_url_path, url};
}
