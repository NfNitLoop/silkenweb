use futures_signals::{
    signal::Mutable,
    signal_vec::{MutableVec, MutableVecLockMut, SignalVecExt},
};
use silkenweb::prelude::ParentBuilder;
use silkenweb_dom::render::render_now;
use silkenweb_elements::{
    html::{div, p},
    macros::ElementBuilder,
    HtmlElement,
};

macro_rules! isomorphic_test {
    (async fn $name:ident() $body:block) => {
        #[cfg(not(target_arch = "wasm32"))]
        #[test]
        fn $name() {
            ::silkenweb_dom::render::server::block_on(async { $body });
        }

        #[cfg(target_arch = "wasm32")]
        #[wasm_bindgen_test::wasm_bindgen_test]
        async fn $name() {
            $body
        }
    };
}

macro_rules! render_test {
    ($name:ident, $node:expr, $expected:expr) => {
        isomorphic_test! {
            async fn $name() {
                assert_eq!($node.build().to_string(), $expected)
            }
        }
    };
}

render_test!(empty_element, div(), "<div></div>");
render_test!(
    single_attribute,
    div().id("my-id"),
    r#"<div id="my-id"></div>"#
);
render_test!(
    multi_attribute,
    div().id("my-id").class(["my-class"]),
    r#"<div id="my-id" class="my-class"></div>"#
);
render_test!(
    boolean_false_attribute,
    div().hidden(false),
    r#"<div></div>"#
);
render_test!(
    boolean_true_attribute,
    div().hidden(true),
    r#"<div hidden=""></div>"#
);

render_test!(
    child,
    div().child(p().text("Hello!")),
    "<div><p>Hello!</p></div>"
);
render_test!(
    children,
    div().children([p().text("Hello"), p().text("World!")]),
    "<div><p>Hello</p><p>World!</p></div>"
);

macro_rules! children_signal_test {
    ($name:ident, $initial:expr, $operations:expr, $expected:expr) => {
        isomorphic_test! {
            async fn $name() {
                children_signal_test($initial, $operations, $expected).await;
            }
        }
    };
}

children_signal_test!(empty_children_signal, [], |_| (), []);
children_signal_test!(append_child, [], |mut children| children.push(0), [0]);
children_signal_test!(
    children_pop_to_empty,
    [0],
    |mut children| {
        children.pop();
    },
    []
);

children_signal_test!(
    children_pop,
    [0, 1],
    |mut children| {
        children.pop();
    },
    [0]
);

children_signal_test!(
    children_insert_from_empty,
    [],
    |mut children| {
        children.insert(0, 0);
    },
    [0]
);

children_signal_test!(
    children_insert_first,
    [1],
    |mut children| {
        children.insert(0, 0);
    },
    [0, 1]
);

children_signal_test!(
    children_insert_middle,
    [0, 2],
    |mut children| {
        children.insert(1, 1);
    },
    [0, 1, 2]
);

children_signal_test!(
    children_insert_end,
    [0],
    |mut children| {
        children.insert(1, 1);
    },
    [0, 1]
);

children_signal_test!(
    children_replace_empty,
    [],
    |mut children| {
        children.replace(vec![0, 1]);
    },
    [0, 1]
);

children_signal_test!(
    children_replace_with_empty,
    [0, 1],
    |mut children| {
        children.replace(vec![]);
    },
    []
);

children_signal_test!(
    children_replace,
    [0, 1],
    |mut children| {
        children.replace(vec![2, 3]);
    },
    [2, 3]
);

children_signal_test!(
    children_clear,
    [0, 1],
    |mut children| {
        children.clear();
    },
    []
);

children_signal_test!(
    children_set,
    [0, 1],
    |mut children| {
        children.set(0, 1);
    },
    [1, 1]
);

children_signal_test!(
    children_remove_at,
    [0, 1, 2],
    |mut children| {
        children.remove(1);
    },
    [0, 2]
);

children_signal_test!(
    children_move_forward,
    [0, 1, 2, 3, 4],
    |mut children| {
        children.move_from_to(1, 2);
    },
    [0, 2, 1, 3, 4]
);

children_signal_test!(
    children_move_backward,
    [0, 1, 2, 3, 4],
    |mut children| {
        children.move_from_to(2, 1);
    },
    [0, 2, 1, 3, 4]
);

children_signal_test!(
    children_move_to_end,
    [0, 1, 2],
    |mut children| {
        children.move_from_to(1, 2);
    },
    [0, 2, 1]
);

children_signal_test!(
    children_move_from_end,
    [0, 1, 2],
    |mut children| {
        children.move_from_to(2, 1);
    },
    [0, 2, 1]
);

isomorphic_test! {
    async fn text_signal() {
        let text = Mutable::new("Initial text");
        let elem = p().text_signal(text.signal()).build();
        render_now().await;
        assert_eq!(elem.to_string(), "<p>Initial text</p>");
        text.set("Updated text");
        render_now().await;
        assert_eq!(elem.to_string(), "<p>Updated text</p>");
    }
}

isomorphic_test! {
    async fn attribute_signal() {
        let text = Mutable::new("Initial text");
        let elem = div().title_signal(text.signal()).build();
        render_now().await;
        assert_eq!(elem.to_string(), r#"<div title="Initial text"></div>"#);
        text.set("Updated text");
        render_now().await;
        assert_eq!(elem.to_string(), r#"<div title="Updated text"></div>"#);
    }
}

isomorphic_test! {
    async fn optional_attribute_signal() {
        let text = Mutable::new(Some("Initial text"));
        let elem = div().title_signal(text.signal()).build();
        render_now().await;
        assert_eq!(elem.to_string(), r#"<div title="Initial text"></div>"#);
        text.set(None);
        render_now().await;
        assert_eq!(elem.to_string(), r#"<div></div>"#);
    }
}

pub async fn children_signal_test<const INITIAL_COUNT: usize, const EXPECTED_COUNT: usize>(
    initial: [usize; INITIAL_COUNT],
    f: impl FnOnce(MutableVecLockMut<usize>),
    expected: [usize; EXPECTED_COUNT],
) {
    let children = MutableVec::<usize>::new_with_values(initial.to_vec());
    let element = div().children_signal(children.signal_vec().map(|i| p().text(&format!("{}", i))));

    f(children.lock_mut());
    let mut expected_html = String::new();

    for i in expected {
        expected_html.push_str(&format!("<p>{}</p>", i));
    }

    render_now().await;
    assert_eq!(element.to_string(), format!("<div>{}</div>", expected_html));
}
