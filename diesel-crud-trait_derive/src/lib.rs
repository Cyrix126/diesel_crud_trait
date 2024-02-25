#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
use proc_macro2::TokenStream;
use quote::quote;

fn create_with_check() -> TokenStream {
    quote! {
        fn create_with_check(
            &'static self,
            conn: &mut diesel::SqliteConnection,
            check: Option<
                Box<
                    dyn FnMut(
                        &Self,
                        &mut diesel::SqliteConnection,
                    ) -> Result<(), diesel_crud_trait::error::ErrorCrud>,
                >,
            >,
        ) -> Result<i32, diesel_crud_trait::error::ErrorCrud> {
            // check data if closure was given for it, return error of check if it failed or continue.
            if let Some(mut checking) = check {
                checking(&self, conn)?;
            }
            // insert the new data and return the id
            Ok(diesel::RunQueryDsl::get_result(
                diesel::insert_into(<Self as diesel::associations::HasTable>::table())
                    .values(self)
                    .returning(diesel::Table::primary_key(
                        &<Self as diesel::associations::HasTable>::table(),
                    )),
                conn,
            )?)
        }
    }
}
fn create() -> TokenStream {
    quote! {
            fn create(
                &'static self,
                conn: &mut diesel::sqlite::SqliteConnection,
            ) -> Result<i32, diesel_crud_trait::error::ErrorCrud> {
                self.create_with_check(conn, None)
            }
    }
}
fn read_with_check() -> TokenStream {
    quote! {
        fn read_with_check(
            rowid: i32,
            conn: &mut diesel::sqlite::SqliteConnection,
            check: Option<
                Box<
                    dyn FnMut(
                        i32,
                        &mut diesel::SqliteConnection,
                    ) -> Result<(), diesel_crud_trait::error::ErrorCrud>,
                >,
            >,
        ) -> Result<Self, diesel_crud_trait::error::ErrorCrud> {
            if let Some(mut checking) = check {
                checking(rowid, conn)?;
            }
            Ok(diesel::RunQueryDsl::first(
                diesel::query_dsl::methods::FindDsl::find(
                    <Self as diesel::associations::HasTable>::table(),
                    rowid,
                ),
                conn,
            )?)
        }
    }
}
fn read() -> TokenStream {
    quote! {
        fn read(
            rowid: i32,
            conn: &mut diesel::sqlite::SqliteConnection,
        ) -> Result<Self, diesel_crud_trait::error::ErrorCrud> {
            Ok(diesel::RunQueryDsl::first(
                diesel::query_dsl::methods::FindDsl::find(
                    <Self as diesel::associations::HasTable>::table(),
                    rowid,
                ),
                conn,
            )?)
        }
    }
}
fn update_with_check() -> TokenStream {
    quote! {
        fn update_with_check(
            &'static self,
            conn: &mut diesel::sqlite::SqliteConnection,
            check: Option<
                Box<
                    dyn FnMut(
                        &Self,
                        &mut diesel::sqlite::SqliteConnection,
                    ) -> Result<(), diesel_crud_trait::error::ErrorCrud>,
                >,
            >,
        ) -> Result<Self, diesel_crud_trait::error::ErrorCrud> {
            // check data if closure was given for it before updating.
            if let Some(mut checking) = check {
                checking(self, conn)?;
            }
            Ok(diesel::SaveChangesDsl::save_changes::<Self>(self, conn)?)
        }
    }
}
fn update() -> TokenStream {
    quote! {
        fn update(
            &'static self,
            conn: &mut diesel::sqlite::SqliteConnection,
        ) -> Result<Self, diesel_crud_trait::error::ErrorCrud> {
            self.update_with_check(conn, None)
        }
    }
}
fn delete_with_check() -> TokenStream {
    quote! {
        fn delete_with_check(
            rowid: i32,
            conn: &mut diesel::sqlite::SqliteConnection,
            check: Option<
                Box<
                    dyn FnMut(
                        i32,
                        &mut diesel::sqlite::SqliteConnection,
                    ) -> Result<(), diesel_crud_trait::error::ErrorCrud>,
                >,
            >,
        ) -> Result<(), diesel_crud_trait::error::ErrorCrud> {
            if let Some(mut checking) = check {
                checking(rowid, conn)?;
            }
            diesel::RunQueryDsl::execute(
                diesel::delete(<Self as diesel::associations::HasTable>::table()).filter(
                    diesel::ExpressionMethods::eq(
                        diesel::Table::primary_key(&<Self as diesel::associations::HasTable>::table()),
                        rowid,
                    ),
                ),
                conn,
            )?;
            Ok(())
        }
    }
}
fn delete() -> TokenStream {
    quote! {
        fn delete(
            rowid: i32,
            conn: &mut diesel::sqlite::SqliteConnection,
        ) -> Result<(), diesel_crud_trait::error::ErrorCrud> {
            Self::delete_with_check(rowid, conn, None)
        }
    }
}

use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature="all_methods")] {
        fn list_all() -> TokenStream {
            quote! {
                fn list_all(
                    conn: &mut diesel::sqlite::SqliteConnection,
                ) -> Result<Vec<Self>, diesel_crud_trait::error::ErrorCrud> {
                    Ok(diesel::RunQueryDsl::load(
                        <Self as diesel::associations::HasTable>::table(),
                        conn,
                    )?)
                }
            }
        }
        fn delete_all() -> TokenStream {
            quote! {
                fn delete_all(
                    conn: &mut diesel::sqlite::SqliteConnection,
                ) -> Result<usize, diesel_crud_trait::error::ErrorCrud> {
                    Ok(diesel::RunQueryDsl::execute(
                        diesel::delete(<Self as diesel::associations::HasTable>::table()),
                        conn,
                    )?)
                }
            }
        }
    }
}
#[doc = include_str!("../doc/derive.md")]
#[proc_macro_derive(CrudAble)]
pub fn crudable_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_crudable(&ast)
}

fn impl_crudable(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    // get name of model struct

    let name_model = &ast.ident;
    // prepare methods
    let create_with_check = create_with_check();
    let create = create();
    let read_with_check = read_with_check();
    let read = read();
    let update_with_check = update_with_check();
    let update = update();
    let delete_with_check = delete_with_check();
    let delete = delete();
    let content_method = quote! {
        #create_with_check
        #create
        #read_with_check
        #read
        #update_with_check
        #update
        #delete_with_check
        #delete
    };
    cfg_if! {
        if #[cfg(feature="all_methods")] {
        let list_all = list_all();
        let delete_all = delete_all();
        let content_method = quote! {
            #content_method
            #list_all
            #delete_all
            }
        }
    }
    quote! {
        impl #name_model {
        #content_method
        }
    }
    .into()
}
