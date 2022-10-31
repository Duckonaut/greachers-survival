use crate::{color::{Color, GreacherColorPalette}, util::SliceExt};
use bevy::prelude::*;
use lazy_static::lazy_static;
use rand::prelude::*;

pub const GREACHER_HEAD_SIZE: usize = 8;
pub const GREACHER_CANVAS_SIZE: usize = GREACHER_HEAD_SIZE + 2;

lazy_static! {
    static ref NAME_STARTS: Vec<&'static str> = vec![
        "qu", "wr", "br", "tl", "p", "s", "d", "fw", "gh", "j", "kl", "l", "z", "cl", "v", "b",
        "n", "m",
    ];
    static ref NAME_JOINS: Vec<&'static str> = vec!["a", "e", "i", "o", "u"];
    static ref NAME_ENDS: Vec<&'static str> = vec!["g", "mb", "kl", "pw", "nk", "mk"];
    static ref NAME_PREFIXES: Vec<&'static str> =
        vec!["THE ", "big ", "little ", "tiny ", "baby ", "loser "];
    static ref NAME_POSTFIXES: Vec<&'static str> =
        vec!["ford", "ley", " the great", "pop", "wer", "ula", "io"];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ColorMapping {
    Transparent,
    Dark,
    Darkish,
    Basic,
    Highlight,
    White,
}

pub fn generate_greacher_name(rng: &mut SmallRng) -> String {
    let mut name = match rng.gen_range(0..4) {
        0 => format!(
            "{}{}{}",
            NAME_STARTS.random(rng),
            NAME_JOINS.random(rng),
            NAME_ENDS.random(rng)
        ),
        1 => format!(
            "{}{}{}{}",
            NAME_STARTS.random(rng),
            NAME_JOINS.random(rng),
            NAME_ENDS.random(rng),
            NAME_JOINS.random(rng)
        ),
        2 => format!(
            "{}{}{}{}",
            NAME_STARTS.random(rng),
            NAME_JOINS.random(rng),
            NAME_STARTS.random(rng),
            NAME_JOINS.random(rng)
        ),
        3 => format!(
            "{}{}{}{}{}",
            NAME_STARTS.random(rng),
            NAME_JOINS.random(rng),
            NAME_STARTS.random(rng),
            NAME_JOINS.random(rng),
            NAME_ENDS.random(rng)
        ),
        _ => panic!(),
    };

    if rng.gen_range(0..10) == 0 {
        name = format!("{}{}", NAME_PREFIXES.random(rng), name);
    }

    if rng.gen_range(0..10) == 0 {
        name = format!("{}{}", name, NAME_POSTFIXES.random(rng));
    }

    name.to_uppercase()
}

pub fn generate_greacher_head_texture(rng: &mut SmallRng, image: &mut Image, palette: &GreacherColorPalette) {
    let mut template = vec![ColorMapping::Transparent; GREACHER_CANVAS_SIZE * GREACHER_CANVAS_SIZE];

    generate_head_shape(&mut template, rng);
    generate_head_pattern(&mut template, rng);
    generate_eyes(&mut template, rng);

    image.data = create_color_data(&template, palette);
}

fn generate_head_shape(data: &mut [ColorMapping], rng: &mut SmallRng) -> (usize, usize) {
    let size = (rng.gen_range(3..5) * 2, rng.gen_range(4..9));

    let min_y = 1 + 8 - size.1;
    let max_y = GREACHER_HEAD_SIZE;

    let min_x = 5 - size.0 / 2;
    let max_x = 4 + size.0 / 2;

    for j in min_y..=max_y {
        for i in min_x..=max_x {
            data[j * GREACHER_CANVAS_SIZE + i] = ColorMapping::Basic;

            if (j == min_y || j == max_y) && (i == min_x || i == max_x) {
                data[j * GREACHER_CANVAS_SIZE + i] = ColorMapping::Darkish;
            }
        }
    }

    size
}

