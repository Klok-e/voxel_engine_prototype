#![feature(min_const_generics)]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;

pub mod camera_move_system;
pub mod core;
pub mod directions;
pub mod gameplay_state;
pub mod ui;
pub mod voxels;
pub mod destroy_on_touch_system;
pub mod world_change_apply_system;
pub mod game_config;