use bevy::ecs as by_ecs;
use bevy::prelude::{self as by};
use bevy_rand::{
  prelude as br,
  traits::{ForkableAsRng, ForkableRng},
};

use frunk;

pub trait VariadicEnableAmbiguityDetectionForLabels {
  fn enable_ambiguity_detection_for_labels1<'a>(self, app: &'a mut by::App) -> &'a mut by::App;
}

impl VariadicEnableAmbiguityDetectionForLabels for frunk::HNil {
  fn enable_ambiguity_detection_for_labels1<'a>(self, app: &'a mut by::App) -> &'a mut by::App {
    app
  }
}

impl<
  H: by_ecs::schedule::ScheduleLabel,
  T: frunk::prelude::HList + VariadicEnableAmbiguityDetectionForLabels,
> VariadicEnableAmbiguityDetectionForLabels for frunk::HCons<H, T>
{
  fn enable_ambiguity_detection_for_labels1<'a>(self, app: &'a mut by::App) -> &'a mut by::App {
    let label = self.head;
    app.edit_schedule(label, |schedule| {
      schedule.set_build_settings(by_ecs::schedule::ScheduleBuildSettings {
        ambiguity_detection: by_ecs::schedule::LogLevel::Warn,
        ..by::default()
      });
    });
    app
  }
}

pub trait AppExtVariadicEnableAmbiguityDetectionForLabels {
  fn enable_ambiguity_detection_for_labels<
    'a,
    H: by_ecs::schedule::ScheduleLabel,
    T: frunk::prelude::HList + VariadicEnableAmbiguityDetectionForLabels,
  >(
    &'a mut self,
    labels: frunk::HCons<H, T>,
  ) -> &'a mut Self;
}

impl AppExtVariadicEnableAmbiguityDetectionForLabels for by::App {
  fn enable_ambiguity_detection_for_labels<
    'a,
    H: by_ecs::schedule::ScheduleLabel,
    T: frunk::prelude::HList + VariadicEnableAmbiguityDetectionForLabels,
  >(
    &'a mut self,
    labels: frunk::HCons<H, T>,
  ) -> &'a mut Self {
    labels.enable_ambiguity_detection_for_labels1(self)
  }
}
