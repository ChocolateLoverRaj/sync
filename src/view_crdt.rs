use iced::{Renderer, Theme};

use crate::*;

pub trait ViewCrdt: Crdt {
    fn view<'a>(state: &'a State<Self>) -> impl Into<Element<'a, Message<Self>, Theme, Renderer>>;
}
