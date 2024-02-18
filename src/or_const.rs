use std::borrow::Borrow;
use std::marker::PhantomData;

use bevy::ecs::query::{ReadOnlyQueryData, WorldQuery};

use super::base::{ModQ, ModQuery};
use super::extensions::AsDeref;

macro_rules! or_const {
    ($OrConst:ident, $OrConstQ:ident, $AsDerefOrConst: ident, $const_type:ty, $wrapped:literal) => {

        #[derive(Debug)]
        pub struct $OrConstQ<T, const V: $const_type>(PhantomData<T>);


        #[cfg(feature="all_docs")]
        paste::paste! {
            #[doc = "When `T` implements `Borrow` for `"]
            #[doc = stringify!($const_type)]
            #[doc = "`, this will return that value or the specified value if T has no result.\n"]
            #[doc = "It's unlikely you'll use this by itself, see [`" $AsDerefOrConst "`] for example of its use."]
            pub type $OrConst<T, const V: $const_type> = ModQ<$OrConstQ<T, V>>;
        }
        #[cfg(not(feature="all_docs"))]
        pub type $OrConst<T, const V: $const_type> = ModQ<$OrConstQ<T, V>>;

        #[cfg(feature="all_docs")]
        paste::paste!{
            #[doc="When `T` implements `Deref` for "]
            #[doc = stringify!($const_type)]
            #[doc=", this will return that value or the specified value if T has no result"]
            #[doc = "## Examples"]
            #[doc = "```"]
            #[doc = "# use bevy::prelude::*;"]
            #[doc = "# use bevy_query_ext::" $AsDerefOrConst ";"]
            #[doc = "#[derive(Component, Deref)]"]
            #[doc = "pub struct Wrapped(" $const_type ");\n\n"]
            #[doc = "fn example(query: Query<" $AsDerefOrConst "<Wrapped, " $wrapped ">>) {"]
            #[doc = "   let _: " $const_type " = query.get_single().unwrap();"]
            #[doc = "}"]
            #[doc = "```"]
            pub type $AsDerefOrConst<T, const V: $const_type> = $OrConst<AsDeref<T>, V>;
        }

        #[cfg(not(feature="all_docs"))]
        pub type $AsDerefOrConst<T, const V: $const_type> = $OrConst<AsDeref<T>, V>;


        impl <T: ReadOnlyQueryData, const V: $const_type> ModQuery for $OrConstQ<T, V>
            where for<'a> <T as WorldQuery>::Item<'a>: Borrow<$const_type> {
            type FromQuery = Option<T>;
            type ModItem<'s> = $const_type;

            fn modify_reference(t: <Self::FromQuery as WorldQuery>::Item<'_>) -> Self::ModItem<'_> {
                t.map(|b|*b.borrow()).unwrap_or(V)
            }

            fn shrink<'wlong: 'wshort, 'wshort>(item: Self::ModItem<'wlong>) -> Self::ModItem<'wshort> {
                item
            }
        }
    }
}

or_const!(OrChar, OrCharQ, AsDerefOrChar, char, "'b'");
or_const!(OrBool, OrBoolQ, AsDerefOrBool, bool, true);
or_const!(OrIsize, OrIsizeQ, AsDerefOrIsize, isize, 1);
or_const!(OrUsize, OrUsizeQ, AsDerefOrUsize, usize, 1);
or_const!(OrI128, OrI128Q, AsDerefOrI128, i128, 1);
or_const!(OrU128, OrU128Q, AsDerefOrU128, u128, 1);
or_const!(OrI64, OrI64Q, AsDerefOrI64, i64, 1);
or_const!(OrU64, OrU64Q, AsDerefOrU64, u64, 1);
or_const!(OrI32, OrI32Q, AsDerefOrI32, i32, 1);
or_const!(OrU32, OrU32Q, AsDerefOrU32, u32, 1);
or_const!(OrI16, OrI16Q, AsDerefOrI16, i16, 1);
or_const!(OrU16, OrU16Q, AsDerefOrU16, u16, 1);
or_const!(OrI8, OrI8Q, AsDerefOrI8, i8, 1);
or_const!(OrU8, OrU8Q, AsDerefOrU8, u8, 1);
