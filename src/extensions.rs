use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use bevy::ecs::component::{Component, Mutable};
use bevy::ecs::query::{QueryData, ReadOnlyQueryData};
use bevy::ecs::world::Mut;

use super::base::{ModQ, ModQMut, ModQuery, ModQueryMut};

#[derive(Debug)]
pub struct CopiedQ<T>(PhantomData<T>);
#[derive(Debug)]
pub struct ClonedQ<T>(PhantomData<T>);
#[derive(Debug)]
pub struct AsDerefQ<T>(PhantomData<T>);
#[derive(Debug)]
pub struct AsDerefMutQ<T>(PhantomData<T>);
#[derive(Debug)]
pub struct OrDefaultQ<T>(PhantomData<T>);

/// Clones a type when it is retrieved
///
/// ## Example
/// ```
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Clone, Component)]
/// struct CloneMe;
///
/// fn example(query: Query<Cloned<CloneMe>>) {
///     let _: CloneMe = query.get_single().unwrap();
/// }
/// ```
///
/// ## Counter Example: Type must be clone
/// ```compile_fail
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component)]
/// struct NotClone;
///
/// fn example(query: Query<Cloned<NotClone>>) {
///     let _: NotClone = query.get_single().unwrap();
/// }
/// ```
///
/// ## Counter Example: Why would you want this
/// ```compile_fail
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Clone,Component)]
/// struct CloneMe;
///
/// fn example(query: Query<Cloned<Cloned<CloneMe>>>) {
///     let _: CloneMe = query.get_single().unwrap();
/// }
/// ```
pub type Cloned<T> = ModQ<ClonedQ<T>>;
impl<T: Component + Clone> ModQuery for ClonedQ<T> {
    type FromQuery = &'static T;
    type ModItem<'a> = T;

    fn modify_reference(t: <Self::FromQuery as QueryData>::Item<'_>) -> Self::ModItem<'_> {
        t.clone()
    }

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::ModItem<'wlong>) -> Self::ModItem<'wshort> {
        item
    }
}

/// Copies a type when it is retrieved
/// ## Example
/// ```
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Clone, Copy, Component)]
/// struct CopyMe;
///
/// fn example(query: Query<Copied<CopyMe>>) {
///     let _: CopyMe = query.get_single().unwrap();
/// }
/// ```
///
/// ## Counter Example: Type must be clone
/// ```compile_fail
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component)]
/// struct NotCopy;
///
/// fn bad_example(query: Query<Copied<NotCopy>>) {
///     let _: NotCopy = query.get_single().unwrap();
/// }
/// ```
///
/// ## Counter Example: Why would you want this
/// ```compile_fail
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Clone, Copy, Component)]
/// struct CopyMe;
///
/// fn bad_example(query: Query<Copied<Copied<CopyMe>>>) {
///     let _: CopyMe = query.get_single().unwrap();
/// }
/// ```
pub type Copied<T> = ModQ<CopiedQ<T>>;
impl<T: Component + Copy> ModQuery for CopiedQ<T> {
    type FromQuery = &'static T;
    type ModItem<'a> = T;

    fn modify_reference(t: <Self::FromQuery as QueryData>::Item<'_>) -> Self::ModItem<'_> {
        *t
    }

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::ModItem<'wlong>) -> Self::ModItem<'wshort> {
        item
    }
}

/// Returns the dereferenced component
/// ## Example
/// ```
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component, Deref)]
/// struct WrappedBool(bool);
///
/// fn example(query: Query<AsDeref<WrappedBool>>) {
///     let _: &bool = query.get_single().unwrap();
/// }
/// ```
/// ## Counter Example: Type must be Deref
/// ```compile_fail
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component)]
/// struct WrappedBool(bool);
///
/// fn bad_example(query: Query<AsDeref<WrappedBool>>) {
///     let _: &bool = query.get_single().unwrap();
/// }
/// ```
/// ## Counter Example: Nested Derefs are not currently supported
/// ```compile_fail
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
///
/// #[derive(Component,Deref)]
/// struct WrappedBool(bool);
///
/// #[derive(Component,Deref)]
/// struct Wwb(WrappedBool);
///
/// fn bad_example(query: Query<AsDeref<AsDeref<WrappedBool>>>) {
///     let _: &bool = query.get_single().unwrap();
/// }
/// ```
pub type AsDeref<T> = ModQ<AsDerefQ<T>>;
impl<T: Component + Deref> ModQuery for AsDerefQ<T> {
    type FromQuery = &'static T;
    type ModItem<'a> = &'a <T as Deref>::Target;

