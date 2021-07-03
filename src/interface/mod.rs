mod music;
mod widget;

use cursive::traits::*;
use cursive::views::{BoxedView, Dialog, LinearLayout, NamedView, ResizedView, SelectView};
use cursive::Cursive;

use std::cmp::PartialEq;
use std::fmt;

/// 頁面列表。
#[derive(PartialEq)]
enum Pages {
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
    // /// 產生器，隨機的標題。
    // RandomTitle,
    // /// 產生器，隨機的想法。
    // RandomIdea,
}

impl fmt::Display for Pages {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pages::Empty => "什麼也沒有",
                Pages::Start => "程式入口點",
                Pages::Quit => "離開此程式",
                Pages::RandomNotes => "隨機的音符",
                Pages::RandomChords => "隨機的旋律",
                Pages::RandomKeys => "隨機的大小調/調號",
                Pages::RandomModes => "隨機的調式",
                Pages::RandomMeter => "隨機的拍號",
                Pages::RandomTempo => "隨機的速度",
                // Pages::RandomTitle => "隨機的標題",
                // Pages::RandomIdea => "隨機的想法",
            }
        )
    }
}

/// 應用程式介面。
pub struct AppInterface {
    // now_page: Pages,
// history: Vec<&'static str>,
}

impl AppInterface {
    /// 介面初始化。
    pub fn new() -> AppInterface {
        AppInterface {
            // now_page: Pages::Empty,
            // history: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        cursive::logger::init();
        // log::error!("Something serious probably happened!");
        // log::warn!("Or did it?");
        // log::debug!("Logger initialized.");
        // log::info!("Starting!");

        let layout: ResizedView<NamedView<LinearLayout>> = LinearLayout::horizontal()
            .child(AppInterface::get_left_menu())
            .child(AppInterface::get_gerenator_view())
            .with_name("main_layout")
            .full_screen();

        let mut siv = cursive::default();

        siv.add_layer(BoxedView::boxed(layout));

        siv.add_global_callback('q', AppInterface::use_want_quit);
        siv.add_global_callback('~', cursive::Cursive::toggle_debug_console);
        // siv.add_global_callback('l', |_| log::trace!("Wooo"));

        siv.run();
    }

    /// 取得左側選單介面。
    fn get_left_menu() -> Dialog {
        let left_menu = SelectView::<Pages>::new()
            .on_submit(AppInterface::use_change_main_view)
            .item("首頁", Pages::Start)
            .item("離開", Pages::Quit)
            .item("--------------------", Pages::Empty)
            .item("隨機的音符", Pages::RandomNotes)
            .item("隨機的弦律", Pages::RandomChords)
            .item("隨機的大小調/調號", Pages::RandomKeys)
            .item("隨機的調式", Pages::RandomModes)
            .item("隨機的拍號", Pages::RandomMeter)
            .item("隨機的速度", Pages::RandomTempo);
        // .item("隨機的標題", Pages::RandomTitle)
        // .item("隨機的想法", Pages::RandomIdea);

        Dialog::around(
            left_menu
                .fixed_size((20, 11))
                .with_name("left_menu")
                .scrollable(),
        )
        .title("目錄")
    }

    /// 依據頁面參數改變主介面的內容。
    fn use_change_main_view(siv: &mut Cursive, page: &Pages) {
        let use_page = AppInterface::use_set_gerenator_view;

        match AppInterface::get_view_by_page(page) {
            None if *page == Pages::Quit => AppInterface::use_want_quit(siv),
            Some(boxed) => use_page(siv, boxed),
            _ => (),
        };
    }

    fn get_view_by_page(page: &Pages) -> Option<BoxedView> {
        match page {
            Pages::Empty => None,
            Pages::Quit => None,
            Pages::Start => Some(AppInterface::get_start_page()),
            Pages::RandomNotes => Some(music::random_notes::main()),
            Pages::RandomChords => Some(music::random_chords::main()),
            Pages::RandomKeys => Some(music::random_keys::main()),
            Pages::RandomModes => Some(music::random_modes::main()),
            Pages::RandomMeter => Some(music::random_meter::main()),
            Pages::RandomTempo => Some(music::random_tempo::main()),
            // Pages::RandomTitle => Some(AppInterface::get_random_title_generator()),
            // Pages::RandomIdea => Some(AppInterface::get_random_idea_generator()),
        }
    }

    fn use_set_gerenator_view(siv: &mut Cursive, view: BoxedView) {
        let mut generator_view = siv.find_name::<LinearLayout>("main_layout").unwrap();

        generator_view.remove_child(1);
        generator_view.add_child(view);
    }

    fn get_gerenator_view() -> BoxedView {
        let content = AppInterface::get_start_page();

        BoxedView::boxed(content) //.with_name("generator_view")
    }

    fn use_want_quit(siv: &mut Cursive) {
        let message = Dialog::text("確定要退出程式嗎？")
            .button("確定", |s| s.quit())
            .button("取消", |s| {
                s.pop_layer();
            })
            .title("訊息");

        siv.add_layer(message);
    }

    fn get_start_page() -> BoxedView {
        let content = Dialog::text("welcome to SomeIdea!").full_screen();

        BoxedView::boxed(content)
    }
}
