//! Base traits and functionalities for entity codecs.





/// An entity method definition for a specific entity structure.
pub trait ExposedMethod<E> {

    /// Get the method index.
    fn index(&self) -> u16;

}
