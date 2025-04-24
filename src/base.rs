use core::marker::PhantomData;

use bevy::ecs::archetype::Archetype;
use bevy::ecs::component::ComponentId;
use bevy::ecs::query::{FilteredAccess, QueryData, ReadOnlyQueryData, WorldQuery};
use bevy::ecs::storage::Table;
use bevy::ecs::world::World;
use bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell;

/// An empty structure type
/// Used to simplify the different modified queries
/// so we don't have as much boilerplate for all the implementations
#[derive(Debug)]
pub struct ModQ<T>(PhantomData<T>);

/// A form of [`ModQ`] for mutable queries
#[derive(Debug)]
pub struct ModQMut<T>(PhantomData<T>);

/// A trait implementation that can be implemented to simplify creating
/// a ReadOnlyQueryData based off another ReadOnlyWorldQuery.
pub trait ModQuery {
    type FromQuery: ReadOnlyQueryData;
    type ModItem<'q>;

    fn modify_reference(from: <Self::FromQuery as QueryData>::Item<'_>) -> Self::ModItem<'_>;

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::ModItem<'wlong>) -> Self::ModItem<'wshort>;
}

/// A trait implementation that can be implemented to simplify creating
/// a WorldQuery based off another WorldQuery.
pub trait ModQueryMut {
    type FromQuery: QueryData;
    type ModItem<'q>;
    type ReadOnly: ReadOnlyQueryData<
        State = <<Self as ModQueryMut>::FromQuery as WorldQuery>::State,
    >;

    fn modify_reference(from: <Self::FromQuery as QueryData>::Item<'_>) -> Self::ModItem<'_>;

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::ModItem<'wlong>) -> Self::ModItem<'wshort>;
}

unsafe impl<T: ModQuery> QueryData for ModQ<T> {
    type ReadOnly = Self;
    type Item<'w> = T::ModItem<'w>;

    const IS_READ_ONLY: bool = true;

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::Item<'wlong>) -> Self::Item<'wshort> {
        T::shrink(item)
    }

    unsafe fn fetch<'w>(
        fetch: &mut Self::Fetch<'w>,
        entity: bevy::prelude::Entity,
        table_row: bevy::ecs::storage::TableRow,
    ) -> Self::Item<'w> {
        unsafe { T::modify_reference(<T::FromQuery as QueryData>::fetch(fetch, entity, table_row)) }
    }
}

unsafe impl<T: ModQuery> WorldQuery for ModQ<T> {
    type Fetch<'w> = <T::FromQuery as WorldQuery>::Fetch<'w>;
    type State = <T::FromQuery as WorldQuery>::State;

    const IS_DENSE: bool = <T::FromQuery>::IS_DENSE;

    #[inline]
    unsafe fn init_fetch<'w>(
        world: UnsafeWorldCell<'w>,
        state: &Self::State,
        last_run: bevy::ecs::component::Tick,
        this_run: bevy::ecs::component::Tick,
    ) -> Self::Fetch<'w> {
        unsafe { <T::FromQuery as WorldQuery>::init_fetch(world, state, last_run, this_run) }
    }

    #[inline]
    unsafe fn set_archetype<'w>(
        fetch: &mut Self::Fetch<'w>,
        state: &Self::State,
        archetype: &'w Archetype,
        table: &'w Table,
    ) {
        unsafe {
            <T::FromQuery as WorldQuery>::set_archetype(fetch, state, archetype, table);
        }
    }

    unsafe fn set_table<'w>(fetch: &mut Self::Fetch<'w>, state: &Self::State, table: &'w Table) {
        unsafe {
            <T::FromQuery as WorldQuery>::set_table(fetch, state, table);
        }
    }

    fn shrink_fetch<'wlong: 'wshort, 'wshort>(fetch: Self::Fetch<'wlong>) -> Self::Fetch<'wshort> {
        <T::FromQuery as WorldQuery>::shrink_fetch(fetch)
    }

    fn update_component_access(state: &Self::State, access: &mut FilteredAccess<ComponentId>) {
        <T::FromQuery as WorldQuery>::update_component_access(state, access)
    }

    fn init_state(world: &mut World) -> Self::State {
        <T::FromQuery as WorldQuery>::init_state(world)
    }

    fn matches_component_set(
        state: &Self::State,
        set_contains_id: &impl Fn(bevy::ecs::component::ComponentId) -> bool,
    ) -> bool {
        <T::FromQuery as WorldQuery>::matches_component_set(state, set_contains_id)
    }

    fn get_state(components: &bevy::ecs::component::Components) -> Option<Self::State> {
        <T::FromQuery as WorldQuery>::get_state(components)
    }
}

