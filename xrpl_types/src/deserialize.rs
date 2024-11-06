mod deserializer;

pub use deserializer::*;

/// Serializes XRPL objects to a [`Deserializer`]
pub trait Deserialize {
    /// Deserialize the object
    fn deserialize<S: Deserializer>(&self, deserializer: &mut S) -> Result<Self, S::Error> where Self: Sized;
}
