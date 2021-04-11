use surfinia::{button, div, mount, use_state, Builder, DivBuilder, GetState, MemoScope, SetState};

fn counter(count: &GetState<u32>, set_count: &SetState<u32>) -> DivBuilder {
    let inc = set_count.clone();
    let dec = set_count.clone();

    div()
        .child(button().on_click(move || inc.map(|i| i + 1)).text("+"))
        .child(button().on_click(move || dec.map(|i| i - 1)).text("-"))
        .child(count.with(|i| div().text(format!("Count = {}", i))))
}

fn main() {
    console_error_panic_hook::set_once();
    let mut child_counts = MemoScope::new();

    mount(
        "app",
        child_counts.with(|child_counts| {
            let (count, set_count) = use_state(0);

            counter(&count, &set_count).child(count.with(move |&count| {
                let mut counters = div();

                for i in 0..count {
                    let (count, set_count) = use_state(0);

                    let child = child_counts.cache(i, || counter(&count, &set_count).build());

                    counters = counters.child(child);
                }

                counters
            }))
        }),
    );
}