// SAFETY: ModQuery comes from a read only query
unsafe impl<T: ModQuery> ReadOnlyQueryData for ModQ<T> {}

unsafe impl<T: ModQueryMut> WorldQuery for ModQMut<T> {
    type Fetch<'w> = <T::FromQuery as WorldQuery>::Fetch<'w>;
    type State = <T::FromQuery as WorldQuery>::State;

    const IS_DENSE: bool = <T::FromQuery>::IS_DENSE;

    #[inline]
    unsafe fn init_fetch<'w>(
        world: UnsafeWorldCell<'w>,
        state: &Self::State,
        last_run: bevy::ecs::component::Tick,
        this_run: bevy::ecs::component::Tick,
    ) -> Self::Fetch<'w> {
        unsafe { <T::FromQuery as WorldQuery>::init_fetch(world, state, last_run, this_run) }
    }

    #[inline]
    unsafe fn set_archetype<'w>(
        fetch: &mut Self::Fetch<'w>,
        state: &Self::State,
        archetype: &'w Archetype,
        table: &'w Table,
    ) {
        unsafe {
            <T::FromQuery as WorldQuery>::set_archetype(fetch, state, archetype, table);
        }
    }

    unsafe fn set_table<'w>(fetch: &mut Self::Fetch<'w>, state: &Self::State, table: &'w Table) {
        unsafe {
            <T::FromQuery as WorldQuery>::set_table(fetch, state, table);
        }
    }

    fn shrink_fetch<'wlong: 'wshort, 'wshort>(fetch: Self::Fetch<'wlong>) -> Self::Fetch<'wshort> {
        <T::FromQuery as WorldQuery>::shrink_fetch(fetch)
    }

    fn update_component_access(state: &Self::State, access: &mut FilteredAccess<ComponentId>) {
        <T::FromQuery as WorldQuery>::update_component_access(state, access)
    }

    fn init_state(world: &mut World) -> Self::State {
        <T::FromQuery as WorldQuery>::init_state(world)
    }

    fn matches_component_set(
        state: &Self::State,
        set_contains_id: &impl Fn(bevy::ecs::component::ComponentId) -> bool,
    ) -> bool {
        <T::FromQuery as WorldQuery>::matches_component_set(state, set_contains_id)
    }

    fn get_state(components: &bevy::ecs::component::Components) -> Option<Self::State> {
        <T::FromQuery as WorldQuery>::get_state(components)
    }
}

unsafe impl<T: ModQueryMut> QueryData for ModQMut<T> {
    type ReadOnly = T::ReadOnly;
    type Item<'w> = T::ModItem<'w>;

    const IS_READ_ONLY: bool = T::FromQuery::IS_READ_ONLY;

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::Item<'wlong>) -> Self::Item<'wshort> {
        T::shrink(item)
    }

    unsafe fn fetch<'w>(
        fetch: &mut Self::Fetch<'w>,
        entity: bevy::prelude::Entity,
        table_row: bevy::ecs::storage::TableRow,
    ) -> Self::Item<'w> {
        unsafe { T::modify_reference(<T::FromQuery as QueryData>::fetch(fetch, entity, table_row)) }
    }
}
