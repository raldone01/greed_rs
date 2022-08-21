use std::{cell::RefCell, collections::HashMap, fs, path::Path, path::PathBuf, rc::Rc};

struct StringResource {
  sid: StrRID,
  string: String,
}
impl StringResource {
  fn new(sid: StrRID, string: String) -> Self {
    Self { sid, string }
  }
}
impl Resource for StringResource {
  fn sid(&self) -> StrRID {
    self.sid
  }

  fn category(&self) -> ResourceCategory {
    ResourceCategory::ASSET(AssetCategory::STRING)
  }
}
struct ImageResource {}
struct VideoResource {}
struct VideoPlayerResource {}
struct AudioResource {}
struct AudioPlayerResource {}

enum ResourceEnum {
  STRING(StringResource),
  IMAGE(ImageResource),
  VIDEO(VideoResource),
  VIDEO_PLAYER(VideoPlayerResource),
  AUDIO(AudioResource),
  AUDIO_PLAYER(AudioPlayerResource),
  CUSTOM(Box<dyn Resource>),
}

enum MutableResourceEnum {}

enum AssetCategory {
  STRING,
  IMAGE,
  VIDEO,
  VIDEO_PLAYER,
  AUDIO,
  AUDIO_PLAYER,
  CUSTOM,
}

enum SettingCategory {
  KEY_MAP,
  CUSTOM,
}

enum InputCategory {
  MOUSE,
  KEYBOARD,
  CONTROLLER,
  CUSTOM,
}

enum GuiCategory {
  TEXT,
  PROGRESS,
  BUTTON,
  CUSTOM,
}

enum ResourceCategory {
  ASSET(AssetCategory),
  SETTING(SettingCategory),
  InputCategory(InputCategory),
  GuiCategory(GuiCategory),
  CUSTOM,
}

struct StrNamespace {
  string: String,
}

impl StrNamespace {
  fn ns_iter(&self) -> impl Iterator<StrNamespace> {
    todo!();
  }
}

/// String ResourceID
#[derive(Clone)]
struct StrRID {
  name: String,
}

impl StrRID {
  fn new(name: String) -> Self {
    Self { name }
  }
  fn ns(&self) -> StrNamespace {
    todo!();
  }
  fn name(&self) -> &str {
    todo!();
  }
}

/// Resolved ResourceID
struct RID {
  id: u64,
}
impl RID {
  pub fn new(id: u64) -> Self {
    Self { id }
  }
  pub fn rid(self) -> u64 {
    self.rid
  }
}

trait Resource {
  fn sid(&self) -> StrRID;
  fn category(&self) -> ResourceCategory;
}

trait ReDeSolver {
  /// It is possible to resolve non existing StrRIDs.
  /// Returns the rid and if the resource is already loaded.
  fn resolve(&mut self, sid: &StrRID) -> (RID, bool);
  fn desolve(&self, rid: RID) -> Option<&StrRID>;
}
struct ReDeSolverImpl {
  map: HashMap<StrRID, RID>,
  next_rid: u64,
}
impl ReDeSolverImpl {
  pub fn new() -> Self {
    Self {
      map: HashMap::new(),
      next_rid: 1,
    }
  }
}
impl ReDeSolver for ReDeSolverImpl {
  fn resolve(&mut self, sid: &StrRID) -> (RID, bool) {
    if let Some(rid) = self.map.get(sid) {
      return rid;
    }
    let rid = RID::new(self.next_rid);
    self.next_rid += 1;
    self.map.insert(sid, rid);
    rid
  }

  fn desolve(&self, rid: RID) -> Option<&StrRID> {
    self.map.get(rid)
  }
}

// trait ResourceProvider {
//   type RC;
//   fn get<R: Resource<RC = Self::RC>>(&self, rid: RID) -> Option<&R>;
//   /// Returns a default resource if it doesn't exist.
//   fn get_default<R: Resource<RC = Self::RC> + Default>(&self, rid: RID) -> Option<&R>;
//   fn get_unwrap<R: Resource<RC = Self::RC>>(&self, rid: RID) -> &R {
//     self.get(rid).unwrap()
//   }

//   // impl detail fn set_resource(&mut self, rid: RRID, resource: Resource<RC = Self::RC>);
//   // impl detail fn del_resource(&mut self, rid: RRID);
// }

// trait ResourceProviderMut: ResourceProvider {
//   fn get<R: Resource<RT = Self::RT>>(&self, rid: RID) -> Option<&mut R>;
//   /// Returns a default resource if it doesn't exist.
//   fn get_default<R: Resource<RT = Self::RT> + Default>(&self, rid: RID) -> Option<&mut R>;
//   fn get_unwrap<R: Resource<RT = Self::RT>>(&self, rid: RID) -> &mut R {
//     self.get(rid).unwrap()
//   }
// }

/// The StrRIDs are not fully qualified!
trait RealResourceProvider {
  type StrRIDIter: Iterator<Item = StrRID>;
  fn get(&self, sid: &StrRID) -> Box<dyn Resource>;
  fn iter_sids(&self) -> Self::StrRIDIter;
}

struct FolderResourceProvider {
  folder: PathBuf,
}
impl FolderResourceProvider {
  pub fn new(folder: PathBuf) -> Self {
    Self { folder }
  }
}
impl RealResourceProvider for FolderResourceProvider {
  type StrRIDIter = impl Iterator<Item = StrRID>;
  /// TODO: SID must be fully qualified
  fn get(&self, sid: &StrRID) -> Box<dyn Resource> {
    // TODO: Error handling
    let path = Path::new(&self.folder).join(sid.name());
    Box::new(StringResource::new(
      sid.clone(),
      String::from_utf8(fs::read(path).unwrap()).unwrap(),
    ))
  }

  fn iter_sids(&self) -> Self::StrRIDIter {
    // TODO: Error handling
    let x = fs::read_dir(&self.folder)
      .unwrap()
      .map(|dir| StrRID::new(dir.unwrap().file_name().into_string().unwrap()));
    x
  }
}

struct OverlayResourceProvider {
  // provider_stack: std::vec<>
}
//impl OverlayResourceProvider : ResourceProvider  {
// fn push_provider()
//}

struct ControllerResourceProvider {}

struct Engine {}

impl Engine {}

// impl GlobalResourceProvider for Engine {}
