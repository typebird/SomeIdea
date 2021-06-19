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
    /// 產生器，隨機的標題。
    RandomTitle,
    /// 產生器，隨機的想法。
    RandomIdea,
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
                Pages::RandomTitle => "隨機的標題",
                Pages::RandomIdea => "隨機的想法",
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
    pub fn new() {
        let mut siv = cursive::default();

        let layout: ResizedView<NamedView<LinearLayout>> = LinearLayout::horizontal()
            .child(AppInterface::get_left_menu())
            .child(AppInterface::get_gerenator_view())
            .with_name("main_layout")
            .full_screen();

        siv.add_layer(BoxedView::boxed(layout));

        siv.add_global_callback('q', AppInterface::use_want_quit);

        siv.run();
    }

    /// 取得左側選單介面。
    fn get_left_menu() -> Dialog {
        let left_menu = SelectView::<Pages>::new()
            .on_submit(AppInterface::use_submit_on_left_menu)
            .item("首頁", Pages::Start)
            .item("離開", Pages::Quit)
            .item("--------------------", Pages::Empty)
            .item("隨機的音符", Pages::RandomNotes)
            .item("隨機的弦律", Pages::RandomChords)
            .item("隨機的大小調/調號", Pages::RandomKeys)
            .item("隨機的調式", Pages::RandomModes)
            .item("隨機的拍號", Pages::RandomMeter)
            .item("隨機的速度", Pages::RandomTempo)
            .item("隨機的標題", Pages::RandomTitle)
            .item("隨機的想法", Pages::RandomIdea);

        Dialog::around(
            left_menu
                // .scrollable()
                .fixed_size((20, 11))
                .with_name("left_menu"),
        )
        .title("目錄")
    }

    /// 當左側選單發送點擊事件時，依據所選項目改變右側介面的內容。
    fn use_submit_on_left_menu(siv: &mut Cursive, page: &Pages) {
        let use_page = AppInterface::use_set_gerenator_view;

        match page {
            Pages::Empty => (),
            Pages::Quit => AppInterface::use_want_quit(siv),
            Pages::Start => use_page(siv, AppInterface::use_start_page()),
            Pages::RandomNotes => use_page(siv, AppInterface::use_random_notes_generator()),
            Pages::RandomChords => use_page(siv, AppInterface::use_random_chords_generator()),
            Pages::RandomKeys => use_page(siv, AppInterface::use_random_keys_generator()),
            Pages::RandomModes => use_page(siv, AppInterface::use_random_modes_generator()),
            Pages::RandomMeter => use_page(siv, AppInterface::use_random_meter_generator()),
            Pages::RandomTempo => use_page(siv, AppInterface::use_random_tempo_generator()),
            Pages::RandomTitle => use_page(siv, AppInterface::use_random_title_generator()),
            Pages::RandomIdea => use_page(siv, AppInterface::use_random_idea_generator()),
        };
    }

    fn use_set_gerenator_view(siv: &mut Cursive, view: BoxedView) {
        let mut generator_view = siv.find_name::<LinearLayout>("main_layout").unwrap();

        generator_view.remove_child(1);
        generator_view.add_child(view);
    }

    fn get_gerenator_view() -> NamedView<BoxedView> {
        let content = AppInterface::use_start_page();

        BoxedView::boxed(content).with_name("generator_view")
    }

    fn use_want_quit(siv: &mut Cursive) {
        let message = Dialog::text("did you want to quit this app?")
            .button("Yes", |s| s.quit())
            .button("No", |s| {
                s.pop_layer();
            });

        siv.add_layer(message);
    }

    fn use_start_page() -> BoxedView {
        let content = Dialog::text("welcome to SomeIdea use_start_page!").full_screen();

        BoxedView::boxed(content)
    }

    fn use_random_notes_generator() -> BoxedView {
        let content = Dialog::text("welcome to SomeIdea use_random_notes_generator!").full_screen();

        BoxedView::boxed(content)
    }

    fn use_random_chords_generator() -> BoxedView {
        let content =
            Dialog::text("welcome to SomeIdea use_random_chords_generator!").full_screen();

        BoxedView::boxed(content)
    }

    fn use_random_keys_generator() -> BoxedView {
        let content = Dialog::text("welcome to SomeIdea use_random_keys_generator!").full_screen();

        BoxedView::boxed(content)
    }

    fn use_random_modes_generator() -> BoxedView {
        let content = Dialog::text("welcome to SomeIdea use_random_modes_generator!").full_screen();

        BoxedView::boxed(content)
    }

    fn use_random_meter_generator() -> BoxedView {
        let content = Dialog::text("welcome to SomeIdea use_random_meter_generator!").full_screen();

        BoxedView::boxed(content)
    }

    fn use_random_tempo_generator() -> BoxedView {
        let content = Dialog::text("welcome to SomeIdea use_random_tempo_generator!").full_screen();

        BoxedView::boxed(content)
    }

    fn use_random_title_generator() -> BoxedView {
        let content = Dialog::text("welcome to SomeIdea use_random_title_generator!").full_screen();

        BoxedView::boxed(content)
    }

    fn use_random_idea_generator() -> BoxedView {
        let content = Dialog::text("welcome to SomeIdea use_random_idea_generator!").full_screen();

        BoxedView::boxed(content)
    }

    // fn start_message(_siv: &mut cursive::Cursive) {}

    // pub fn run() {}
}
