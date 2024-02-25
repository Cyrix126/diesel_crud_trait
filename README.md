# diesel_crud_trait

library to implement [CRUD](https://en.wikipedia.org/wiki/Create,_read,_update_and_delete) operations on models of diesel in a generic way.

The goal was to make a generic trait for this, but because it appears to be very complex to put the good trait bounds, I have decided instead to make a simple impl on the model on which the derive macro is applied.

## Disclaimer WIP

This library is a Work In Progress.
Everything desscribed in this README are not ready or tested. 

## Objectives

The idea is to make a way to not have to implement CRUD methods to models everytime we write a library making use of diesel.
The use of this library must be easy for the dev user.

This library will bring some functionnality, like optionnaly passing closure with usefull parameter to check input data.
Simpler method can be called if no check is needed.

A Restful API could be generated from thoses methods, or they could be used directly in an app.

### Features:

#### Derive

Bring the derive macro to add the methods easly on your model.

#### Batch

An implementation for Vec<T> where T: CrudAble allows to use CRUD operation in a batch way.

#### all_methods

Add methods outside of CRUD method to list or delete all element at once.

## TODO

### Fonctionnality

- [x] defined methods for CRUD operations
  - [x] create
  - [x] read
  - [x] update
  - [x] delete
  - [x] possible closure to check provided data before applying operation. 
- [x] add non CRUD usefull operations (all_methods feature)
  - [x] list all
  - [x] delete all
- [ ] implementation for Vec\<T\> for using multiples elements (batch feature)

### Other:

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

And then you use the methods of the directly. 
```rust,ignore
let value = Model {
  name: String::from("whatever")
  ..Default::default()
};
value.create(conn);
```
You can check the data before the operation occurs.
You can pass a Boxed FnMut closure which takes the value and the connection and return a Result<(), ErrorCrude>.

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
