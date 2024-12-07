#![allow(dead_code)]
#![allow(incomplete_features)]
#![allow(unused_imports)]
#![feature(generic_const_exprs)]

use bevy::app::{self as bevy_app, Update};
use bevy::app::{FixedFirst, FixedLast, FixedPostUpdate, FixedPreUpdate, FixedUpdate};
use bevy::ecs as by_ecs;
use bevy::prelude::{self as by, AppExtStates};
use bevy_rand::{
  prelude as br,
  traits::{ForkableAsRng, ForkableRng},
};

use rand_core::RngCore;
use sha2::{Digest, Sha512};

use greed_lib_rs::{Direction, Greed, Playable};

use frunk;

mod ambiguity_detection;
use ambiguity_detection::AppExtVariadicEnableAmbiguityDetectionForLabels;

#[derive(by::Component)]
struct GreedGame {
  greed: Greed,
}

#[derive(by::States, Debug, Clone, PartialEq, Eq, Hash)]
enum GreedGameState {
  MainMenu,
  InGameGreedClassic,
}

fn get_rand_seed_slice<const N: u8>(seed: &str) -> [u8; N as usize] {
  let mut hasher = Sha512::new();
  hasher.update(seed);
  let hash = hasher.finalize();
  <[u8; N as usize]>::try_from(&hash[0..N as usize])
    .expect("The hash did not have the requested amount of bytes")
}

fn hello_world_system(mut exit: by::EventWriter<by::AppExit>) {
  println!("Hello, world!");
  exit.send(by::AppExit::Success);
}

#[allow(unreachable_code, unused_variables)]
fn main() {
  let main_menu_seed = get_rand_seed_slice::<32>("Oranges");
  let labels_for_ambiguity_detection = frunk::hlist![
    by::First,
    by::PreUpdate,
    by::StateTransition,
    by::RunFixedMainLoop,
    by::Update,
    by::PostUpdate,
    by::Last,
    by::PreStartup,
    by::Startup,
    by::PostStartup,
    by::RunFixedMainLoop,
    FixedFirst,
    FixedPreUpdate,
    FixedUpdate,
    FixedPostUpdate,
    FixedLast,
  ];

  by::App::new()
    .add_plugins((by::MinimalPlugins, bevy::state::app::StatesPlugin))
    .add_plugins(br::EntropyPlugin::<br::ChaCha20Rng>::with_seed(
      main_menu_seed,
    ))
    .enable_ambiguity_detection_for_labels(labels_for_ambiguity_detection)
    .insert_state(GreedGameState::MainMenu)
    .add_systems(Update, hello_world_system)
    .run();
  println!("Thank you for playing GreedRS!");
}
