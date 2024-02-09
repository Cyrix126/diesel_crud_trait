# diesel_crud_trait

library to implement [CRUD](https://en.wikipedia.org/wiki/Create,_read,_update_and_delete) operations on models of diesel in a generic way.

## Disclaimer WIP

This library is a Work In Progress.
Everything desscribed in this README are not ready or tested. 

## Question of the design

Is it better to allow closure to check/modify data in the trait or to let the dev user 

## Objectives

The idea is to make a way to not have to implement CRUD methods to models everytime we write a library making use of diesel.
This library will also bring some functionnality, like optionnaly passing closure(s) with usefull parameters to check or modify data before/after the operation (depending on the method). 
The advantage to use closures over functions is that it can use the same connection and keep a litteral trait (instead of trait\<T\> where T would be the args to pass as a tuple).

### Features:

#### Derive

Brig the derive macro to add the trait easly on your model.

#### Batch

An implementation for Vec<T> where T: CrudAble allows to use CRUD operation in a batch way.

#### List

Add a outside of CRUD method (but still in the same trait) to list all element at once.

#### Delete

Add a outside of CRUD method (but still in the same trait) to delete all element at once.

A Restful API could be generated from thoses methods, or they could be used directly in an app.

## Fonctionnality

- [ ] generic backend (sqlite only for now, mysql and postgres are planned).
- [ ] defined methods for CRUD operations
  - [x] create
    - [x] possible closure to check provided data before applying operation. 
    - [x] possible closure to modify provided data before applying operation.
  - [ ] read
    - [ ] possible closure to modify returned data.
  - [ ] update
    - [ ] possible closure to check provided data before applying operation. 
    - [ ] possible closure to modify provided data before applying operation.
  - [ ] delete
    - [ ] possible closure to check provided data before applying operation. 
- [ ] derive trait
  - [ ] retrieve table and column id of model automaticly.
  - [ ] disable method (make them return an error)
- [ ] add non CRUD usefull operations
  - [ ] list all
    - [ ] possible closure to modify returned data.
  - [ ] delete all
- [ ] implementation for Vec\<T\> for using multiples elements.

## Usage

You would apply the derive macro to your model struct.

```rust,ignore
#[derive(Insertable, QueryAble, CrudAble)]
  struct Model {
  #[diesel(skip_insertion)] // so that you can use Insertable and QueryAble on this model. You need diesel master on at least commit [a0aee2b](https://github.com/Ten0/diesel/commit/a0aee2b3e1b2d6b2246869cdc2fdac7f6dc4bcd1)
    rowid: i32,
    name: String
  }
```

And then you use the methods of the trait directly. 
```rust,ignore
let value = Model {
  name: String::from("whatever")
  ..Default::default()
};
value.create(conn, None, None);
```
You can check the data before the operation occurs.
You can pass a Boxed FnOnce closure which takes the value and the connection and return a Result<(), ErrorCrude>.

```rust,ignore
let check = Box::new(|value: &Model, _conn: &mut SqliteConnection| {
  if value.name.contains("bad word") {
    Err(ErrorCrud::InvalidData(&value.name))
  } else {
    Ok()
  }
});
fn check_data(value: &Model) -> Result<(), ErrorCrude> {
}
value.create(conn, Some(check) None);
```

## Licence

GPLv3.
