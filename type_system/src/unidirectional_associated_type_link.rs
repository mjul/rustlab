//! Linking types with associated types
//!
//! This example links an Entity and its ID field type so an
//! identifier for one type of entity is not mistaken for an
//! identifier for another type of object.
//!
//! The linking is from Entity to EntityId only.
//! Here we use a new type for the ID and have to manually ensure that it is
//! not used as ID for more than one Entity type.

use std::fmt::Debug;
use std::hash::{Hash, Hasher};

/// An identifier for an [Entity].
/// Identifiers are value types and implement the `Copy` trait.
pub trait EntityId: Copy + Clone + Debug + PartialEq + Eq + Hash {}

/// A domain entity
pub trait Entity {
    type Id: EntityId;

    /// Unique identifier for this entity.
    fn id(&self) -> Self::Id;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FooId(i64);

impl EntityId for FooId {}

pub struct Foo {
    id: FooId,
}
impl Entity for Foo {
    type Id = FooId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn entity_id() {
        let id1 = FooId(1);
        let foo1 = Foo { id: id1 };
    }
}
