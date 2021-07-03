use cursive::traits::*;
use cursive::view::IntoBoxedView;
use cursive::views::ScrollView;
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
            .title("訊息"),
    )
}

pub fn get_controller_frame<V: IntoBoxedView>(view: V) -> BoxedView {
    let dialog = Dialog::around(view)
        .title("調整")
        .full_height()
        .scrollable();

    BoxedView::boxed(dialog)
}
