use std::fmt::Display;

use bevy::{asset::LoadState, prelude::*, utils::HashMap};
use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::states::AppState;

pub struct IndexerPlugin;

impl Plugin for IndexerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            IndexedImageServer::new()
                .preload(vec!["indexed/wings.png".into(), "indexed/legs.png".into()]),
        )
        .insert_resource(GreacherPalettes::default())
        .add_system_set(
            SystemSet::on_update(AppState::Loading)
                .with_system(GreacherPalettes::init_color_palettes)
                .with_system(IndexedImageServer::do_preload),
        );
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn from_raw(bytes: &[u8]) -> Color {
        if bytes.len() != 4 {
            panic!("Bad color raw format!");
        }

        Color {
            r: bytes[0],
            g: bytes[1],
            b: bytes[2],
            a: bytes[3],
        }
    }
}

impl From<Color> for Vec<u8> {
    fn from(val: Color) -> Self {
        vec![val.r, val.g, val.b, val.a]
    }
}

impl From<Color> for [u8; 4] {
    fn from(val: Color) -> Self {
        [val.r, val.g, val.b, val.a]
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GreacherColorPalette {
    pub dark: Color,
    pub darkish: Color,
    pub basic: Color,
    pub highlight: Color,
}

impl GreacherColorPalette {
    const DARK_MAP: Color = Color {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };
    const DARKISH_MAP: Color = Color {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    const BASIC_MAP: Color = Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    const HIGHLIGHT_MAP: Color = Color {
        r: 255,
        g: 255,
        b: 0,
        a: 255,
    };

    pub fn from_seed(seed: u64) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed);

        GreacherColorPalette {
            dark: Color::new(
                rng.gen_range(0..=255) / 16 * 16,
                rng.gen_range(0..=255) / 16 * 16,
                rng.gen_range(0..=255) / 16 * 16,
                255,
            ),
            darkish: Color::new(
                rng.gen_range(0..=255) / 16 * 16,
                rng.gen_range(0..=255) / 16 * 16,
                rng.gen_range(0..=255) / 16 * 16,
                255,
            ),
            basic: Color::new(
                rng.gen_range(0..=255) / 16 * 16,
                rng.gen_range(0..=255) / 16 * 16,
                rng.gen_range(0..=255) / 16 * 16,
                255,
            ),
            highlight: Color::new(
                rng.gen_range(0..=255) / 16 * 16,
                rng.gen_range(0..=255) / 16 * 16,
                rng.gen_range(0..=255) / 16 * 16,
                255,
            ),
        }
    }

    pub fn from_rng(rng: &mut SmallRng) -> Self {
        GreacherColorPalette {
            dark: Color::new(
                rng.gen_range(0..=255) / 16 * 16,
                rng.gen_range(0..=255) / 16 * 16,
                rng.gen_range(0..=255) / 16 * 16,
                255,
            ),
            darkish: Color::new(
                rng.gen_range(0..=255) / 16 * 16,
                rng.gen_range(0..=255) / 16 * 16,
                rng.gen_range(0..=255) / 16 * 16,
                255,
            ),
            basic: Color::new(
                rng.gen_range(0..=255) / 16 * 16,
                rng.gen_range(0..=255) / 16 * 16,
                rng.gen_range(0..=255) / 16 * 16,
                255,
            ),
            highlight: Color::new(
                rng.gen_range(0..=255) / 16 * 16,
                rng.gen_range(0..=255) / 16 * 16,
                rng.gen_range(0..=255) / 16 * 16,
                255,
            ),
        }
    }

    pub fn from_raw(bytes: &[u8]) -> Self {
        if bytes.len() != 16 {
            panic!("Bad palette format!")
        };

        GreacherColorPalette {
            dark: Color::from_raw(&bytes[0..4]),
            darkish: Color::from_raw(&bytes[4..8]),
            basic: Color::from_raw(&bytes[8..12]),
            highlight: Color::from_raw(&bytes[12..16]),
        }
    }

    pub fn map(&self, color: Color) -> Color {
        match color {
            Self::DARK_MAP => self.dark,
            Self::DARKISH_MAP => self.darkish,
            Self::BASIC_MAP => self.basic,
            Self::HIGHLIGHT_MAP => self.highlight,
            _ => color,
        }
    }
}

impl Default for GreacherColorPalette {
    fn default() -> Self {
        GreacherColorPalette {
            dark: Color::new(255, 0, 0, 255),
            darkish: Color::new(0, 255, 0, 255),
            basic: Color::new(0, 0, 255, 255),
            highlight: Color::new(255, 255, 0, 255),
        }
    }
}

pub struct GreacherPalettes {
    pub palette_source: Option<Handle<Image>>,
    pub palettes: Vec<GreacherColorPalette>,
}

impl Default for GreacherPalettes {
    fn default() -> Self {
        Self { palette_source: Default::default(), palettes: vec![GreacherColorPalette::default()] }
    }
}

impl GreacherPalettes {
    pub fn init_color_palettes(
        mut greacher_palettes: ResMut<GreacherPalettes>,
        asset_server: Res<AssetServer>,
        images: Res<Assets<Image>>,
    ) {
        let palette_source: Handle<Image> = asset_server.load("palette.png");

        let load_status = asset_server.get_load_state(&palette_source);

        if load_status == LoadState::Loading || load_status == LoadState::NotLoaded {
            return;
        } else if load_status == LoadState::Failed {
            panic!("Failed to load palette! Is the assets folder present in the build?");
        }

        let palette = images.get(&palette_source).unwrap();

        dbg!(palette);

        let data = &palette.data;
        let mut palettes = vec![];

        for row_id in 0..(palette.texture_descriptor.size.height as usize) {
            let row = &data[(row_id * 4 * 4)..(row_id * 4 * 4 + 16)];

            palettes.push(GreacherColorPalette::from_raw(row));
        }

        *greacher_palettes = GreacherPalettes {
            palette_source: Some(palette_source),
            palettes,
        };
    }
}

pub struct IndexedImageServer {
    preloaded: Vec<String>,
    indexed_handles: HashMap<Handle<Image>, Vec<Handle<Image>>>,
}

#[derive(Debug)]
pub enum ImageIndexError {
    NotLoaded,
    LoadFailure,
}

impl Display for ImageIndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageIndexError::NotLoaded => {
                f.write_str("NotLoaded")?;
            }
            ImageIndexError::LoadFailure => {
                f.write_str("LoadFailure")?;
            }
        }

        Ok(())
    }
}

