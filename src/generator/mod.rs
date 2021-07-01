mod music_chords;
mod music_notes;
mod random;

use rand::{self, Rng};

pub use music_chords::MusicChordTypes;
pub use music_notes::MusicNotes;

pub fn get_random_chords(match_list: Vec<MusicChordTypes>, number: usize) -> String {
    music_chords::get_random_chords(match_list, number)
}

/// 取得隨機的速度值（在 30 到 210 之間）。
pub fn get_random_tempo() -> usize {
    random::get_random_number(30, 210)
}

/// 取得隨機的音符列表。
pub fn get_random_notes(number: usize, use_notes: Vec<MusicNotes>) -> String {
    music_notes::get_random_notes(use_notes, number)
}

pub fn get_random_modes() -> String {
    let roots = vec![
        "C", "Db", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B",
    ];

    let modes = vec![
        "Ionian",
        "Dorian",
        "Phrygian",
        "Lydian",
        "Mixolydian",
        "Aeolian",
        "Locrian",
    ];

    format!("{} {}", get_random_item(&roots), get_random_item(&modes))
}

pub fn get_random_meter() -> &'static str {
    let meter = vec![
        "2/4", "3/4", "4/4", "5/4", "6/4", "7/4", "3/8", "5/8", "6/8", "7/8", "9/8", "11/8",
        "12/8", "13/8", "15/8", "2/2", "3/2",
    ];

    *get_random_item(&meter)
}

pub fn get_random_keys() -> &'static str {
    let keys = vec![
        "C 大調 / A 小調 （沒有升降記號）",
        "G 大調 / E 小調 （1 個升記號）",
        "D 大調 / B 小調 （2 個升記號）",
        "A 大調 / F# 小調 （3 個升記號）",
        "E 大調 / C# 小調 （4 個升記號）",
        "B 大調 / G# 小調 （5 個升記號）",
        "F# 大調 / D# 小調 （6 個升記號）",
        "C# 大調 / A# 小調 （7 個升記號）",
        "F 大調 / D 小調 （1 個降記號）",
        "Bb 大調 / G 小調 （2 個降記號）",
        "Eb 大調 / C 小調 （3 個降記號）",
        "Ab 大調 / F 小調 （4 個降記號）",
        "Db 大調 / Bb 小調 （5 個降記號）",
        "Gb 大調 / Eb 小調 （6 個降記號）",
        "Cb 大調 / Ab 小調 （7 個降記號）",
    ];

    *get_random_item(&keys)
}

fn get_random_item<T>(list: &Vec<T>) -> &T {
    &list[rand::thread_rng().gen_range(0..list.len())]
}