    fn modify_reference(t: <Self::FromQuery as QueryData>::Item<'_>) -> Self::ModItem<'_> {
        t.deref()
    }

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::ModItem<'wlong>) -> Self::ModItem<'wshort> {
        item
    }
}

/// Returns the dereferenced component as a [`Mut`](bevy::ecs::world::Mut), or a reference if it is
/// readonly.
///
/// ## Example
/// ```
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component, Deref, DerefMut)]
/// struct WrappedBool(bool);
///
/// fn example(mut query: Query<AsDerefMut<WrappedBool>>) {
///     let _: Mut<bool> = query.get_single_mut().unwrap();
///     let _: &bool = query.get_single().unwrap();
/// }
/// ```
/// ## Counter Example: Type must be DerefMut
/// ```compile_fail
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component, Deref)]
/// struct WrappedBool(bool);
///
/// fn bad_example(mut query: Query<AsDerefMut<WrappedBool>>) {
///     let _: Mut<bool> = query.get_single_mut().unwrap();
///     let _: &bool = query.get_single().unwrap();
/// }
/// ```
/// ## Counter Example: DerefMut does not really compose well with others (Why would it?)
/// ```compile_fail
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component, Deref, DerefMut)]
/// struct WrappedBool(bool);
///
/// fn bad_example(query: Query<Copied<AsDerefMut<WrappedBool>>>) {
///     let _: bool = query.get_single().unwrap();
/// }
///
/// ```
pub type AsDerefMut<T> = ModQMut<AsDerefMutQ<T>>;
impl<T: Component<Mutability = Mutable> + DerefMut> ModQueryMut for AsDerefMutQ<T> {
    type FromQuery = &'static mut T;
    type ModItem<'a> = Mut<'a, <T as Deref>::Target>;
    type ReadOnly = AsDeref<T>;

