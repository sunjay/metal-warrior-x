//! Generates the spritesheet for this game
//!
//! This should be run from the root folder of this repo with the command:
//!
//!     cargo run --release -p spritesheets

use std::path::Path;
use std::sync::{Arc, Mutex};
use std::num::NonZeroU32;

use spritec::tasks::WeakFileCache;
use spritec::math::{Rgb, Rgba};
use spritec::query3d::{
    File,
    CameraQuery,
    LightQuery,
    GeometryQuery,
    GeometryFilter,
};
use spritec::renderer::{
    FileQuery,
    ThreadRenderContext,
    RenderJob,
    RenderNode,
    RenderLayout,
    LayoutType,
    Size,
    RenderedImage,
    Outline,
    RenderCamera,
    RenderLights,
};

/// It is undefined behaviour to pass zero to this function
const fn nz32(size: u32) -> NonZeroU32 {
    unsafe {
        NonZeroU32::new_unchecked(size)
    }
}

const PLAYER_SIZE: Size = Size {
    width: nz32(32),
    height: nz32(32),
};

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

    let mut nodes = Vec::new();
    generate_player_sprites(&mut nodes, &mut file_cache)?;

    let job = RenderJob {
        scale: nz32(4),
        root: RenderNode::Layout(RenderLayout {
            nodes,
            layout: LayoutType::Grid {
                cols: NonZeroU32::new(1).unwrap(),
            },
        }),
    };

    let mut ctx = ThreadRenderContext::new()?;
    let sheet = job.execute(&mut ctx)?;
    sheet.save("assets/spritesheet.png")?;

    Ok(())
}

fn generate_player_sprites(
    nodes: &mut Vec<RenderNode>,
    file_cache: &mut WeakFileCache,
) -> Result<(), anyhow::Error> {
    let player_file = file_cache.open_gltf(Path::new("assets/gltf/player.gltf"))?;

    for (camera, lights) in cameras_lights(&player_file) {
        nodes.push(RenderNode::RenderedImage(RenderedImage {
            size: PLAYER_SIZE,
            background: Rgba {r: 0.0, g: 0.0, b: 0.0, a: 0.0},
            camera,
            lights,
            ambient_light: Rgb::white() * 0.4,
            geometry: FileQuery {
                query: GeometryQuery {
                    models: GeometryFilter::all_in_default_scene(),
                    animation: None,
                },

                file: player_file.clone(),
            },

            outline: Outline {color: Rgba::black(), thickness: 0.0},
        }));
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
