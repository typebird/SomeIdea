use super::random;
use std::{fmt, num::NonZeroUsize};

#[derive(PartialEq, Clone, Copy)]
pub enum MusicChordTypes {
    Major,
    Minor,
    Augmented,
    Diminished,
    Sus2,
    Maj7,
    M7,
    Dom7,
    Dom7sus,
    M7b5,
    Dim7,
}

impl fmt::Display for MusicChordTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MusicChordTypes::Major => "Major",
                MusicChordTypes::Minor => "Minor",
                MusicChordTypes::Augmented => "Augmented",
                MusicChordTypes::Diminished => "Diminished",
                MusicChordTypes::Sus2 => "Sus2",
                MusicChordTypes::Maj7 => "Maj7",
                MusicChordTypes::M7 => "M7",
                MusicChordTypes::Dom7 => "Dom7",
                MusicChordTypes::Dom7sus => "Dom7sus",
                MusicChordTypes::M7b5 => "M7b5",
                MusicChordTypes::Dim7 => "Dim7",
            }
        )
    }
}

impl MusicChordTypes {
    pub fn default() -> [MusicChordTypes; 11] {
        [
            MusicChordTypes::Major,
            MusicChordTypes::Minor,
            MusicChordTypes::Augmented,
            MusicChordTypes::Diminished,
            MusicChordTypes::Sus2,
            MusicChordTypes::Maj7,
            MusicChordTypes::M7,
            MusicChordTypes::Dom7,
            MusicChordTypes::Dom7sus,
            MusicChordTypes::M7b5,
            MusicChordTypes::Dim7,
        ]
    }

    fn to_chord(self) -> Vec<&'static str> {
        match self {
            MusicChordTypes::Major => vec![
                "C", "Db", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B",
            ],
            MusicChordTypes::Minor => vec![
                "Cm", "C#m", "Dm", "Ebm", "Em", "Fm", "F#m", "Gm", "G#m", "Am", "Bbm", "Bm",
            ],
            MusicChordTypes::Augmented => vec![
                "C+", "Db+", "D+", "Eb+", "E+", "F+", "Gb+", "G+", "Ab+", "A+", "Bb+", "B+",
            ],
            MusicChordTypes::Diminished => vec![
                "Cdim", "C#dim", "Ddim", "D#dim", "Edim", "Fdim", "F#dim", "Gdim", "G#dim", "Adim",
                "A#dim", "Bdim",
            ],
            MusicChordTypes::Sus2 => vec![
                "Csus2", "Dbsus2", "Dsus2", "Ebsus2", "Esus2", "Fsus2", "F#sus2", "Gsus2",
                "Absus2", "Asus2", "Bbsus2", "Bsus2",
            ],
            MusicChordTypes::Maj7 => vec![
                "Cmaj7", "Dbmaj7", "Dmaj7", "Ebmaj7", "Emaj7", "Fmaj7", "Gbmaj7", "Gmaj7",
                "Abmaj7", "Amaj7", "Bbmaj7", "Bmaj7",
            ],
            MusicChordTypes::M7 => vec![
                "Cm7", "C#m7", "Dm7", "Ebm7", "Em7", "Fm7", "F#m7", "Gm7", "G#m7", "Am7", "Bbm7",
                "Bm7",
            ],
            MusicChordTypes::Dom7 => vec![
                "C7", "Db7", "D7", "Eb7", "E7", "F7", "F#7", "G7", "Ab7", "A7", "Bb7", "B7",
            ],
            MusicChordTypes::Dom7sus => vec![
                "C7sus", "C#7sus", "D7sus", "Eb7sus", "E7sus", "F7sus", "F#7sus", "G7sus",
                "Ab7sus", "A7sus", "Bb7sus", "B7sus",
            ],
            MusicChordTypes::M7b5 => vec![
                "Cm7(b5)", "C#m7(b5)", "Dm7(b5)", "D#m7(b5)", "Em7(b5)", "Fm7(b5)", "F#m7(b5)",
                "Gm7(b5)", "G#m7(b5)", "Am7(b5)", "A#m7(b5)", "Bm7(b5)",
            ],
            MusicChordTypes::Dim7 => vec![
                "Cdim7", "C#dim7", "Ddim7", "D#dim7", "Edim7", "Fdim7", "F#dim7", "Gdim7",
                "G#dim7", "Adim7", "A#dim7", "Bdim7",
            ],
        }
    }
}

pub fn get_random_chords(match_list: Vec<MusicChordTypes>, number: usize) -> String {
    match NonZeroUsize::new(number) {
        None => String::new(),
        Some(n) => {
            let choose_chords = match_list.iter().fold(Vec::new(), |mut res, val| {
                res.append(&mut val.to_chord());
                res
            });

            random::get_random_items::<&str>(&choose_chords, n).join(", ")
        }
    }
}

pub fn get_random_chords_by_default() -> String {
    get_random_chords(MusicChordTypes::default().to_vec(), 8)
}
