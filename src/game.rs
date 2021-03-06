/* bevsteroids/src/game.rs
This file is the primary place for central game data and logic.
*/

use bevy::prelude::*;
use bevy::render::texture::FilterMode;

use crate::components::{
    main_camera::*,
    moving::*,
    looping::*,
    player_controller::*,
    asteroid::*,
    ship::*,
};
use crate::game_config::GameConfig;

// RESOURCES

/// TextureHandles resource
/// Keeps track of texture handles that are needed later on in the game
pub struct TextureHandles {
    pub bullet_texture: Handle<Texture>,
}

// COMPONENTS

// SYSTEMS

/// init system - runs once at the start of the game to set everything up
/// inputs:
///     * cmds          - command buffer to send all setup commands to
///     * cfg           - game configuration loaded at startup
///     * asset_server  - asset loader resource
///     * materials     - material resource for sprite rendering
pub fn init(
    mut cmds: Commands,
    cfg: Res<GameConfig>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("Initializing game...");

    // TODO: Load the filename from external file (non-compiled)
    // AssetServer::load() seemed to have troubles with a non-static lifetime
    // argument being passed. This requires further investigation on my part
    // if I want to be able to pass a reference to a string slice that was
    // populated at runtime...
    let asteroid_texture = asset_server.load(cfg.asteroid);
    let ship_texture = asset_server.load(cfg.ship);
    let bullet_texture = asset_server.load(cfg.bullet);

    // Insert the texture handles into a resource so they are accessible later
    cmds.insert_resource(TextureHandles {
        bullet_texture,
    });

    // Push resources and entities to add to the world into the command buffer
    // Cameras
    cmds.spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);    // Flag this camera as the main one
    
    // TODO: wrap entity creation in functions to clean up this setup script
    // Asteroids
    cmds.spawn_bundle(SpriteBundle {
        material: materials.add(asteroid_texture.into()),
        ..Default::default()
    })
    .insert_bundle(LoopingBundle {
        looping: Looping { radius: 128.0 },  // TODO: centrally define this
        // Specify initial conditions (position and velocity) for the entity
        moving: MovingBundle::new_in_plane(
            -500.0, 0.0, 0.0,
            15.0, -50.0, 1.0
        )
    })
    .insert(Asteroid);

    // Ship
    cmds.spawn_bundle(SpriteBundle {
        material: materials.add(ship_texture.into()),
        ..Default::default()
    })
    .insert_bundle(LoopingBundle {
        looping: Looping { radius: 50.0 },   // TODO: centrally define this
        // Specify initial conditions (position and velocity) for the entity
        moving: MovingBundle::new_in_plane(
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0
        )
    })
    // TODO: centrally define these parameters
    .insert(PlayerController::new(250.0, 3.14, 0.08))
    .insert(Ship);
}

/// texture_update_sys system - Listens to update events for Assets<Texture>
/// resources to apply any changes
/// inputs:
///     * ev_asset  - event listener tuned to updates to texture resources
///     * textures  - texture asset resource for applying changes
pub fn texture_update_sys(
    mut ev_asset: EventReader<AssetEvent<Texture>>,
    mut textures: ResMut<Assets<Texture>>
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                // A texture was just loaded
                trace!("Event {:?} recieved on texture id {:?}", ev, handle);
                let texture = textures.get_mut(handle).unwrap();

                // Apply filtering to anti-alias the texture
                texture.sampler.min_filter = FilterMode::Linear;
                texture.sampler.mag_filter = FilterMode::Linear;
            }
            AssetEvent::Modified { handle} => {
                trace!("Event {:?} recieved on texture id {:?}", ev, handle);
            }
            AssetEvent::Removed { handle: _ } => {}
        }
    }
}
