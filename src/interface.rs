use cursive::traits::*;
use cursive::views::{
    BoxedView, Button, Checkbox, Dialog, DummyView, EditView, LinearLayout, ListView, NamedView,
    ResizedView, SelectView, TextView, ViewRef,
};
use cursive::Cursive;

use std::cmp::PartialEq;
use std::fmt;

use crate::generator::{self, MusicChordTypes, MusicNotes};

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
            Pages::RandomNotes => Some(AppInterface::get_random_notes_generator()),
            Pages::RandomChords => Some(AppInterface::get_random_chords_generator()),
            Pages::RandomKeys => Some(AppInterface::get_random_keys_generator()),
            Pages::RandomModes => Some(AppInterface::get_random_modes_generator()),
            Pages::RandomMeter => Some(AppInterface::get_random_meter_generator()),
            Pages::RandomTempo => Some(AppInterface::get_random_tempo_generator()),
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
        let message = Dialog::text("did you want to quit this app?")
            .button("Yes", |s| s.quit())
            .button("No", |s| {
                s.pop_layer();
            });

        siv.add_layer(message);
    }

    fn use_show_tips(siv: &mut Cursive, message: &str) {
        siv.add_layer(Dialog::text(message).button("關閉", |s| {
            s.pop_layer();
        }))
    }

    fn get_start_page() -> BoxedView {
        let content = Dialog::text("welcome to SomeIdea!").full_screen();

        BoxedView::boxed(content)
    }

    fn get_random_notes_generator() -> BoxedView {
        let control_box = Dialog::around(
            LinearLayout::vertical()
                .child(Button::new("random", |s| {
                    let number_input_box: ViewRef<EditView> =
                        s.find_name("random_notes_number").unwrap();
                    let input_number = match number_input_box.get_content().parse::<usize>() {
                        Ok(num) => {
                            if num == 0 {
                                AppInterface::use_show_tips(s,"數量欄位請填入大於零的整數。");
                                0
                            } else {
                                num
                            }
                        },
                        Err(error_message) => {
                            AppInterface::use_show_tips(s,&format!(
                                    "解析輸入的數量值時發生錯誤，請確認輸入的內容僅包含不為零的數字\n\
                                     ，此程式並不支援空格或其他特殊符號。\n\n錯誤細節：\n\n{}",
                                    error_message
                                ));
                            0
                        }
                    };

                    if input_number == 0 {
                        return;
                    }

                    let default_list = MusicNotes::default();
                    let choose_notes: Vec<MusicNotes> = (0..12)
                        .filter_map(|index| {
                            let is_need = s
                                .find_name::<Checkbox>(&format!("note_check_{}", index))
                                .unwrap()
                                .is_checked();

                            if is_need {
                                Some(default_list[index])
                            } else {
                                None
                            }
                        })
                        .collect();

                    if choose_notes.len() == 0 {
                        AppInterface::use_show_tips(s,"請選擇至少一個音符。" );

                        return;
                    }

                    let mut output_box: ViewRef<Dialog> =
                        s.find_name("random_notes_output").unwrap();
                    let new_random_data = generator::get_random_notes(input_number,choose_notes);
                    output_box
                        .set_content(TextView::new(format!("You notes is: {}", new_random_data)));
                }))
                .child(DummyView)
                .child(
                    ListView::new().child(
                        "數量",
                        EditView::new()
                            .content("8")
                            .with_name("random_notes_number"),
                    ),
                )
                .child(DummyView)
                .child(
                    LinearLayout::horizontal()
                        .child(
                            ListView::new()
                                .child("C ", Checkbox::new().checked().with_name("note_check_0"))
                                .child("Db", Checkbox::new().checked().with_name("note_check_1"))
                                .child("D ", Checkbox::new().checked().with_name("note_check_2"))
                                .child("Eb", Checkbox::new().checked().with_name("note_check_3"))
                                .child("E ", Checkbox::new().checked().with_name("note_check_4"))
                                .child("F ", Checkbox::new().checked().with_name("note_check_5"))
                        )
                        .child(DummyView)
                        .child(DummyView)
                        .child(
                            ListView::new()
                                .child("F#", Checkbox::new().checked().with_name("note_check_6"))
                                .child("G ", Checkbox::new().checked().with_name("note_check_7"))
                                .child("Ab", Checkbox::new().checked().with_name("note_check_8"))
                                .child("A ", Checkbox::new().checked().with_name("note_check_9"))
                                .child("Bb", Checkbox::new().checked().with_name("note_check_10"))
                                .child("B ", Checkbox::new().checked().with_name("note_check_11"))
                        ),
                ),
        )
        .title("choose you want")
        .full_height()
        .scrollable();

        let default_result = generator::get_random_notes(8, MusicNotes::default());
        let layout = LinearLayout::horizontal()
            .child(control_box)
            .child(
                Dialog::text(format!("You notes is: {}", default_result))
                    .with_name("random_notes_output")
                    .full_width(),
            )
            .full_screen();

        BoxedView::boxed(layout)
    }

    fn get_random_chords_generator() -> BoxedView {
        let info = Dialog::text("this is what you got!");
        let result = generator::get_random_chords(MusicChordTypes::default(), 8);
        let layout = LinearLayout::vertical()
            .child(info)
            .child(Dialog::text(format!("You chords is: {}", result)).full_height())
            .full_screen();

        BoxedView::boxed(layout)
    }

    fn get_random_keys_generator() -> BoxedView {
        let info = Dialog::text("this is what you got!");
        let result = generator::get_random_keys();
        let layout = LinearLayout::vertical()
            .child(info)
            .child(Dialog::text(format!("You key is: {}", result)).full_height())
            .full_screen();

        BoxedView::boxed(layout)
    }

    fn get_random_modes_generator() -> BoxedView {
        let info = Dialog::text("this is what you got!");
        let result = generator::get_random_modes();
        let layout = LinearLayout::vertical()
            .child(info)
            .child(Dialog::text(format!("You modes is: {}", result)).full_height())
            .full_screen();

        BoxedView::boxed(layout)
    }

    fn get_random_meter_generator() -> BoxedView {
        let info = Dialog::text("this is what you got!");
        let result = generator::get_random_meter();
        let layout = LinearLayout::vertical()
            .child(info)
            .child(Dialog::text(format!("You meter is: {}", result)).full_height())
            .full_screen();

        BoxedView::boxed(layout)
    }

    fn get_random_tempo_generator() -> BoxedView {
        let info = Dialog::text("this is what you got!");
        let result = generator::get_random_tempo();
        let layout = LinearLayout::vertical()
            .child(info)
            .child(Dialog::text(format!("You tempo is: {}", result)).full_height())
            .full_screen();

        BoxedView::boxed(layout)
    }

    // fn get_random_title_generator() -> BoxedView {
    //     let content = Dialog::text("welcome to SomeIdea use_random_title_generator!").full_screen();

    //     BoxedView::boxed(content)
    // }

    // fn get_random_idea_generator() -> BoxedView {
    //     let content = Dialog::text("welcome to SomeIdea use_random_idea_generator!").full_screen();

    //     BoxedView::boxed(content)
    // }
}
