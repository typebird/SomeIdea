use cursive::views::{
    BoxedView, Button, Checkbox, Dialog, DummyView, EditView, LinearLayout, ListView, TextView,
};
use cursive::{traits::*, Cursive};

use super::widget;
use crate::generator::{self, MusicModes, MusicNotes};

fn use_get_input_number(siv: &mut Cursive) -> usize {
    let input_box = siv.find_name::<EditView>("random_modes_number").unwrap();

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

fn get_all_check_status(siv: &mut Cursive) -> Option<(Vec<MusicNotes>, Vec<MusicModes>)> {
    let default_roots = MusicNotes::default();

    let choose_roots = (0..default_roots.len())
        .filter_map(|index| {
            let is_need = siv
                .find_name::<Checkbox>(&format!("random_modes_check_roots_{}", index))
                .unwrap()
                .is_checked();

            match is_need {
                true => Some(default_roots[index]),
                false => None,
            }
        })
        .collect::<Vec<MusicNotes>>();

    if choose_roots.len() == 0 {
        return None;
    }

    let default_modes = MusicModes::default();

    let choose_modes = (0..default_modes.len())
        .filter_map(|index| {
            let is_need = siv
                .find_name::<Checkbox>(&format!("random_modes_check_modes_{}", index))
                .unwrap()
                .is_checked();

            match is_need {
                true => Some(default_modes[index]),
                false => None,
            }
        })
        .collect::<Vec<MusicModes>>();

    if choose_modes.len() == 0 {
        return None;
    }

    Some((choose_roots, choose_modes))
}

fn use_change_display(siv: &mut Cursive) {
    let input_number = use_get_input_number(siv);

    if input_number == 0 {
        return;
    }

    let user_choose = get_all_check_status(siv);

    match user_choose {
        Some((choose_roots, choose_modes)) => {
            let new_random_data =
                generator::get_random_modes(choose_roots, choose_modes, input_number);

            siv.find_name::<Dialog>("random_modes_display")
                .unwrap()
                .set_content(TextView::new(new_random_data));
        }
        _ => {
            widget::use_show_message(siv, "請選擇至少一個項目。");
        }
    }
}

fn get_controller_view() -> BoxedView {
    let roots_list =
        MusicNotes::default()
            .iter()
            .enumerate()
            .fold(ListView::new(), |list, (index, note)| {
                list.child(
                    &note.to_string(),
                    Checkbox::new()
                        .checked()
                        .with_name(format!("random_modes_check_roots_{}", index)),
                )
            });

    let modes_list =
        MusicModes::default()
            .iter()
            .enumerate()
            .fold(ListView::new(), |list, (index, mode)| {
                list.child(
                    &mode.to_string(),
                    Checkbox::new()
                        .checked()
                        .with_name(format!("random_modes_check_modes_{}", index)),
                )
            });

    let layout = widget::get_controller_frame(
        LinearLayout::vertical()
            .child(Button::new("更新", use_change_display))
            .child(DummyView)
            .child(
                ListView::new().child(
                    "數量",
                    EditView::new()
                        .content("8")
                        .with_name("random_modes_number"),
                ),
            )
            .child(DummyView)
            .child(
                LinearLayout::horizontal()
                    .child(roots_list)
                    .child(DummyView)
                    .child(modes_list),
            ),
    );

    BoxedView::boxed(layout)
}

pub fn main() -> BoxedView {
    let controller = get_controller_view();
    let result = generator::get_random_modes_by_default();
    let layout = LinearLayout::horizontal()
        .child(controller)
        .child(
            Dialog::text(result)
                .title("隨機的調式")
                .with_name("random_modes_display")
                .full_screen(),
        )
        .full_screen();

    BoxedView::boxed(layout)
}