fn generate_head_pattern(data: &mut [ColorMapping], rng: &mut SmallRng) {
    let mut generator: (usize, usize) = (1 + rng.gen_range(0..8), 1 + rng.gen_range(0..8));

    let pattern_length: usize = rng.gen_range(8..16);

    for _ in 0..pattern_length {
        data[generator.1 * GREACHER_CANVAS_SIZE + generator.0] = ColorMapping::Highlight;
        data[generator.1 * GREACHER_CANVAS_SIZE + (9 - generator.0)] = ColorMapping::Highlight;

        let dir: (isize, isize) = match rng.gen_range(0usize..4usize) {
            0 => (1, 0),
            1 => (-1, 0),
            2 => (0, 1),
            3 => (0, -1),
            _ => (0, 0),
        };

        generator = (
            (1 + (generator.0 as isize + dir.0 + GREACHER_HEAD_SIZE as isize)
                % GREACHER_HEAD_SIZE as isize) as usize,
            (1 + (generator.1 as isize + dir.1 + GREACHER_HEAD_SIZE as isize)
                % GREACHER_HEAD_SIZE as isize) as usize,
        );
    }
}

fn generate_eyes(data: &mut [ColorMapping], rng: &mut SmallRng) {
    let eye_size = (rng.gen_range(1..3), rng.gen_range(1..3));
    let eye_pos = (1 + rng.gen_range(0..2), 1 + rng.gen_range(0..7));

    for j in eye_pos.1..(eye_pos.1 + eye_size.1) {
        for i in eye_pos.0..(eye_pos.0 + eye_size.0) {
            data[j * GREACHER_CANVAS_SIZE + i] = ColorMapping::White;
            data[j * GREACHER_CANVAS_SIZE + (GREACHER_CANVAS_SIZE - 1 - i)] = ColorMapping::White;
        }
    }

    for j in 0..GREACHER_CANVAS_SIZE {
        for i in 0..GREACHER_CANVAS_SIZE {
            let current = data[j * GREACHER_CANVAS_SIZE + i];

            if current != ColorMapping::White && current != ColorMapping::Transparent {
                let nearby = [
                    if j > 0 {
                        data[(j - 1) * 10 + i]
                    } else {
                        ColorMapping::Transparent
                    },
                    if j < GREACHER_CANVAS_SIZE - 1 {
                        data[(j + 1) * 10 + i]
                    } else {
                        ColorMapping::Transparent
                    },
                    if i > 0 {
                        data[j * 10 + (i - 1)]
                    } else {
                        ColorMapping::Transparent
                    },
                    if i < GREACHER_CANVAS_SIZE - 1 {
                        data[j * 10 + (i + 1)]
                    } else {
                        ColorMapping::Transparent
                    },
                ];

                for e in nearby {
                    if e == ColorMapping::White {
                        data[j * GREACHER_CANVAS_SIZE + i] = ColorMapping::Dark;
                        break;
                    }
                }
            }
        }
    }
}

fn create_color_data(template: &[ColorMapping], palette: &GreacherColorPalette) -> Vec<u8> {
    let mut data = vec![0u8; GREACHER_CANVAS_SIZE * GREACHER_CANVAS_SIZE * 4];

    for j in 0..GREACHER_CANVAS_SIZE {
        for i in 0..GREACHER_CANVAS_SIZE {
            let col = match template[j * GREACHER_CANVAS_SIZE + i] {
                ColorMapping::Transparent => Color::new(0, 0, 0, 0),
                ColorMapping::Dark => palette.dark,
                ColorMapping::Darkish => palette.darkish,
                ColorMapping::Basic => palette.basic,
                ColorMapping::Highlight => palette.highlight,
                ColorMapping::White => Color::new(255, 255, 255, 255),
            };

            let col_bytes: Vec<u8> = col.into();

            for k in 0..4 {
                data[(j * GREACHER_CANVAS_SIZE + i) * 4 + k] = col_bytes[k];
            }
        }
    }

    data
}
