#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "derive")]
pub extern crate diesel_crud_trait_derive;
use diesel::backend::Backend;
use diesel::insertable::CanInsertInSingleQuery;
use diesel::internal::table_macro::AliasAliasAppearsInFromClause;
use diesel::internal::table_macro::AliasAppearsInFromClause;
use diesel::internal::table_macro::FieldAliasMapperAssociatedTypesDisjointnessTrick;
use diesel::internal::table_macro::NoFromClause;
use diesel::internal::table_macro::StaticQueryFragment;
use diesel::query_builder::{IntoUpdateTarget, UpdateStatement};
use diesel::query_source::Alias;
use diesel::query_source::AliasSource;
use diesel::query_source::AppearsInFromClause;
use diesel::sql_types::HasSqlType;
use diesel::Column;
use diesel::EqAll;
use diesel::JoinTo;
use diesel::Queryable;
use diesel::Selectable;
use diesel::{
    associations::HasTable,
    deserialize::FromSql,
    dsl::Desc,
    expression::{is_aggregate::No, MixedAggregates, NonAggregate, ValidGrouping},
    query_builder::{AsQuery, QueryFragment, QueryId},
    query_dsl::{
        methods::{FilterDsl, FindDsl, LimitDsl, LoadQuery, OrderDsl, SelectDsl},
        UpdateAndFetchResults,
    },
    sql_types::SingleValue,
    sqlite::Sqlite,
    AsChangeset, Identifiable, Insertable, QueryDsl, QuerySource, RunQueryDsl, SaveChangesDsl,
    SelectableExpression, SqliteConnection, Table,
};
use error::ErrorCrud;
use std::fmt::Debug;
pub mod error;
use diesel::internal::derives::insertable::UndecoratedInsertRecord;
use diesel::query_builder::DeleteStatement;
use diesel::query_dsl::methods::ExecuteDsl;

/// the trait CrudAble doc todo
pub trait CrudAble
where
    Self: 'static
        + Sized
        + HasTable
        + Queryable<<Self::Table as AsQuery>::SqlType, Sqlite>
        + Selectable<Sqlite>
        + Insertable<Self::Table>
        + UndecoratedInsertRecord<Self::Table>
        + AsChangeset
        + Clone
        + Debug,
    &'static Self: Identifiable + Insertable<Self::Table> + AsChangeset,
    Self::Table: Debug
        + Clone
        + Copy
        + QueryId
        + Default
        + QuerySource
        + QueryFragment<Sqlite>
        + StaticQueryFragment
        + AsQuery
        + Table
        + HasTable
        + IntoUpdateTarget
        + AppearsInFromClause<Self::Table>, // + AliasAppearsInFromClause<Alias<Self::Table>, Self::Table>
    // + AliasAliasAppearsInFromClause<Self::Table, Alias<Self::Table>, Alias<Self::Table>>
    // + AppearsInFromClause<Alias<Self::Table>>, // + FieldAliasMapperAssociatedTypesDisjointnessTrick<Self::Table, Self::Table, Self::Table>,
    NoFromClause: AppearsInFromClause<Self::Table>,
{
    // /// method create of CRUD. correspond to POST /elements for REST API.
    // fn create_with_check(
    //     &'static self,
    //     conn: &mut SqliteConnection,
    //     check: Option<Box<dyn FnMut(&Self, &mut SqliteConnection) -> Result<(), ErrorCrud>>>,
    // ) -> Result<i32, ErrorCrud> {
    //     // check data if closure was given for it, return error of check if it failed or continue.
    //     if let Some(mut checking) = check {
    //         checking(&self, conn)?;
    //     }
    //     // insert the new data and return the id
    //     Ok(diesel::insert_into(Self::table())
    //         .values(self)
    //         .returning(Self::table().primary_key())
    //         .get_result(conn)?)
    // }
    // /// wrapper for create_with_check without providing a check.
    // /// It exist to make a simpler usage.
    // fn create(&'static self, conn: &mut SqliteConnection) -> Result<i32, ErrorCrud> {
    //     self.create_with_check(conn, None)
    // }
    // /// method read of CRUD. correspond to GET /elements/id for REST API.
    // fn read(rowid: i32, conn: &mut SqliteConnection) -> Result<Self, ErrorCrud> {
    //     Ok(QueryDsl::find(Self::table(), rowid).first::<Self>(conn)?)
    // }
    // /// method update of CRUD. correspond to PUT or PATCH /elements/id for REST API.
    // /// The model needs to have a rowid
    // fn update_with_check(
    //     &'static self,
    //     conn: &mut SqliteConnection,
    //     check: Option<Box<dyn FnMut(&Self, &mut SqliteConnection) -> Result<(), ErrorCrud>>>,
    // ) -> Result<Self, ErrorCrud> {
    //     // check data if closure was given for it before updating.
    //     if let Some(mut checking) = check {
    //         checking(self, conn)?;
    //     }
    //     Ok(SaveChangesDsl::save_changes::<Self>(self, conn)?)
    // }
    // /// wrapper for update_with_check without providing a check.
    // /// It exist to make a simpler usage.
    // fn update(&'static self, conn: &mut SqliteConnection) -> Result<Self, ErrorCrud> {
    //     self.update_with_check(conn, None)
    // }
    // #[cfg(feature = "methods_on_all")]
    // /// return all rows of table. correspont to GET /elements
    // fn list_all(conn: &mut SqliteConnection) -> Result<Vec<Self>, ErrorCrud> {
    //     Ok(Self::table().load(conn)?)
    // }
    // #[cfg(feature = "methods_on_all")]
    // /// empty the table. correspont to DELETE /elements
    // fn delete_all(conn: &mut SqliteConnection) -> Result<usize, ErrorCrud> {
    //     Ok(diesel::delete(Self::table()).execute(conn)?)
    // }
}