    fn modify_reference(t: <Self::FromQuery as QueryData>::Item<'_>) -> Self::ModItem<'_> {
        t.map_unchanged(|t| t.deref_mut())
    }

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::ModItem<'wlong>) -> Self::ModItem<'wshort> {
        item
    }
}

/// Returns a copy of the dereferenced value (alias of `Copied<AsDeref<T>`)
/// ## Example
/// ```
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component, Deref)]
/// struct WrappedBool(bool);
///
/// fn example(query: Query<AsDerefCopied<WrappedBool>>) {
///     let _: bool = query.get_single().unwrap();
/// }
/// ```
/// ## Counter example: Outer type must implement Deref
/// ```compile_fail
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component, Copy)]
/// struct WrappedBool(bool);
///
/// fn example(query: Query<AsDerefCopied<WrappedBool>>) {
///     let _: bool = query.get_single().unwrap();
/// }
/// ```
/// ## Counter example: Inner type must implement Copy
///
/// But noteably, the outer type does NOT need to implement Copy
///
/// ```compile_fail
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component, Deref)]
/// struct WrappedBool(Vec<bool>);
///
/// fn example(query: Query<AsDerefCopied<WrappedBool>>) {
///     let _: Vec<bool> = query.get_single().unwrap();
/// }
/// ```
pub type AsDerefCopied<T> = Copied<AsDeref<T>>;
impl<T: Component + Deref> ModQuery for CopiedQ<AsDeref<T>>
where
    <T as Deref>::Target: Copy,
{
    type FromQuery = &'static T;
    type ModItem<'a> = <T as Deref>::Target;

    fn modify_reference(t: <Self::FromQuery as QueryData>::Item<'_>) -> Self::ModItem<'_> {
        *t.deref()
    }

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::ModItem<'wlong>) -> Self::ModItem<'wshort> {
        item
    }
}

/// Returns a clone of the dereferenced value (alias of `Cloned<AsDeref<T>>`)
/// ## Example
/// ```
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component, Deref)]
/// struct WrappedBool(Vec<bool>);
///
/// fn example(query: Query<AsDerefCloned<WrappedBool>>) {
///     let _: Vec<bool> = query.get_single().unwrap();
/// }
/// ```
/// ## Counter example: Outer type must implement Deref
/// ```compile_fail
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component, Clone)]
/// struct WrappedBool(Vec<bool>);
///
/// fn example(query: Query<AsDerefCloned<WrappedBool>>) {
///     let _: Vec<bool> = query.get_single().unwrap();
/// }
/// ```
/// ## Counter example: Inner type must implement Clone
///
/// But notably, the outer type does NOT need to implement Clone
/// ```compile_fail
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
///
/// struct Uncloneable;
///
/// #[derive(Component, Deref)]
/// struct WrappedBool(Uncloneable);
///
/// fn example(query: Query<AsDerefCloned<WrappedBool>>) {
///     let _: Uncloneable = query.get_single().unwrap();
/// }
/// ```
pub type AsDerefCloned<T> = Cloned<AsDeref<T>>;
impl<T: Component + Deref> ModQuery for ClonedQ<AsDeref<T>>
where
    <T as Deref>::Target: Clone,
{
    type FromQuery = &'static T;
    type ModItem<'a> = <T as Deref>::Target;

    fn modify_reference(t: <Self::FromQuery as QueryData>::Item<'_>) -> Self::ModItem<'_> {
        t.deref().clone()
    }

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::ModItem<'wlong>) -> Self::ModItem<'wshort> {
        item
    }
}

/// First either clones component T or gets the default value, then dereferences this value and
/// copies it.
///
/// This is primarily useful over [`AsDerefCopiedOrDefault`] when default for the component is
/// different than the default for the dereferenced type.
///
/// ## Example
/// ```
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component, Deref, Clone)]
/// struct Temperature(f32);
///
/// // Notably the default for Temperature is different than the default for the
/// // dereferenced value. Using this type, if the component is not present on
/// // the entity, the query will return 20.0, rather than 0.0.
/// impl Default for Temperature {
///     fn default() -> Self {
///         Self(20.0)
///     }
/// }
///
/// fn example(query: Query<AsDerefCopiedOfClonedOrDefault<Temperature>>) {
///     let _: f32 = query.get_single().unwrap();
/// }
/// ```
/// ## Counter example: Outer type must implement Default, Deref AND Clone
/// ```compile_fail
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component, Deref)]
/// struct Temperature(f32);
///
/// impl Default for Temperature {
///     fn default() -> Self {
///         Self(20.0)
///     }
/// }
///
/// fn bad_example(query: Query<AsDerefCopiedOfClonedOrDefault<Temperature>>) {
///     let _: f32 = query.get_single().unwrap();
/// }
/// ```
/// ## Counter example: Dereferenced type must implement Copy
///
/// ```compile_fail
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component, Deref)]
/// struct Temperatures(Vec<f32>);
///
/// impl Default for Temperatures {
///     fn default() -> Self {
///         Self(vec![20.0])
///     }
/// }
///
/// fn bad_example(query: Query<AsDerefCopiedOfClonedOrDefault<Temperatures>>) {
///     let _: Vec<f32> = query.get_single().unwrap();
/// }
///
/// ```
pub type AsDerefCopiedOfClonedOrDefault<T> = Copied<AsDeref<OrDefault<Cloned<T>>>>;
impl<T: Component + Clone + Deref + Default> ModQuery for CopiedQ<AsDeref<OrDefault<Cloned<T>>>>
where
    <T as Deref>::Target: Copy,
{
    type FromQuery = Option<&'static T>;
    type ModItem<'a> = <T as Deref>::Target;

    fn modify_reference(t: <Self::FromQuery as QueryData>::Item<'_>) -> Self::ModItem<'_> {
        *t.cloned().unwrap_or_default().deref()
    }

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::ModItem<'wlong>) -> Self::ModItem<'wshort> {
        item
    }
}

/// First either copies component T or gets the default value, then dereferences this value and
/// copies it.
///
/// This is primarily useful over [`AsDerefCopiedOrDefault`] when default for the component is
/// different than the default for the dereferenced type.
///
/// ## Example
/// ```
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component, Deref, Clone, Copy)]
/// struct Temperature(f32);
///
/// // Notably the default for Temperature is different than the default for the
/// // dereferenced value. Using this type, if the component is not present on
/// // the entity, the query will return 20.0, rather than 0.0.
/// impl Default for Temperature {
///     fn default() -> Self {
///         Self(20.0)
///     }
/// }
///
/// fn example(query: Query<AsDerefCopiedOfCopiedOrDefault<Temperature>>) {
///     let _: f32 = query.get_single().unwrap();
/// }
/// ```
/// ## Counter example: Outer type must implement Default, Deref AND Copy
/// ```compile_fail
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component, Deref, Clone)]
/// struct NoCopyTemperature(f32);
///
/// impl Default for NoCopyTemperature {
///     fn default() -> Self {
///         Self(20.0)
///     }
/// }
///
/// fn bad_example(query: Query<AsDerefCopiedOfCopiedOrDefault<NoCopyTemperature>>) {
///     let _: f32 = query.get_single().unwrap();
/// }
/// ```
/// ## Counter example: Dereferenced type must implement Copy
///
/// ```compile_fail
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component, Deref)]
/// struct Temperatures(Vec<f32>);
///
/// impl Default for Temperatures {
///     fn default() -> Self {
///         Self(vec![20.0])
///     }
/// }
///
/// fn bad_example(query: Query<AsDerefCopiedOfCopiedOrDefault<Temperatures>>) {
///     let _: Vec<f32> = query.get_single().unwrap();
/// }
/// ```
pub type AsDerefCopiedOfCopiedOrDefault<T> = Copied<AsDeref<OrDefault<Copied<T>>>>;
impl<T: Component + Copy + Deref + Default> ModQuery for CopiedQ<AsDeref<OrDefault<Copied<T>>>>
where
    <T as Deref>::Target: Copy,
{
    type FromQuery = Option<&'static T>;
    type ModItem<'a> = <T as Deref>::Target;

    fn modify_reference(t: <Self::FromQuery as QueryData>::Item<'_>) -> Self::ModItem<'_> {
        *t.copied().unwrap_or_default().deref()
    }

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::ModItem<'wlong>) -> Self::ModItem<'wshort> {
        item
    }
}

/// First either clones component T or gets the default value, then dereferences this value and
/// clones it.
///
/// This is primarily useful over [`AsDerefClonedOrDefault`] when default for the component is
/// different than the default for the dereferenced type.
///
/// ## Example
/// ```
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component, Deref, Clone)]
/// struct Temperatures(Vec<f32>);
///
/// // Notably the default for Temperature is different than the default for the
/// // dereferenced value. Using this type, if the component is not present on
/// // the entity, the query will return 20.0, rather than 0.0.
/// impl Default for Temperatures {
///     fn default() -> Self {
///         Self(vec![20.0])
///     }
/// }
///
/// fn example(query: Query<AsDerefClonedOfClonedOrDefault<Temperatures>>) {
///     let _: Vec<f32> = query.get_single().unwrap();
/// }
/// ```
/// ## Counter example: Outer type must implement Default, Deref AND Clone
/// ```compile_fail
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component, Deref)]
/// struct Temperatures(Vec<f32>);
///
/// impl Default for Temperatures {
///     fn default() -> Self {
///         Self(vec![20.0])
///     }
/// }
///
/// fn bad_example(query: Query<AsDerefClonedOfClonedOrDefault<Temperatures>>) {
///     let _: Vec<f32> = query.get_single().unwrap();
/// }
/// ```
/// ## Counter example: Dereferenced type must implement Clone
///
/// ```compile_fail
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
///
/// struct Uncloneable;
///
/// #[derive(Component, Deref, Clone)]
/// struct Temperature(Uncloneable);
///
/// impl Default for Temperature {
///     fn default() -> Self {
///         Self(Uncloneable)
///     }
/// }
///
/// fn bad_example(query: Query<AsDerefClonedOfClonedOrDefault<Temperature>>) {
///     let _: Uncloneable = query.get_single().unwrap();
/// }
///
/// ```
pub type AsDerefClonedOfClonedOrDefault<T> = Cloned<AsDeref<OrDefault<Cloned<T>>>>;
impl<T: Component + Clone + Deref + Default> ModQuery for ClonedQ<AsDeref<OrDefault<Cloned<T>>>>
where
    <T as Deref>::Target: Clone,
{
    type FromQuery = Option<&'static T>;
    type ModItem<'a> = <T as Deref>::Target;

    fn modify_reference(t: <Self::FromQuery as QueryData>::Item<'_>) -> Self::ModItem<'_> {
        t.cloned().unwrap_or_default().deref().clone()
    }

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::ModItem<'wlong>) -> Self::ModItem<'wshort> {
        item
    }
}

// ModQuery: OrX, works on any readonly query
/// If the query exists on the entity it is returned, or else the default for the query result
/// ## Example
/// ```
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component, Clone, Copy, Default)]
/// struct Velocity2D{x: f32, y: f32};
///
/// // Note: This query is also aliased as `CopiedOrDefault`
/// fn example(query: Query<OrDefault<Copied<Velocity2D>>>) {
///     // If item does not have Velocity2D, a default is created
///     let _: Velocity2D = query.get_single().unwrap();
/// }
/// ```
/// ## Counter example: Can't use on component directly
/// ```compile_fail
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component, Clone, Copy, Default)]
/// struct Velocity2D{x: f32, y: f32};
///
/// fn bad_example(query: Query<OrDefault<Velocity2D>>) {
///     let _: Velocity2D = query.get_single().unwrap();
/// }
/// ```
/// ## Example: Default for references
/// Normally default is not implemented for &T, even if T: Default. The following will not work
/// ```compile_fail
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component, Copy, Clone, Default)]
/// struct Velocity2D{x: f32, y: f32};
///
/// fn example(query: Query<OrDefault<&Velocity2D>>) {
///     let _: &Velocity2D = query.get_single().unwrap();
/// }
/// ```
///
/// But you can implement it manually if you don't want to copy/clone components but still have a
/// default. You'll have to try something like this though:
/// ```
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component)]
/// struct Velocity2D{x: f32, y: f32};
///
/// const DEFAULT_VEL: Velocity2D = Velocity2D {x: 0.0, y: 0.0};
///
/// impl Default for &Velocity2D {
///     fn default() -> Self {
///         &DEFAULT_VEL
///     }
/// }
///
/// fn example(query: Query<OrDefault<&Velocity2D>>) {
///     let _: &Velocity2D = query.get_single().unwrap();
/// }
/// ```
pub type OrDefault<T> = ModQ<OrDefaultQ<T>>;
impl<T: ReadOnlyQueryData> ModQuery for OrDefaultQ<T>
where
    for<'a> <T as QueryData>::Item<'a>: Default,
{
    type FromQuery = Option<T>;
    type ModItem<'b> = T::Item<'b>;

    fn modify_reference(t: <Self::FromQuery as QueryData>::Item<'_>) -> Self::ModItem<'_> {
        t.unwrap_or_default()
    }

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::ModItem<'wlong>) -> Self::ModItem<'wshort> {
        <T as QueryData>::shrink(item)
    }
}

/// Returns a copy of component or default. See [`Copied`] and [`OrDefault`]
/// ```
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component, Clone, Copy, Default)]
/// struct Velocity2D{x: f32, y: f32};
///
/// fn example(query: Query<CopiedOrDefault<Velocity2D>>) {
///     // If item does not have Velocity2D, a default is created
///     let _: Velocity2D = query.get_single().unwrap();
/// }
/// ```
pub type CopiedOrDefault<T> = OrDefault<Copied<T>>;

