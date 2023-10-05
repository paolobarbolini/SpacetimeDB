// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#[allow(unused)]
use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, ser::Serialize},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
    Address,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct InsertOneBoolArgs {
    pub b: bool,
}

impl Reducer for InsertOneBoolArgs {
    const REDUCER_NAME: &'static str = "insert_one_bool";
}

#[allow(unused)]
pub fn insert_one_bool(b: bool) {
    InsertOneBoolArgs { b }.invoke();
}

#[allow(unused)]
pub fn on_insert_one_bool(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &bool) + Send + 'static,
) -> ReducerCallbackId<InsertOneBoolArgs> {
    InsertOneBoolArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let InsertOneBoolArgs { b } = __args;
        __callback(__identity, __addr, __status, b);
    })
}

#[allow(unused)]
pub fn once_on_insert_one_bool(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &bool) + Send + 'static,
) -> ReducerCallbackId<InsertOneBoolArgs> {
    InsertOneBoolArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let InsertOneBoolArgs { b } = __args;
        __callback(__identity, __addr, __status, b);
    })
}

#[allow(unused)]
pub fn remove_on_insert_one_bool(id: ReducerCallbackId<InsertOneBoolArgs>) {
    InsertOneBoolArgs::remove_on_reducer(id);
}