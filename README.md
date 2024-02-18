[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

A collection of types used to simplify how components are used in queries, by implementing
bevy's [`WorldQuery`](https://docs.rs/bevy/latest/bevy/ecs/query/trait.WorldQuery.html) on generics.

# Example of use
```rust
use bevy::prelude::*;
use std::ops::Deref;
use bevy_query_ext::AsDerefOrU32;

// Component which indicates ammo left in a weapon
// A weapon with an ammo component can't be used if ammo is 0
// A weapon with no ammo component, like a knife, can be used
#[derive(Component, Deref)]
struct Ammo(u32);

fn identify_usable_weapons_1(weapons: Query<(&Name, Option<&Ammo>)>) {

    for (name, ammo_count) in weapons.iter() {
        if ammo_count.map(|ammo_count|*ammo_count.deref()).unwrap_or(1) > 0 {
            println!("{:?} can be used!", name)
        }
    }
}

// AsDerefOrU32<T, V> is a type alias for OrU32<AsDeref<T>, V>
fn identify_usable_weapons_2(weapons: Query<(&Name, AsDerefOrU32<Ammo, 1>)>) {
    for (name, ammo_count) in weapons.iter() {
        if ammo_count > 0 {
            println!("{:?} can be used!", name)
        }
    }
}

// If you find yourself reusing a type in the same way across multiple systems, just use a type 
// alias for it and you can use it like that in code.
type AmmoCount = AsDerefOrU32<Ammo, 1>;
```

# Basic types

Our crate is composed of these basic types:
* [`AsDeref<T>`](https://docs.rs/bevy_query_ext/latest/bevy_query_ext/type.AsDeref.html) - Returns T dereferenced (a la `Deref` trait)
* [`AsDerefMut<T>`](https://docs.rs/bevy_query_ext/latest/bevy_query_ext/type.AsDerefMut.html) - Returns T dereferenced (a la `DerefMut` trait)
* [`Copied<T>`](https://docs.rs/bevy_query_ext/latest/bevy_query_ext/type.Copied.html) - Returns T copied (a la `Copy` trait)
* [`Cloned<T>`](https://docs.rs/bevy_query_ext/latest/bevy_query_ext/type.Cloned.html) - Returns T cloned (a la `Clone` trait)
* [`OrDefault<T>`](https://docs.rs/bevy_query_ext/latest/bevy_query_ext/type.OrDefault.html) - Returns T if the entity has this component, or its default (a la `Default` trait)
* [`OrBool<T, const V: bool>`](https://docs.rs/bevy_query_ext/latest/bevy_query_ext/type.OrBool.html), [`OrChar<T, const V: bool>`](https://docs.rs/bevy_query_ext/latest/bevy_query_ext/type.OrChar.html), [`OrUsize<T, const V: usize>`](https://docs.rs/bevy_query_ext/latest/bevy_query_ext/type.OrUsize.html), etc. -
Returns T.borrow() (a la `Borrow` trait), or the constant provided if the entity does not have
this component

You can use these basic types by themselves, but they are most useful composed with each other (except `DerefMut`).
There are type aliases for most valid compositions of these types. For example,
`type AsDerefCopied<T> = Copied<AsDeref<T>>`.

# Note about destructors and Entity components

A common use case the author has had is having components that dereference to entities. But using `AsDerefCopied` in
long tuple-queries can lead to you getting entities mixed up since they are no longer type-protected. It is better
in these cases to make the dereference fields public and use destructuring assignments if possible. For instance, if
your component was `pub struct PointsTo(Entity)`, you can get the value using something like 
`for (&PointsTo(points_to), <..>) = query.iter()` instead.

This is also not a problem for derived queries with named fields.


# Note on limitations of composition

Because of the way WorldQuery works, in order to implement it, we need to know that the
lifetime for all parameters is "covariant," or rather that if `'a : 'b` then `T<'a> : T<'b>`. 

For more information on variance, see [here](https://doc.rust-lang.org/nomicon/subtyping.html).

However, Rust's associated types are assumed to be invariant, and there is no language feature
that allows us to enforce that the associated types are covariant types so even if we could get
around it with unsafe code we couldn't restrict it to the proper types. This means we can't
simply implement `<T: WorldQuery> Copied<T>`. However, thanks to the way Rust's typing works,
we can manually implement composed types like `Copied<AsDeref<T>>` even with `Copied<T>` already implemented.

This crate attempts to manually implement all useful compositions of the types here, and to
indicate these with specialized type aliases. [`AsDerefCopiedOfClonedOrDefault`](https://docs.rs/bevy_query_ext/latest/bevy_query_ext/type.AsDerefCopiedOfClonedOrDefault.html) is probably the
most egregious of these.

# Bevy Compatibility

Since there can be breaking changes to our APIs we will have different versions for our code than the compatible
bevy library, but we'll list compatibility here.

| bevy | bevy_query_ext |
|------|----------------|
| 0.13 | 0.3            |
| 0.12 | 0.2            |
| 0.11 | 0.1            |

# Feedback

Could the docs be clearer? Is a useful composition missing? Is there a type I haven't considered? 
For any of these, please open an issue or PR on [Github](https://github.com/Testare/bevy_query_ext)! 
