#[derive(Clone, Copy, Debug)]
pub enum CapsuleContent<T> where T: Copy + Sync + Send {
  Pending,
  Empty,
  Some(T),
}

impl<T: Copy + Sync + Send> CapsuleContent<T> {
  pub fn unwrap(&self) -> T {
    match self {
      CapsuleContent::Some(out) => *out,
      _ => panic!("Cannot unwrap CapsuleContent unless a value is returned."),
    }
  }
}

pub trait Capsule<K, V> where V: Copy + Sync + Send {
  fn request_content(&self, key: &K) -> CapsuleContent<V>;
  fn set_content(&self, key: K, value: CapsuleContent<V>);
}

#[macro_export]
macro_rules! capsule_get {
  ($context:ident, $repo:ident, $capsule:ident, $key:expr) => {
    {
      use std::any::Any;
      use omnidux_core::capsule::Capsule;

      let repo_uuid = unsafe { $repo::uuid };
      let repo = $context.get_repo(repo_uuid).as_any().downcast_ref::<$repo::Repository>().unwrap();
      repo.capsules.$capsule.request_content($key)
    }
  };
}

#[macro_export]
macro_rules! capsule_set {
  ($context:ident, $repo:ident, $capsule:ident, $key:expr, $value:expr) => {
    {
      use std::any::Any;
      use omnidux_core::capsule::Capsule;

      let repo_uuid = unsafe { $repo::uuid };
      let repo = $context.get_repo(repo_uuid).as_any().downcast_ref::<$repo::Repository>().unwrap();
      repo.capsules.$capsule.set_content($key, $value);
    }
  };
}

/// Implements the default capsule.
/// The default implementation is not safe when used within multiple threads.
#[macro_export]
macro_rules! impl_default_capsule {
  ($name: ident, $keyType: ident, $valueType: ident) => {
    pub struct $name {
      map: std::sync::RwLock<
        std::collections::HashMap<
          $keyType, CapsuleContent<$valueType>
        >
      >,
    }

    impl $name {
      fn new() -> $name {
        $name {
          map: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
      }
    }

    impl omnidux_core::capsule::Capsule<$keyType, $valueType> for $name {
      fn request_content(&self, key: &$keyType) -> CapsuleContent<$valueType> {
        let map = self.map.read().unwrap();
        *map.get(key).or_else(|| Some(&CapsuleContent::Empty)).unwrap()
      }

      fn set_content(&self, key: $keyType, value: CapsuleContent<$valueType>) {
        let mut map = self.map.write().unwrap();
        map.insert(key, value);
      }
    }
  };
}
