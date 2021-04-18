use std::iter;

use surfinia_core::{
    clone,
    hooks::state::{ReadSignal, Signal, WriteSignal},
    mount,
    Builder,
};
use surfinia_html::{button, div, element_list, h1, header, input, label, li, section, ul, Li};
use web_sys::HtmlInputElement;

struct TodoItem {
    text: Signal<String>,
    completed: Signal<bool>,
    editing: Signal<bool>,
}

impl TodoItem {
    fn new(text: impl Into<String>, completed: bool) -> Self {
        Self {
            text: Signal::new(text.into()),
            completed: Signal::new(completed),
            editing: Signal::new(false),
        }
    }

    fn save_edits(
        input: &HtmlInputElement,
        set_text: &WriteSignal<String>,
        set_editing: &WriteSignal<bool>,
    ) {
        let text = input.value();
        let text = text.trim();
        set_text.set(text.to_string());
        set_editing.set(false);
    }

    fn class(completed: bool, editing: bool) -> String {
        let mut classes = Vec::new();

        if completed {
            classes.push("completed");
        }

        if editing {
            classes.push("editing");
        }

        classes.join(" ")
    }

    fn render(&self) -> ReadSignal<Li> {
        self.editing.read().map({
            let set_editing = self.editing.write();
            let set_completed = self.completed.write();
            let get_completed = self.completed.read();
            let get_text = self.text.read();
            let set_text = self.text.write();

            move |&editing| {
                {
                    let item = li().class(
                        get_completed.map(move |&completed| Self::class(completed, editing)),
                    );

                    if editing {
                        // TODO: on_blur and on_keyup(Enter) to finish editing

                        // TODO: Set focus once this is rendered.
                        item.child(
                            input()
                                .class("edit")
                                .type_("text")
                                .value(&get_text)
                                .on_blur({
                                    // TODO: This doesn't seem to work
                                    clone!(set_editing, set_text);
                                    move |_, input| {
                                        Self::save_edits(&input, &set_text, &set_editing)
                                    }
                                })
                                .on_keyup({
                                    clone!(set_editing, set_text);
                                    move |keyup, input| {
                                        let key = keyup.key();

                                        if key == "Escape" {
                                            set_editing.set(false);
                                        } else if key == "Enter" {
                                            Self::save_edits(&input, &set_text, &set_editing);
                                        }
                                    }
                                }),
                        )
                    } else {
                        let completed_checkbox = input()
                            .class("toggle")
                            .type_("checkbox")
                            .on_click({
                                clone!(set_completed);
                                move |_, _| set_completed.replace(|completed| !completed)
                            })
                            .checked(get_completed.map(|&completed| completed));

                        item.child(
                            div()
                                .class("view")
                                .child(completed_checkbox)
                                .child(label().text(&get_text).on_dblclick({
                                    clone!(set_editing);
                                    move |_, _| set_editing.set(true)
                                }))
                                .child(button().class("destroy")),
                        )
                    }
                }
                .build()
            }
        })
    }
}

fn main() {
    console_error_panic_hook::set_once();
    let list = Signal::new(element_list(
        ul().class("todo-list"),
        TodoItem::render,
        iter::empty(),
    ));
    let list_mut = list.write();

    mount(
        "app",
        section()
            .class("todoapp")
            .child(
                header().child(h1().text("todos")).child(
                    input()
                        .class("new-todo")
                        .placeholder("What needs to be done?")
                        .autofocus(true)
                        .on_keyup(move |keyup, input| {
                            if keyup.key() == "Enter" {
                                list_mut.mutate(move |ts| {
                                    let text = input.value();
                                    let text = text.trim();

                                    if !text.is_empty() {
                                        ts.push(&TodoItem::new(text, false));
                                        input.set_value("");
                                    }
                                })
                            }
                        }),
                ),
            )
            .child(section().class("main").child(list.read())),
    );
}
