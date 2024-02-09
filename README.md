# diesel_crud_trait

library to implement [CRUD](https://en.wikipedia.org/wiki/Create,_read,_update_and_delete) operations on models of diesel in a generic way.

## Disclaimer WIP

This library is a Work In Progress.
Everything desscribed in this README are not ready or tested. 

## Question of the design

Is it better to allow closure to check/modify data in the trait or to let the dev user 

## Objectives

The idea is to make a way to not have to implement CRUD methods to models everytime we write a library making use of diesel.
The use of this library must be easy ofr the dev user.

This library will bring some functionnality, like optionnaly passing closure with usefull parameter to check input data.
Simpler method can be called if no check is needed.
Maybe the derive macro could bring triggers pre/post operations.

A Restful API could be generated from thoses methods, or they could be used directly in an app.

### Features:

#### Derive

Brig the derive macro to add the trait easly on your model.

#### Batch

An implementation for Vec<T> where T: CrudAble allows to use CRUD operation in a batch way.

#### methods_on_all

Add a outside of CRUD method (but still in the same trait) to list all element at once.

Add a outside of CRUD method (but still in the same trait) to delete all element at once.

## TODO

### Fonctionnality

- [ ] generic backend (sqlite only for now, mysql and postgres are planned).
- [x] defined methods for CRUD operations
  - [x] create
  - [x] read
  - [x] update
  - [x] delete
  - [x] possible closure to check provided data before applying operation. 
- [x] add non CRUD usefull operations
  - [x] list all
  - [x] delete all
- [ ] derive trait
  - [ ] retrieve table and column id of model automaticly.
  - [ ] disable method (make them return an error)
  - [ ] triggers pre/post operation ?
- [ ] implementation for Vec\<T\> for using multiples elements.

### Other:

- [ ] find a better solution to manage bounds
- [ ] add tests
- [ ] add examples

## Usage

You would apply the derive macro to your model struct.

```rust,ignore
#[derive(Insertable, CrudAble)]
  struct Model {
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
value.create(conn);
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
value.create_after_check(conn, Some(check));
```

## Licence

GPLv3.
