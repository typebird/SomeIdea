use rand::{self, Rng};
use std::fmt;

#[derive(PartialEq, Clone, Copy)]
pub enum Chords {
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

impl fmt::Display for Chords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Chords::Major => "Major",
                Chords::Minor => "Minor",
                Chords::Augmented => "Augmented",
                Chords::Diminished => "Diminished",
                Chords::Sus2 => "Sus2",
                Chords::Maj7 => "Maj7",
                Chords::M7 => "M7",
                Chords::Dom7 => "Dom7",
                Chords::Dom7sus => "Dom7sus",
                Chords::M7b5 => "M7b5",
                Chords::Dim7 => "Dim7",
            }
        )
    }
}

impl Chords {
    pub fn default() -> Vec<Chords> {
        vec![
            Chords::Major,
            Chords::Minor,
            Chords::Augmented,
            Chords::Diminished,
            Chords::Sus2,
            Chords::Maj7,
            Chords::M7,
            Chords::Dom7,
            Chords::Dom7sus,
            Chords::M7b5,
            Chords::Dim7,
        ]
    }

    fn to_chord(self) -> Vec<&'static str> {
        match self {
            Chords::Major => vec![
                "C", "Db", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B",
            ],
            Chords::Minor => vec![
                "Cm", "C#m", "Dm", "Ebm", "Em", "Fm", "F#m", "Gm", "G#m", "Am", "Bbm", "Bm",
            ],
            Chords::Augmented => vec![
                "C+", "Db+", "D+", "Eb+", "E+", "F+", "Gb+", "G+", "Ab+", "A+", "Bb+", "B+",
            ],
            Chords::Diminished => vec![
                "Cdim", "C#dim", "Ddim", "D#dim", "Edim", "Fdim", "F#dim", "Gdim", "G#dim", "Adim",
                "A#dim", "Bdim",
            ],
            Chords::Sus2 => vec![
                "Csus2", "Dbsus2", "Dsus2", "Ebsus2", "Esus2", "Fsus2", "F#sus2", "Gsus2",
                "Absus2", "Asus2", "Bbsus2", "Bsus2",
            ],
            Chords::Maj7 => vec![
                "Cmaj7", "Dbmaj7", "Dmaj7", "Ebmaj7", "Emaj7", "Fmaj7", "Gbmaj7", "Gmaj7",
                "Abmaj7", "Amaj7", "Bbmaj7", "Bmaj7",
            ],
            Chords::M7 => vec![
                "Cm7", "C#m7", "Dm7", "Ebm7", "Em7", "Fm7", "F#m7", "Gm7", "G#m7", "Am7", "Bbm7",
                "Bm7",
            ],
            Chords::Dom7 => vec![
                "C7", "Db7", "D7", "Eb7", "E7", "F7", "F#7", "G7", "Ab7", "A7", "Bb7", "B7",
            ],
            Chords::Dom7sus => vec![
                "C7sus", "C#7sus", "D7sus", "Eb7sus", "E7sus", "F7sus", "F#7sus", "G7sus",
                "Ab7sus", "A7sus", "Bb7sus", "B7sus",
            ],
            Chords::M7b5 => vec![
                "Cm7(b5)", "C#m7(b5)", "Dm7(b5)", "D#m7(b5)", "Em7(b5)", "Fm7(b5)", "F#m7(b5)",
                "Gm7(b5)", "G#m7(b5)", "Am7(b5)", "A#m7(b5)", "Bm7(b5)",
            ],
            Chords::Dim7 => vec![
                "Cdim7", "C#dim7", "Ddim7", "D#dim7", "Edim7", "Fdim7", "F#dim7", "Gdim7",
                "G#dim7", "Adim7", "A#dim7", "Bdim7",
            ],
        }
    }
}

pub fn get_random_chords(match_list: Vec<Chords>) -> &'static str {
    let choose_chords = match_list.iter().fold(Vec::new(), |mut res, val| {
        res.append(&mut val.to_chord());
        res
    });

    get_random_item::<&str>(&choose_chords)
}

pub fn get_random_number(min: u8, max: u8) -> u8 {
    rand::thread_rng().gen_range(min..max)
}

pub fn get_random_notes() -> &'static str {
    let notes = vec![
        "C", "Db", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B",
    ];

    *get_random_item(&notes)
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
