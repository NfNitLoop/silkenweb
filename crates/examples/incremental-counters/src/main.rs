use std::iter;

use surfinia_core::{
    hooks::{list_state::ElementList, state::Signal},
    mount,
    Builder,
};
use surfinia_html::{button, div, element_list, Div};

fn counter() -> Div {
    let count = Signal::new(0);
    let inc = count.writer();
    let dec = count.writer();

    div()
        .child(button().on_click(move || dec.replace(|i| i - 1)).text("-"))
        .text(count.map(|i| format!("{}", i)))
        .child(button().on_click(move || inc.replace(|i| i + 1)).text("+"))
        .build()
}

fn main() {
    console_error_panic_hook::set_once();
    let list = Signal::new(element_list(div(), move |()| counter(), iter::empty()));
    let push_elem = list.writer();
    let pop_elem = list.writer();

    mount(
        "app",
        div()
            .child(
                button()
                    .on_click(move || pop_elem.mutate(ElementList::pop))
                    .text("-"),
            )
            .child(
                button()
                    .on_click(move || push_elem.mutate(|l| l.push(&())))
                    .text("+"),
            )
            .child(list),
    );
}
