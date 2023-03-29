extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{Data, DataStruct, DeriveInput, Fields, Ident};

fn impl_dynamic_update(ast: DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let field_name = fields.iter().map(|field| &field.ident);
    let field_name2 = field_name.clone();
    let field_name3 = field_name.clone();
    let field_name4 = field_name.clone();
    let field_name5 = field_name.clone();

    let field_type = fields.iter().map(|field| &field.ty);

    let enum_name = syn::parse_str::<Ident>(&format!("{}_field", name.to_string())).unwrap();

    TokenStream::from(quote! {
        //#ast
        fn type_id<T: 'static + ?Sized>(_: &T) -> TypeId {
            TypeId::of::<T>()
        }
        #[allow(non_camel_case_types)]
        pub enum #enum_name {
            #(
                #field_name(#field_type),
            )*
        }

        impl #name{
            pub fn get(&self, field: impl ToString) -> Result<std::any::TypeId> {
                match field.to_string().as_str() {
                    #(
                        stringify!(#field_name2) => Ok(type_id(&self.#field_name3)),
                    )*
                    _ => panic!("Unknown field"),
                }
            }

           pub fn update(self, field: impl ToString, val: &str) -> Result<Self> {
               match field.to_string().as_str() {
                    #(
                        stringify!(#field_name4) => {
                            let mut new = self.clone();
                            new.#field_name5 = val.parse().unwrap();
                            Ok(new)
                        },
                     )*
                    _ => panic!("Unknown field"),
               }
           }
        }
    })
}

#[proc_macro_derive(dynamic_update)]
pub fn dynamic_update(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast = syn::parse_macro_input!(input as DeriveInput);

    // Build and return the generated impl
    impl_dynamic_update(ast)
}
