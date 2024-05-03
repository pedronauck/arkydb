pub use downcast::TypeMismatch;
use downcast::{downcast_sync, AnySync};
use dyn_clone::{clone_trait_object, DynClone};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;

pub trait AnyData: AnySync + fmt::Debug + DynClone + 'static {
    fn eq_as_any(&self, other: &(dyn std::any::Any + 'static)) -> bool;
}
clone_trait_object!(AnyData);
downcast_sync!(dyn AnyData);

impl<T> AnyData for T
where
    T: AnySync + fmt::Debug + DynClone + PartialEq + 'static,
{
    fn eq_as_any(&self, other: &(dyn std::any::Any + 'static)) -> bool {
        if let Some(other) = other.downcast_ref::<T>() {
            self == other
        } else {
            false
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Data {
    #[serde(skip_serializing, skip_deserializing)]
    Some(Arc<dyn AnyData>),
    None,
}
impl Data {
    pub fn new<T: AnyData + Sync>(value: T) -> Self {
        Self::Some(Arc::new(value))
    }
    pub fn get<T: AnyData + Sync>(&self) -> Result<&T, downcast::TypeMismatch> {
        match &self {
            Self::Some(value) => value.downcast_ref::<T>(),
            Self::None => Err(downcast::TypeMismatch {
                expected: std::any::type_name::<T>(),
                found: "None",
            }),
        }
    }
    pub fn get_mut<T: AnyData + Sync>(&mut self) -> Result<&mut T, downcast::TypeMismatch> {
        match self {
            Self::Some(value) => Arc::get_mut(value).unwrap().downcast_mut::<T>(),
            Self::None => Err(downcast::TypeMismatch {
                expected: std::any::type_name::<T>(),
                found: "None",
            }),
        }
    }
}
impl Eq for Data {}
impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Some(self_value), Self::Some(other_value)) => {
                self_value.type_id() == other_value.type_id()
                    && self_value.eq_as_any(other_value.as_any())
            }
            (Self::None, Self::None) => true,
            _ => false,
        }
    }
}
impl fmt::Debug for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Some(value) => write!(f, "Some({:?})", value),
            Self::None => write!(f, "None"),
        }
    }
}
impl Default for Data {
    fn default() -> Self {
        Self::None
    }
}
