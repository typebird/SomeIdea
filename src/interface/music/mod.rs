pub mod random_chords;
pub mod random_modes;
pub mod random_notes;

use cursive::traits::*;
use cursive::views::{BoxedView, Dialog, LinearLayout, SelectView};
use cursive::Cursive;

use super::widget;

pub mod random_keys {
    use cursive::traits::*;
    use cursive::views::{BoxedView, Dialog};

    use crate::generator;

    pub fn main() -> BoxedView {
        let result = generator::get_random_keys();
        let layout = Dialog::text(result)
            .title("隨機的大小調/調號")
            .full_screen();

        BoxedView::boxed(layout)
    }
}

pub mod random_meter {
    use cursive::traits::*;
    use cursive::views::{BoxedView, Dialog};

    use crate::generator;

    pub fn main() -> BoxedView {
        let result = generator::get_random_meter();
        let layout = Dialog::text(result).title("隨機的拍號").full_screen();

        BoxedView::boxed(layout)
    }
}

pub mod random_tempo {
    use cursive::traits::*;
    use cursive::views::{BoxedView, Dialog};

    use crate::generator;

    pub fn main() -> BoxedView {
        let result = generator::get_random_tempo();
        let layout = Dialog::text(result.to_string())
            .title("隨機的速度")
            .full_screen();

        BoxedView::boxed(layout)
    }
}

/// 頁面列表。
#[derive(PartialEq)]
enum MusicPages {
    /// 空頁面，什麼也沒有。
    Empty,
    /// 起始頁，程式入口點。
    Start,
    /// 離開此程式。
    Quit,
    /// 產生器，隨機的音符。
    RandomNotes,
    /// 產生器，隨機的弦律。
    RandomChords,
    /// 產生器，隨機的大小調/調號。
    RandomKeys,
    /// 產生器，隨機的調式。
    RandomModes,
    /// 產生器，隨機的拍號。
    RandomMeter,
    /// 產生器，隨機的速度。
    RandomTempo,
}

fn get_view_by_page(page: &MusicPages) -> Option<BoxedView> {
    match page {
        MusicPages::Empty => None,
        MusicPages::Quit => None,
        MusicPages::Start => Some(widget::get_start_page()),
        MusicPages::RandomNotes => Some(random_notes::main()),
        MusicPages::RandomChords => Some(random_chords::main()),
        MusicPages::RandomKeys => Some(random_keys::main()),
        MusicPages::RandomModes => Some(random_modes::main()),
        MusicPages::RandomMeter => Some(random_meter::main()),
        MusicPages::RandomTempo => Some(random_tempo::main()),
    }
}

fn use_set_main_area(siv: &mut Cursive, view: BoxedView) {
    let mut generator_view = siv.find_name::<LinearLayout>("main_layout").unwrap();

    generator_view.remove_child(1);
    generator_view.add_child(view);
}

fn use_change_display(siv: &mut Cursive, page: &MusicPages) {
    match get_view_by_page(page) {
        None if *page == MusicPages::Quit => widget::use_want_quit(siv),
        Some(view) => use_set_main_area(siv, view),
        _ => (),
    }
}

fn get_left_menu() -> Dialog {
    let left_menu = SelectView::<MusicPages>::new()
        .on_submit(use_change_display)
        .item("首頁", MusicPages::Start)
        .item("離開", MusicPages::Quit)
        .item("--------------------", MusicPages::Empty)
        .item("隨機的音符", MusicPages::RandomNotes)
        .item("隨機的弦律", MusicPages::RandomChords)
        .item("隨機的大小調/調號", MusicPages::RandomKeys)
        .item("隨機的調式", MusicPages::RandomModes)
        .item("隨機的拍號", MusicPages::RandomMeter)
        .item("隨機的速度", MusicPages::RandomTempo);

    Dialog::around(
        left_menu
            .fixed_size((20, 11))
            .with_name("left_menu")
            .scrollable(),
    )
    .title("目錄")
}

pub fn main() -> BoxedView {
    let layout = LinearLayout::horizontal()
        .child(get_left_menu())
        .child(widget::get_start_page())
        .with_name("main_layout")
        .full_screen();

    BoxedView::boxed(layout)
}
