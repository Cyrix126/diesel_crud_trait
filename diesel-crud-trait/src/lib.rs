#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
use diesel::{
    dsl::Desc,
    expression::NonAggregate,
    query_builder::{AsQuery, QueryFragment, QueryId},
    query_dsl::methods::{LimitDsl, LoadQuery, OrderDsl, SelectDsl},
    sqlite::Sqlite,
    Expression, ExpressionMethods, Insertable, QueryDsl, QuerySource, RunQueryDsl,
    SelectableExpression, SqliteConnection, Table,
};
use error::ErrorCrud;
mod error;
/// the trait CrudAble doc todo
pub trait CrudAble 
where
    // insert
    Self: Insertable<<Self as CrudAble>::Table> + 'static,
    &'static Self: Insertable<<Self as CrudAble>::Table>,
    <<Self as CrudAble>::Table as QuerySource>::FromClause: QueryFragment<Sqlite>,
    <&'static Self as Insertable<<Self as CrudAble>::Table>>::Values: QueryFragment<Sqlite>,
    <Self as CrudAble>::Table: QueryId,
    <&'static Self as Insertable<<Self as CrudAble>::Table>>::Values: QueryId,
    <&'static Self as Insertable<<Self as CrudAble>::Table>>::Values:
        diesel::insertable::CanInsertInSingleQuery<Sqlite>,
    // select
    <<Self as CrudAble>::Table as AsQuery>::Query: SelectDsl<Self::IdColumn>,
    // order
    <<<Self as CrudAble>::Table as AsQuery>::Query as SelectDsl<<Self as CrudAble>::IdColumn>>::Output:
        OrderDsl<diesel::dsl::Desc<<Self as CrudAble>::IdColumn>>,
    <<<Self as CrudAble>::Table as AsQuery>::Query as SelectDsl<<Self as CrudAble>::IdColumn>>::Output:
        QueryDsl,
    // first
    <<<<Self as CrudAble>::Table as AsQuery>::Query as SelectDsl<<Self as CrudAble>::IdColumn>>::Output as OrderDsl<Desc<<Self as CrudAble>::IdColumn>>>::Output: AsQuery,
    <<<<Self as CrudAble>::Table as AsQuery>::Query as SelectDsl<<Self as CrudAble>::IdColumn>>::Output as OrderDsl<Desc<<Self as CrudAble>::IdColumn>>>::Output: LimitDsl,
    <<<<Self as CrudAble>::Table as AsQuery>::Query as SelectDsl<<Self as CrudAble>::IdColumn>>::Output as OrderDsl<Desc<<Self as CrudAble>::IdColumn>>>::Output: RunQueryDsl<SqliteConnection>,
<<<<<Self as CrudAble>::Table as AsQuery>::Query as SelectDsl<<Self as CrudAble>::IdColumn>>::Output as OrderDsl<Desc<<Self as CrudAble>::IdColumn>>>::Output as LimitDsl>::Output: RunQueryDsl<SqliteConnection>,
<<<<<Self as CrudAble>::Table as AsQuery>::Query as SelectDsl<<Self as CrudAble>::IdColumn>>::Output as OrderDsl<Desc<<Self as CrudAble>::IdColumn>>>::Output as LimitDsl>::Output: LoadQuery<'static, SqliteConnection, i32>,
{
    /// type of the table. like schemas::table_name::table
    /// will be determined automaticly by the derive trait
    type Table: Table;
    /// type of the column with primary key. like schemas::table_name::rowid
    /// will be determined automaticly by the derive trait
    type IdColumn: ExpressionMethods + Expression + SelectableExpression<Self::Table> + NonAggregate;
    /// type of the table. like schemas::table_name::table
    /// will be determined automaticly by the derive trait
    const TABLE: Self::Table;
    /// type of the column with primary key. like schemas::table_name::rowid
    /// will be determined automaticly by the derive trait
    const ID_COLUMN: Self::IdColumn;
    /// method create of CRUD. correspond to POST /elements for REST API.
    fn create(&'static self, conn: &mut SqliteConnection, check: Option<Box<dyn FnOnce(&Self, &mut SqliteConnection) -> Result<(), ErrorCrud>>>, alter: Option<Box<dyn FnOnce(&Self, &mut SqliteConnection) -> Result<&'static 
        Self, ErrorCrud>>>) -> Result<i32, ErrorCrud> {
        // check data if closure was given for it, return error of check if it failed or continue.
        if let Some(checking) = check {
            checking(&self, conn)?;
        }
        // alter the data that will be inserted if closure was given for it, return error of closure if it failed or use the new altered value.
        let value = if let Some(altering) = alter {
            &altering(&self, conn)?
        } else {
            self
        };
        // insert the new data
        diesel::insert_into(Self::TABLE).values(value).execute(conn)?;
        // get the last row id and return it
        Ok(RunQueryDsl::first::<i32>(QueryDsl::order(QueryDsl::select(Self::TABLE, Self::ID_COLUMN), ExpressionMethods::desc(Self::ID_COLUMN)), conn)?)
    }
}
