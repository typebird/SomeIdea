use std::{fmt, num::NonZeroUsize};

use super::random;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum MusicNotes {
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Fs,
    G,
    Ab,
    A,
    Bb,
    B,
}

impl fmt::Display for MusicNotes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MusicNotes::C => "C",
                MusicNotes::Db => "Db",
                MusicNotes::D => "D",
                MusicNotes::Eb => "Eb",
                MusicNotes::E => "E",
                MusicNotes::F => "F",
                MusicNotes::Fs => "F#",
                MusicNotes::G => "G",
                MusicNotes::Ab => "Ab",
                MusicNotes::A => "A",
                MusicNotes::Bb => "Bb",
                MusicNotes::B => "B",
            }
        )
    }
}

impl MusicNotes {
    pub fn default() -> [MusicNotes; 12] {
        [
            MusicNotes::C,
            MusicNotes::Db,
            MusicNotes::D,
            MusicNotes::Eb,
            MusicNotes::E,
            MusicNotes::F,
            MusicNotes::Fs,
            MusicNotes::G,
            MusicNotes::Ab,
            MusicNotes::A,
            MusicNotes::Bb,
            MusicNotes::B,
        ]
    }
}

pub fn get_random_notes(choose_list: Vec<MusicNotes>, number: usize) -> String {
    match NonZeroUsize::new(number) {
        None => String::new(),
        Some(n) => random::get_random_items::<MusicNotes>(&choose_list, n)
            .iter()
            .map(|note| note.to_string())
            .reduce(|main, note| format!("{}, {}", main, note))
            .unwrap(),
    }
}

pub fn get_random_notes_by_default() -> String {
    get_random_notes(MusicNotes::default().to_vec(), 8)
}
