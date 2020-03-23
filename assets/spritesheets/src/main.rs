//! Generates the spritesheet for this game
//!
//! This should be run from the root folder of this repo with the command:
//!
//!     cargo run --release -p spritesheets

use std::path::Path;
use std::sync::{Arc, Mutex};
use std::num::NonZeroU32;

use spritec::tasks::WeakFileCache;
use spritec::math::{Rgb, Rgba, Milliseconds};
use spritec::query3d::{
    File,
    CameraQuery,
    LightQuery,
    GeometryQuery,
    GeometryFilter,
    AnimationQuery,
    AnimationPosition,
};
use spritec::renderer::{
    FileQuery,
    ThreadRenderContext,
    RenderJob,
    RenderNode,
    RenderLayout,
    GridLayout,
    GridLayoutCell,
    Size,
    RenderedImage,
    Outline,
    RenderCamera,
    RenderLights,
};

/// It is undefined behaviour to pass zero to this function
const fn nz32(size: u32) -> NonZeroU32 {
    unsafe { NonZeroU32::new_unchecked(size) }
}

const PLAYER_SIZE: Size = Size {
    width: nz32(32),
    height: nz32(32),
};
const WALK_FRAMES: usize = 8;

static CAMERAS_LIGHTS: &[(&str, &str)] = &[
    ("camera_N", "light_N"),
    ("camera_NE", "light_NE"),
    ("camera_E", "light_E"),
    ("camera_SE", "light_SE"),
    ("camera_S", "light_S"),
    ("camera_SW", "light_SW"),
    ("camera_W", "light_W"),
    ("camera_NW", "light_NW"),
];

fn main() -> Result<(), anyhow::Error> {
    let mut file_cache = WeakFileCache::default();

    let mut cells = Vec::new();
    generate_player_sprites(&mut cells, &mut file_cache)?;

    let job = RenderJob {
        scale: nz32(4),
        root: RenderNode::Layout(RenderLayout::Grid(GridLayout {
            rows: nz32(CAMERAS_LIGHTS.len() as u32),
            cols: nz32(WALK_FRAMES as u32),
            cell_size: PLAYER_SIZE,
            cells,
        })),
    };

    let mut ctx = ThreadRenderContext::new()?;
    let sheet = job.execute(&mut ctx)?;
    sheet.save("assets/spritesheet.png")?;

    Ok(())
}

fn generate_player_sprites(
    cells: &mut Vec<Vec<GridLayoutCell>>,
    file_cache: &mut WeakFileCache,
) -> Result<(), anyhow::Error> {
    let player_file = file_cache.open_gltf(Path::new("assets/gltf/player.gltf"))?;

    for (camera, lights) in cameras_lights(&player_file) {
        let mut row = Vec::new();
        for i in 0..WALK_FRAMES {
            row.push(GridLayoutCell::single(RenderNode::RenderedImage(RenderedImage {
                size: PLAYER_SIZE,
                background: Rgba {r: 0.0, g: 0.0, b: 0.0, a: 0.0},
                camera: camera.clone(),
                lights: lights.clone(),
                ambient_light: Rgb::white() * 0.4,
                geometry: FileQuery {
                    query: GeometryQuery {
                        models: GeometryFilter::all_in_default_scene(),
                        animation: Some(AnimationQuery {
                            name: None,
                            position: AnimationPosition::RelativeTime {
                                start_time: Milliseconds::from_msec(0.0),
                                weight: i as f32 / WALK_FRAMES as f32,
                            },
                        }),
                    },

                    file: player_file.clone(),
                },

                outline: Outline {color: Rgba::black(), thickness: 0.0},
            })));
        }
        cells.push(row);
    }

    Ok(())
}

fn cameras_lights(file: &Arc<Mutex<File>>) -> impl Iterator<Item = (RenderCamera, RenderLights)> + '_ {
    CAMERAS_LIGHTS.iter().map(move |(cam_name, light_name)| {
        let cam = RenderCamera::Query(FileQuery {
            query: CameraQuery::Named {
                name: cam_name.to_string(),
                scene: None,
            },

            file: file.clone(),
        });

        let light = RenderLights::Query(FileQuery {
            query: LightQuery::Named {
                name: light_name.to_string(),
                scene: None,
            },

            file: file.clone(),
        });

        (cam, light)
    })
}
