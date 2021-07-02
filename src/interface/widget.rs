use cursive::traits::*;
use cursive::views::{
    BoxedView, Button, Checkbox, Dialog, DummyView, EditView, LinearLayout, ListView, NamedView,
    ResizedView, SelectView, TextView, ViewRef,
};
use cursive::Cursive;

pub fn use_show_message(siv: &mut Cursive, message: &str) {
    siv.add_layer(
        Dialog::text(message)
            .button("關閉", |s| {
                s.pop_layer();
            })
            .title("message"),
    )
}
