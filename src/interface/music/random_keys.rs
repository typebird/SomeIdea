use cursive::traits::*;
use cursive::views::{BoxedView, Dialog, LinearLayout};

use crate::generator;

pub fn main() -> BoxedView {
    let result = generator::get_random_keys();
    let layout = LinearLayout::horizontal().child(
        Dialog::text(result)
            .title("隨機的大小調/調號")
            .full_screen(),
    );

    BoxedView::boxed(layout)
}
