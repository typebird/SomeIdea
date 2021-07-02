use cursive::traits::*;
use cursive::views::{
    BoxedView, Button, Checkbox, Dialog, DummyView, EditView, LinearLayout, ListView, TextView,
};
use cursive::Cursive;

use super::widget;
use crate::generator::{self, MusicChordTypes};

/// 取得並檢查數量輸入值。
///
/// 在輸入內容不符合規則時顯示提示。
fn get_chords_number_input(siv: &mut Cursive) -> usize {
    let input = siv.find_name::<EditView>("random_chords_number").unwrap();

    match input.get_content().parse::<usize>() {
        Ok(num) => {
            if num == 0 {
                widget::use_show_message(siv, "數量欄位請填入大於零的整數。");
            }
            num
        }
        Err(_) => {
            widget::use_show_message(siv, "解析輸入的數量值時發生錯誤，請確認輸入的內容僅包含數字\n，此程式並不支援空格或其他特殊符號。");
            0
        }
    }
}

/// 取得控制介面勾選框的狀態。
fn get_all_chords_status(siv: &mut Cursive) -> Vec<MusicChordTypes> {
    let default_vec = MusicChordTypes::default();

    (0..default_vec.len())
        .filter_map(|index| {
            let is_need = siv
                .find_name::<Checkbox>(&format!("random_chords_check_{}", index))
                .unwrap()
                .is_checked();

            match is_need {
                true => Some(default_vec[index]),
                false => None,
            }
        })
        .collect()
}

/// 以對應名稱取得功能區個欄位的值，並據以更改顯示內容。
///
/// 如果檢查到任何不符合規則的輸入參數，便提前停止。
fn use_change_display_by_controler(siv: &mut Cursive) {
    let user_number = get_chords_number_input(siv);

    if user_number == 0 {
        return;
    }

    let user_choose = get_all_chords_status(siv);

    if user_choose.len() == 0 {
        widget::use_show_message(siv, "請選擇至少一個類型。");

        return;
    }

    let result = generator::get_random_chords(user_choose, user_number);

    siv.find_name::<Dialog>("random_chords_display")
        .unwrap()
        .set_content(TextView::new(result));
}

/// 側邊控制欄。
fn side_control_area() -> BoxedView {
    let chords_check_list = MusicChordTypes::default().iter().enumerate().fold(
        ListView::new(),
        |list, (index, chord)| {
            list.child(
                &chord.to_string(),
                Checkbox::new()
                    .checked()
                    .with_name(format!("random_chords_check_{}", index)),
            )
        },
    );

    let control = Dialog::around(
        LinearLayout::vertical()
            .child(Button::new("random", use_change_display_by_controler))
            .child(DummyView)
            .child(
                ListView::new().child(
                    "數量",
                    EditView::new()
                        .content("8")
                        .with_name("random_chords_number"),
                ),
            )
            .child(DummyView)
            .child(chords_check_list),
    )
    .title("control")
    .scrollable()
    .with_name("random_chords_control");

    BoxedView::boxed(control)
}

pub fn main() -> BoxedView {
    let result = generator::get_random_chords(MusicChordTypes::default(), 8);
    let control = side_control_area();
    let display = Dialog::text(result)
        .title("result")
        .with_name("random_chords_display")
        .full_width();

    let layout = LinearLayout::horizontal()
        .child(control)
        .child(display)
        .full_screen();

    BoxedView::boxed(layout)
}
