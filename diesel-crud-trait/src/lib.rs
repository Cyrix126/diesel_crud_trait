#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
use diesel::{
    associations::HasTable, deserialize::FromSql, dsl::Desc, expression::{NonAggregate, ValidGrouping, MixedAggregates, is_aggregate::No}, query_builder::{AsQuery, QueryFragment, QueryId}, query_dsl::{methods::{FilterDsl, FindDsl, LimitDsl, LoadQuery, OrderDsl, SelectDsl}, UpdateAndFetchResults}, sql_types::SingleValue, sqlite::Sqlite, AsChangeset, Expression, ExpressionMethods, Identifiable, Insertable, QueryDsl, QuerySource, RunQueryDsl, SaveChangesDsl, SelectableExpression, SqliteConnection, Table
};
use diesel::query_builder::{IntoUpdateTarget, UpdateStatement};
use diesel::EqAll;
use error::ErrorCrud;
use diesel::sql_types::SqlType;
use diesel::sql_types::HasSqlType;
use diesel::insertable::CanInsertInSingleQuery;
mod error;

/// the trait CrudAble doc todo
pub trait CrudAble  
where
    Self: Insertable<Self::Table> + 'static + Sized + HasTable,
    &'static Self: Insertable<Self::Table> + Identifiable + SaveChangesDsl<SqliteConnection> + AsChangeset + IntoUpdateTarget + HasTable,
    <Self::Table as QuerySource>::FromClause: QueryFragment<Sqlite>,
    <&'static Self as Insertable<Self::Table>>::Values: QueryFragment<Sqlite> + QueryId + CanInsertInSingleQuery<Sqlite>,
    Self::Table: QueryId + FindDsl<i32> + Identifiable,
    <Self::Table as AsQuery>::Query: SelectDsl<Self::IdColumn> +  FilterDsl<i32>,
    <<Self::Table as AsQuery>::Query as SelectDsl<Self::IdColumn>>::Output: QueryDsl + OrderDsl<Desc<Self::IdColumn>>,
    <<<Self::Table as AsQuery>::Query as SelectDsl<Self::IdColumn>>::Output as OrderDsl<Desc<Self::IdColumn>>>::Output: AsQuery + LimitDsl + RunQueryDsl<SqliteConnection>,
<<<<Self::Table as AsQuery>::Query as SelectDsl<Self::IdColumn>>::Output as OrderDsl<Desc<Self::IdColumn>>>::Output as LimitDsl>::Output: RunQueryDsl<SqliteConnection> + LoadQuery<'static,SqliteConnection, i32>,
<Self::Table as FindDsl<i32>>::Output: RunQueryDsl<SqliteConnection> + LimitDsl + HasTable + IntoUpdateTarget,
<<Self::Table as FindDsl<i32>>::Output as LimitDsl>::Output: LoadQuery<'static, SqliteConnection, Self>,
<<&'static Self as HasTable>::Table as AsQuery>::Query: FilterDsl<i32>,
<<<&'static Self as HasTable>::Table as AsQuery>::Query as FilterDsl<i32>>::Output: LoadQuery<'static, SqliteConnection, Self>,
    // UpdateStatement<<&'static Self as HasTable>::Table, <&'static Self as IntoUpdateTarget>::WhereClause, <&'static Self as AsChangeset>::Changeset>: QueryFragment<Sqlite>,
    // UpdateStatement<<<Self::Table as FindDsl<i32>>::Output as HasTable>::Table, <<Self::Table as FindDsl<i32>>::Output as IntoUpdateTarget>::WhereClause, <&'static Self as AsChangeset>::Changeset>: LoadQuery<'static, SqliteConnection, Self>,
    // UpdateStatement<<<<Self as HasTable>::Table as FindDsl<i32>>::Output as HasTable>::Table, <<<Self as HasTable>::Table as FindDsl<i32>>::Output as IntoUpdateTarget>::WhereClause, <&'static Self as AsChangeset>::Changeset>: AsQuery, 
<<&'static Self as HasTable>::Table as Table>::PrimaryKey: EqAll<<&'static Self as Identifiable>::Id>,
<<<Self as HasTable>::Table as Table>::PrimaryKey as ValidGrouping<()>>::IsAggregate: MixedAggregates<diesel::expression::is_aggregate::No>,
<<<&'static Self as HasTable>::Table as Table>::AllColumns as ValidGrouping<()>>::IsAggregate: MixedAggregates<diesel::expression::is_aggregate::No>,
<<<<&'static Self as HasTable>::Table as Table>::AllColumns as ValidGrouping<()>>::IsAggregate as MixedAggregates<diesel::expression::is_aggregate::No>>::Output: MixedAggregates<No>,
<<&'static Self as HasTable>::Table as AsQuery>::Query: FilterDsl<<<<&'static Self as HasTable>::Table as Table>::PrimaryKey as EqAll<<&'static Self as Identifiable>::Id>>::Output>,
<<<&'static Self as HasTable>::Table as AsQuery>::Query as FilterDsl<<<<&'static Self as HasTable>::Table as Table>::PrimaryKey as EqAll<<&'static Self as Identifiable>::Id>>::Output>>::Output: LoadQuery<'static, SqliteConnection, Self>,
    <<Self::Table as Table>::PrimaryKey as diesel::Expression>::SqlType: SingleValue + QueryId,
    <Self::Table as Table>::PrimaryKey: QueryFragment<Sqlite> + QueryId + NonAggregate,
