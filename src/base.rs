use std::marker::PhantomData;

use bevy::ecs::archetype::Archetype;
use bevy::ecs::component::{ComponentId, ComponentInitializer};
use bevy::ecs::query::{FilteredAccess, QueryData, ReadOnlyQueryData, WorldQuery};
use bevy::ecs::storage::Table;
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

    fn modify_reference(from: <Self::FromQuery as WorldQuery>::Item<'_>) -> Self::ModItem<'_>;

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::ModItem<'wlong>) -> Self::ModItem<'wshort>;
}

/// A trait implementation that can be implemented to simplify creating
/// a WorldQuery based off another WorldQuery.
pub trait ModQueryMut {
    type FromQuery: WorldQuery;
    type ModItem<'q>;
    type ReadOnly: ReadOnlyQueryData<
        State = <<Self as ModQueryMut>::FromQuery as WorldQuery>::State,
    >;

    fn modify_reference(from: <Self::FromQuery as WorldQuery>::Item<'_>) -> Self::ModItem<'_>;

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::ModItem<'wlong>) -> Self::ModItem<'wshort>;
}

unsafe impl<T: ModQuery> QueryData for ModQ<T> {
    type ReadOnly = Self;
}

unsafe impl<T: ModQuery> WorldQuery for ModQ<T> {
    type Fetch<'w> = <T::FromQuery as WorldQuery>::Fetch<'w>;
    type Item<'w> = T::ModItem<'w>;
    type State = <T::FromQuery as WorldQuery>::State;

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::Item<'wlong>) -> Self::Item<'wshort> {
        T::shrink(item)
    }

    const IS_DENSE: bool = <T::FromQuery>::IS_DENSE;

    #[inline]
    unsafe fn init_fetch<'w>(
        world: UnsafeWorldCell<'w>,
        state: &Self::State,
        last_run: bevy::ecs::component::Tick,
        this_run: bevy::ecs::component::Tick,
    ) -> Self::Fetch<'w> {
        <T::FromQuery as WorldQuery>::init_fetch(world, state, last_run, this_run)
    }

    #[inline]
    unsafe fn set_archetype<'w>(
        fetch: &mut Self::Fetch<'w>,
        state: &Self::State,
        archetype: &'w Archetype,
        table: &'w Table,
    ) {
        <T::FromQuery as WorldQuery>::set_archetype(fetch, state, archetype, table);
    }

    unsafe fn set_table<'w>(fetch: &mut Self::Fetch<'w>, state: &Self::State, table: &'w Table) {
        <T::FromQuery as WorldQuery>::set_table(fetch, state, table);
    }

    unsafe fn fetch<'w>(
        fetch: &mut Self::Fetch<'w>,
        entity: bevy::prelude::Entity,
        table_row: bevy::ecs::storage::TableRow,
    ) -> Self::Item<'w> {
        T::modify_reference(<T::FromQuery as WorldQuery>::fetch(
            fetch, entity, table_row,
        ))
    }

    fn update_component_access(state: &Self::State, access: &mut FilteredAccess<ComponentId>) {
        <T::FromQuery as WorldQuery>::update_component_access(state, access)
    }

    fn init_state(component_initializer: &mut ComponentInitializer) -> Self::State {
        <T::FromQuery as WorldQuery>::init_state(component_initializer)
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
    type Item<'w> = T::ModItem<'w>;
    type State = <T::FromQuery as WorldQuery>::State;

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::Item<'wlong>) -> Self::Item<'wshort> {
        T::shrink(item)
    }

    const IS_DENSE: bool = <T::FromQuery>::IS_DENSE;

    #[inline]
    unsafe fn init_fetch<'w>(
        world: UnsafeWorldCell<'w>,
        state: &Self::State,
        last_run: bevy::ecs::component::Tick,
        this_run: bevy::ecs::component::Tick,
    ) -> Self::Fetch<'w> {
        <T::FromQuery as WorldQuery>::init_fetch(world, state, last_run, this_run)
    }

    #[inline]
    unsafe fn set_archetype<'w>(
        fetch: &mut Self::Fetch<'w>,
        state: &Self::State,
        archetype: &'w Archetype,
        table: &'w Table,
    ) {
        <T::FromQuery as WorldQuery>::set_archetype(fetch, state, archetype, table);
    }

    unsafe fn set_table<'w>(fetch: &mut Self::Fetch<'w>, state: &Self::State, table: &'w Table) {
        <T::FromQuery as WorldQuery>::set_table(fetch, state, table);
    }

    unsafe fn fetch<'w>(
        fetch: &mut Self::Fetch<'w>,
        entity: bevy::prelude::Entity,
        table_row: bevy::ecs::storage::TableRow,
    ) -> Self::Item<'w> {
        T::modify_reference(<T::FromQuery as WorldQuery>::fetch(
            fetch, entity, table_row,
        ))
    }

    fn update_component_access(state: &Self::State, access: &mut FilteredAccess<ComponentId>) {
        <T::FromQuery as WorldQuery>::update_component_access(state, access)
    }

    fn init_state(component_initializer: &mut ComponentInitializer) -> Self::State {
        <T::FromQuery as WorldQuery>::init_state(component_initializer)
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
}
