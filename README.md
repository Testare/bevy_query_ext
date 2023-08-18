[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

A collection of types used to simplify how components are used in queries, by implementing
bevy's [`WorldQuery`](bevy::ecs::query::WorldQuery) on generics.

# Example of use
```
# use bevy::prelude::*;
# use std::ops::Deref;
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
* [`AsDeref<T>`] - Returns T dereferenced (a la `Deref` trait)
* [`AsDerefMut<T>`] - Returns T dereferenced (a la `DerefMut` trait)
* [`Copied<T>`] - Returns T copied (a la `Copy` trait)
* [`Cloned<T>`] - Returns T cloned (a la `Clone` trait)
* [`OrDefault<T>`] - Returns T if the entity has this component, or its default (a la `Default` trait)
* [`OrBool<T, const V: bool>`], [`OrChar<T, const V: bool>`], [`OrUsize<T, const V: usize>`], etc. -
Returns T.borrow() (a la `Borrow` trait), or the constant provided if the entity does not have
this component

You can use these basic types by themselves, but they are most useful composed with each other (except `DerefMut`).
There are type aliases for most valid compositions of these types. For example,
`type AsDerefCopied<T> = Copied<AsDeref<T>>`.


# Note on limitations of composition

Because of the way WorldQuery works, in order to implement it, we need to know that the
lifetime for all parameters is "covariant," or rather that if `'a : 'b` then `T<'a> : T<'b>`. 

For more information on variance, see [here](https://doc.rust-lang.org/nomicon/subtyping.html).

However, Rust's associated types are assumed to be invariant, and there is no language feature
that allows us to enforce that the associated types are covariant types so even if we could get
around it with unsafe code we couldn't restrict it to the proper types. This means we can't
simply implement `<T: WorldQuery> Copied<T>`. However, thanks to the way Rust's typing works,
we can manually implement composed types like `Copied<AsDeref<T>>` even if `AsDeref<T>`.

This crate attempts to manually implement all useful compositions of the types here, and to
indicate these with specialized type aliases (`AsDerefCopiedOfClonedOrDefault` is probably the
most egregious for `Copied<AsDeref<OrDefault<Cloned<T>>>>`).

# Bevy Compatibility

Since there can be breaking changes to our APIs we will have different versions for our code than the compatible
bevy library, but we'll list compatibility here.

| bevy | bevy_query_ext |
|------|----------------|
| 0.11 | 0.1            |

# Feedback

Could the docs be clearer? Is a useful composition missing? Is there a type I haven't considered? 
For any of these, please open an issue or PR on [Github](https://github.com/Testare/bevy_query_ext)! 
