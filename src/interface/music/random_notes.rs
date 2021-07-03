use cursive::traits::*;
use cursive::views::{
    BoxedView, Button, Checkbox, Dialog, DummyView, EditView, LinearLayout, ListView, TextView,
    ViewRef,
};
use cursive::Cursive;

use super::widget;
use crate::generator::{self, MusicNotes};

/// 取得並檢查數量輸入值。
///
/// 在輸入內容不符合規則時顯示提示。
fn use_get_input_number(siv: &mut Cursive) -> usize {
    let input_box: ViewRef<EditView> = siv.find_name("random_notes_number").unwrap();

    match input_box.get_content().parse::<usize>() {
        Ok(num) if num == 0 => {
            widget::use_show_message(siv, "數量欄位請填入大於零的整數。");
            num
        }
        Ok(num) => num,
        Err(_) => {
            widget::use_show_message(siv, "解析輸入的數量值時發生錯誤，請確認輸入的內容僅包含數字\n，此程式並不支援空格或其他特殊符號。");
            0
        }
    }
}

/// 以目前音符勾選框的狀態產生對應向量列表。
fn get_all_check_status(siv: &mut Cursive) -> Vec<MusicNotes> {
    let default_list = MusicNotes::default();

    (0..default_list.len())
        .filter_map(|index| {
            let is_need = siv
                .find_name::<Checkbox>(&format!("random_note_check_{}", index))
                .unwrap()
                .is_checked();

            match is_need {
                true => Some(default_list[index]),
                false => None,
            }
        })
        .collect::<Vec<MusicNotes>>()
}

/// 以對應名稱取得功能區個欄位的值，並據以更改顯示內容。
///
/// 如果檢查到任何不符合規則的輸入參數，便提前停止。
fn use_change_display(siv: &mut Cursive) {
    let input_number = use_get_input_number(siv);

    if input_number == 0 {
        return;
    }

    let choose_notes = get_all_check_status(siv);

    if choose_notes.len() == 0 {
        widget::use_show_message(siv, "請選擇至少一個音符。");

        return;
    }

    let new_random_data = generator::get_random_notes(choose_notes, input_number);
    siv.find_name::<Dialog>("random_notes_display")
        .unwrap()
        .set_content(TextView::new(new_random_data));
}

/// 側邊功能區。
fn get_controller_view() -> BoxedView {
    let notes_check_list = LinearLayout::horizontal()
        .child(
            ListView::new()
                .child(
                    "C ",
                    Checkbox::new().checked().with_name("random_note_check_0"),
                )
                .child(
                    "Db",
                    Checkbox::new().checked().with_name("random_note_check_1"),
                )
                .child(
                    "D ",
                    Checkbox::new().checked().with_name("random_note_check_2"),
                )
                .child(
                    "Eb",
                    Checkbox::new().checked().with_name("random_note_check_3"),
                )
                .child(
                    "E ",
                    Checkbox::new().checked().with_name("random_note_check_4"),
                )
                .child(
                    "F ",
                    Checkbox::new().checked().with_name("random_note_check_5"),
                ),
        )
        .child(DummyView)
        .child(DummyView)
        .child(
            ListView::new()
                .child(
                    "F#",
                    Checkbox::new().checked().with_name("random_note_check_6"),
                )
                .child(
                    "G ",
                    Checkbox::new().checked().with_name("random_note_check_7"),
                )
                .child(
                    "Ab",
                    Checkbox::new().checked().with_name("random_note_check_8"),
                )
                .child(
                    "A ",
                    Checkbox::new().checked().with_name("random_note_check_9"),
                )
                .child(
                    "Bb",
                    Checkbox::new().checked().with_name("random_note_check_10"),
                )
                .child(
                    "B ",
                    Checkbox::new().checked().with_name("random_note_check_11"),
                ),
        );

    widget::get_controller_frame(
        LinearLayout::vertical()
            .child(Button::new("更新", use_change_display))
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
            .child(notes_check_list),
    )
}

pub fn main() -> BoxedView {
    let layout = LinearLayout::horizontal()
        .child(get_controller_view())
        .child(
            Dialog::text(generator::get_random_notes_by_default())
                .title("隨機的音符")
                .with_name("random_notes_display")
                .full_width(),
        )
        .full_screen();

    BoxedView::boxed(layout)
}
