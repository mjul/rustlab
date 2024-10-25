//! Linking types with associated types
//!
//! This example links an Entity and its ID field type so an
//! identifier for one type of entity is not mistaken for an
//! identifier for another type of object.
//!
//! The linking is bidirectional between the Entity and the EntityId.
//!
//! This ensures that an ID type is not used for more than one Entity type.

use std::fmt::Debug;
use std::hash::{Hash, Hasher};

/// An identifier for an [Entity].
/// Identifiers are value types and implement the `Copy` trait.
pub trait EntityId: Copy + Clone + Debug + PartialEq + Eq + Hash + Sized {
    type EntityType: Entity;
}

/// A domain entity
pub trait Entity {
    type Id: EntityId<EntityType = Self>;

    /// Unique identifier for this entity.
    fn id(&self) -> Self::Id;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FooId(i64);

impl EntityId for FooId {
    type EntityType = Foo;
}

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