impl IndexedImageServer {
    pub fn new() -> Self {
        IndexedImageServer {
            preloaded: vec![],
            indexed_handles: HashMap::new(),
        }
    }

    pub fn preload(mut self, paths: Vec<String>) -> Self {
        self.preloaded = paths;
        self
    }

    fn do_preload(
        mut state: ResMut<State<AppState>>,
        mut server: ResMut<IndexedImageServer>,
        greacher_palettes: Res<GreacherPalettes>,
        asset_server: Res<AssetServer>,
        mut image_assets: ResMut<Assets<Image>>,
    ) {
        for path in server.preloaded.clone() {
            let result = server.generate_indexed_images(
                &asset_server.load(&path),
                &greacher_palettes,
                &asset_server,
                &mut image_assets,
            );

            match result {
                Ok(_) => {}
                Err(why) => match why {
                    ImageIndexError::NotLoaded => {
                        return; // not ready to index stuff yet.
                    }
                    ImageIndexError::LoadFailure => {
                        panic!("Failed to preload indexed!");
                    }
                },
            }
        }

        state.set(AppState::InGame).unwrap();
    }

    pub fn get(&self, source: &Handle<Image>, palette_index: usize) -> Handle<Image> {
        if !self.indexed_handles.contains_key(source) {
            panic!("Image not loaded as indexed!")
        }

        self.indexed_handles[source][palette_index].clone()
    }

    fn generate_indexed_images(
        &mut self,
        source: &Handle<Image>,
        greacher_palettes: &GreacherPalettes,
        asset_server: &Res<AssetServer>,
        image_assets: &mut ResMut<Assets<Image>>,
    ) -> Result<(), ImageIndexError> {
        if self.indexed_handles.contains_key(source) {
            return Ok(());
        }

        let load_status = asset_server.get_load_state(source);

        if load_status == LoadState::Loading  || load_status == LoadState::NotLoaded{
            return Err(ImageIndexError::NotLoaded);
        } else if load_status == LoadState::Failed {
            return Err(ImageIndexError::LoadFailure);
        }

        let source_image = image_assets.get(source).unwrap().clone();

        let mut indexed_images = vec![];

        for palette in &greacher_palettes.palettes {
            indexed_images.push(self.generate_indexed_image(&source_image, palette));
        }

        let mut indexed_handles = vec![];

        for image in indexed_images {
            indexed_handles.push(image_assets.add(image));
        }

        self.indexed_handles.insert(source.clone(), indexed_handles);

        Ok(())
    }

    fn generate_indexed_image(&mut self, source: &Image, palette: &GreacherColorPalette) -> Image {
        let mut data = vec![];

        let chunks = source.data.chunks_exact(4);

        for color in chunks {
            data.append(&mut (palette.map(Color::from_raw(color))).into());
        }

        Image::new(
            source.texture_descriptor.size,
            source.texture_descriptor.dimension,
            data,
            source.texture_descriptor.format,
        )
    }
}