/// Returns a clone of component or default. See [`Cloned`] and [`OrDefault`]
/// ```
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component, Clone, Default)]
/// struct Velocity2D{x: f32, y: f32};
///
/// fn example(query: Query<ClonedOrDefault<Velocity2D>>) {
///     // If item does not have Velocity2D, a default is created
///     let _: Velocity2D = query.get_single().unwrap();
/// }
/// ```
pub type ClonedOrDefault<T> = OrDefault<Cloned<T>>;

/// Returns a copy of component's dereferenced value, or default for that type. See [`Copied`], [`AsDeref`] and [`OrDefault`]
///
/// If you want a copied value of the component's default value instead of the default value of the
/// dereferenced type, see [`AsDerefCopiedOfCopiedOrDefault`] or [`AsDerefCopiedOfClonedOrDefault`]
/// ```
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component, Deref)]
/// struct IsFrozen(bool);
///
/// fn example(query: Query<AsDerefCopiedOrDefault<IsFrozen>>) {
///     // If IsFrozen is not present, will default to `false`
///     let _: bool = query.get_single().unwrap();
/// }
/// ```
pub type AsDerefCopiedOrDefault<T> = OrDefault<AsDerefCopied<T>>;

/// Returns a clone of component's dereferenced value, or default for that type. See [`Cloned`], [`AsDeref`] and [`OrDefault`]
///
/// If you want a cloned value of the component's default value instead of the default value of the
/// dereferenced type, see [`AsDerefClonedOfClonedOrDefault`]
/// ```
/// # use bevy_query_ext::prelude::*;
/// # use bevy::prelude::*;
/// #[derive(Component, Deref)]
/// struct FriendNames(Vec<String>);
///
/// fn example(query: Query<AsDerefClonedOrDefault<FriendNames>>) {
///     let _: Vec<String> = query.get_single().unwrap();
/// }
/// ```
pub type AsDerefClonedOrDefault<T> = OrDefault<AsDerefCloned<T>>;
