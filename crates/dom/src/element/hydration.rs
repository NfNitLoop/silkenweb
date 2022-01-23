// TODO: Extract some traits to make it more obvious that Lazy* and Strict* are compatible.

pub use super::lazy::{
    LazyElement as HydrationElement, LazyNode as HydrationNode, LazyNodeBase as HydrationNodeBase,
    LazyNodeRef as HydrationNodeRef, LazyText as HydrationText,
};
// pub use super::strict::{
//     StrictElement as HydrationElement, StrictNode as HydrationNode,
//     StrictNodeBase as HydrationNodeBase, StrictNodeRef as HydrationNodeRef,
//     StrictText as HydrationText,
// };