UpdateStatement<<<<Self as HasTable>::Table as FindDsl<i32>>::Output as HasTable>::Table, <<<Self as HasTable>::Table as FindDsl<i32>>::Output as IntoUpdateTarget>::WhereClause, <<<Self as HasTable>::Table as FindDsl<i32>>::Output as HasTable>::Table>: AsQuery,  UpdateStatement<<<<Self as HasTable>::Table as FindDsl<i32>>::Output as HasTable>::Table, <<<Self as HasTable>::Table as FindDsl<i32>>::Output as IntoUpdateTarget>::WhereClause, <&'static Self as AsChangeset>::Changeset>: AsQuery
{
    /// type of the column with primary key. like schemas::table_name::rowid
    /// will be determined automaticly by the derive trait
    // type IdColumn: ExpressionMethods + Expression + SelectableExpression<Self::Table> + NonAggregate;
    type IdColumn: SelectableExpression<Self> + ValidGrouping<()> + QueryId + QueryFragment<Sqlite> + NonAggregate;
    /// type of the table. like schemas::table_name::table
    const TABLE: Self::Table;
    //  = Self::table();
    /// type of the column with primary key. like schemas::table_name::rowid
    /// will be determined automaticly by the derive trait
    const ID_COLUMN: <Self::Table as Table>::PrimaryKey;
    // = Self::table().primary_key();
    
    /// method create of CRUD. correspond to POST /elements for REST API.
    fn create(&'static self, conn: &mut SqliteConnection, check: Option<Box<dyn FnMut(&Self, &mut SqliteConnection) -> Result<(), ErrorCrud>>>, alter: Option<Box<dyn FnMut(&Self, &mut SqliteConnection) -> Result<&'static 
        Self, ErrorCrud>>>) -> Result<i32, ErrorCrud> where 
        <Self::IdColumn as diesel::Expression>::SqlType: SingleValue, 
     i32: FromSql<<<Self::Table as Table>::PrimaryKey as diesel::Expression>::SqlType, Sqlite>,
    Sqlite: HasSqlType<<<Self::Table as Table>::PrimaryKey as diesel::Expression>::SqlType>,
    {
        // check data if closure was given for it, return error of check if it failed or continue.
        if let Some(mut checking) = check {
            checking(&self, conn)?;
        }
        // alter the data that will be inserted if closure was given for it, return error of closure if it failed or use the new altered value.
        let value = if let Some(mut altering) = alter {
            &altering(&self, conn)?
        } else {
            self
        };
        // insert the new data and return the id
        Ok(diesel::insert_into(Self::TABLE).values(value).returning(Self::ID_COLUMN).get_result(conn)?)
    }
    /// method read of CRUD. correspond to GET /elements/id for REST API.
    fn read(rowid: i32, conn: &mut SqliteConnection, alter: Option<Box<dyn FnMut(&mut Self, &mut SqliteConnection) -> Result<(), ErrorCrud>>>) -> Result<Self, ErrorCrud> {

        let mut value: Self = QueryDsl::find(Self::TABLE, rowid).first::<Self>(conn)?;
        if let Some(mut altering) = alter {
            altering(&mut value, conn)?
        };         Ok(value)
    }
    /// method update of CRUD. correspond to PUT or PATCH /elements/id for REST API.
    /// we consider the model could perhaps not have the id.
    /// we return the modified row
    fn update(&'static self, rowid: i32, conn: &mut SqliteConnection, check: Option<Box<dyn FnMut(&Self, &mut SqliteConnection) -> Result<(), ErrorCrud>>>, alter: Option<Box<dyn FnMut(&mut Self, &mut SqliteConnection) -> Result<(), ErrorCrud>>>) -> Result<Self, ErrorCrud> 
    where 
    UpdateStatement<<&'static Self as HasTable>::Table, <&'static Self as IntoUpdateTarget>::WhereClause, <&'static Self as AsChangeset>::Changeset>: QueryFragment<Sqlite>,
    SqliteConnection: UpdateAndFetchResults<&'static Self, Self>
    
{
        // check data if closure was given for it before updating.
        if let Some(mut checking) = check {
            checking(self, conn)?;
        }
        let mut value = SaveChangesDsl::save_changes::<Self>(self, conn)?;
        
        // alter the data that will be inserted if closure was given for it.
        if let Some(mut altering) = alter {
            altering(&mut value, conn)?;
        }
        Ok(value)
 }
}
