//! Linking types with associated types
//!
//! This example links an Entity and its ID field type so an
//! identifier for one type of entity is not mistaken for an
//! identifier for another type of object.
//!
//! The linking is bidirectional between the Entity and the EntityId.
//!
//! In this example, rather than using new types for the ID, we
//! reuse the ID type and use [PhantomData] to create unique variants
//! for each entity.
//!
//! The implementation here is a bit more verbose than [crate::bidirectional_associated_type_links]
//! but the identifier type is reusable across all entities.

use std::any::{type_name, Any};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

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

// If we add a PhantomData<TEntity> field the TEntity has to implement all the required
// traits for the Identifier. We do not want this, e.g:
//
//#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
//pub struct Identifier<TEntity>(i64, PhantomData<TEntity>) where TEntity: Entity<Id=Self>;

// Let's try with implementing the traits without derive and see if we can work around
// these limitations:
pub struct Identifier<TEntity>(i64, PhantomData<TEntity>)
where
    TEntity: Entity<Id = Self>;

impl<T> Copy for Identifier<T> where T: Entity<Id = Self> {}

impl<T> Clone for Identifier<T>
where
    T: Entity<Id = Self>,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData::default())
    }
}
impl<T> Debug for Identifier<T>
where
    T: Entity<Id = Self>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Identifier<{}>({})", type_name::<T>(), self.0))
    }
}

impl<T> PartialEq for Identifier<T>
where
    T: Entity<Id = Self>,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T> Eq for Identifier<T> where T: Entity<Id = Self> {}

impl<T> Hash for Identifier<T>
where
    T: Entity<Id = Self>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> EntityId for Identifier<T>
where
    T: Entity<Id = Self>,
{
    type EntityType = T;
}

type FooId = Identifier<Foo>;

pub struct Foo {
    id: Identifier<Self>,
}

impl Entity for Foo {
    type Id = FooId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

type BarId = Identifier<Bar>;
pub struct Bar {
    id: Identifier<Self>,
}

impl Entity for Bar {
    type Id = BarId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn entity_id_foo_uses_identifier() {
        let id1 = Identifier::<Foo>(1, PhantomData::default());
        let foo1 = Foo { id: id1 };
    }
    #[test]
    fn entity_id_bar_also_uses_identifier() {
        let id1 = Identifier::<Bar>(1, PhantomData::default());
        let bar1 = Bar { id: id1 };
        // It it not possible to assign a Bar identifier to a Foo entity:
        // This does not compile:
        // let foo1 = Foo { id: id1 };
    }
}
