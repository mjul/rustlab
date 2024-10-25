# Type System Explorations

Playground for associated types and more.

# Type Coupling with Associated Types
Three *études* on coupling domain entities and their ID-types to have type-safe
object identifiers without extra code or "NewTypes".

1) [Unidirectionally linking the Entity to a specific ID-type](src/unidirectional_associated_type_link.rs)
2) [Bidirectionally linking the Entity and its ID-type](src/bidirectional_associated_type_links.rs)
3) [Reducing the code even more with PhantomData](src/phantom_associated_type_links.rs)


