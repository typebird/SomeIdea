use super::random;
use std::fmt;

use super::MusicNotes;

#[derive(PartialEq, Clone, Copy)]
pub enum MusicModes {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}

impl fmt::Display for MusicModes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MusicModes::Ionian => "Ionian",
                MusicModes::Dorian => "Dorian",
                MusicModes::Phrygian => "Phrygian",
                MusicModes::Lydian => "Lydian",
                MusicModes::Mixolydian => "Mixolydian",
                MusicModes::Aeolian => "Aeolian",
                MusicModes::Locrian => "Locrian",
            }
        )
    }
}

impl MusicModes {
    pub fn default() -> [MusicModes; 7] {
        [
            MusicModes::Ionian,
            MusicModes::Dorian,
            MusicModes::Phrygian,
            MusicModes::Lydian,
            MusicModes::Mixolydian,
            MusicModes::Aeolian,
            MusicModes::Locrian,
        ]
    }
}

fn get_random_mode(roots: &Vec<MusicNotes>, modes: &Vec<MusicModes>) -> String {
    format!(
        "{} {}",
        random::get_random_item(roots),
        random::get_random_item(modes)
    )
}

pub fn get_random_modes(roots: Vec<MusicNotes>, modes: Vec<MusicModes>, number: usize) -> String {
    match number {
        0 => String::new(),
        1 => get_random_mode(&roots, &modes),
        _ => (0..number)
            .map(|_| get_random_mode(&roots, &modes))
            .reduce(|a, b| format!("{}, {}", a, b))
            .unwrap(),
    }
}

pub fn get_random_modes_by_default() -> String {
    get_random_modes(
        MusicNotes::default().to_vec(),
        MusicModes::default().to_vec(),
        8,
    )
}
