use cursive::traits::*;
use cursive::view::IntoBoxedView;
use cursive::views::{BoxedView, Button, Dialog, DummyView, LinearLayout, TextView};
use cursive::Cursive;

use super::music;

pub fn use_show_message(siv: &mut Cursive, message: &str) {
    siv.add_layer(
        Dialog::text(message)
            .button("關閉", |s| {
                s.pop_layer();
            })
            .title("訊息"),
    )
}

pub fn use_want_quit(siv: &mut Cursive) {
    let message = Dialog::text("確定要退出程式嗎？")
        .button("確定", |s| s.quit())
        .button("取消", |s| {
            s.pop_layer();
        })
        .title("訊息");

    siv.add_layer(message);
}

pub fn get_controller_frame<V: IntoBoxedView>(view: V) -> BoxedView {
    let dialog = Dialog::around(view)
        .title("調整")
        .full_height()
        .scrollable();

    BoxedView::boxed(dialog)
}

pub fn get_start_page() -> BoxedView {
    let title = TextView::new(
        "                                  welcome to\n\
                 ███████╗  ██████╗  ███╗   ███╗ ███████╗   ██╗ ██████╗  ███████╗  █████╗    ██╗\n\
                 ██╔════╝ ██╔═══██╗ ████╗ ████║ ██╔════╝   ██║ ██╔══██╗ ██╔════╝ ██╔══██╗   ██║\n\
                 ███████╗ ██║   ██║ ██╔████╔██║ █████╗     ██║ ██║  ██║ █████╗   ███████║   ██║\n\
                 ╚════██║ ██║   ██║ ██║╚██╔╝██║ ██╔══╝     ██║ ██║  ██║ ██╔══╝   ██╔══██║   ╚═╝\n\
                 ███████║ ╚██████╔╝ ██║ ╚═╝ ██║ ███████╗   ██║ ██████╔╝ ███████╗ ██║  ██║   ██╗\n\
                 ╚══════╝  ╚═════╝  ╚═╝     ╚═╝ ╚══════╝   ╚═╝ ╚═════╝  ╚══════╝ ╚═╝  ╚═╝   ╚═╝",
    );
    let layout = LinearLayout::vertical()
        .child(title)
        .child(DummyView)
        .child(
            LinearLayout::horizontal()
                .child(Button::new("Music", |siv| {
                    siv.pop_layer();
                    siv.add_layer(music::main());
                }))
                .child(DummyView)
                .child(Button::new("Story", |_| ())),
        );
    let content = Dialog::around(layout).full_screen();

    BoxedView::boxed(content)
}
