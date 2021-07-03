pub mod random_chords;
pub mod random_modes;
pub mod random_notes;

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
