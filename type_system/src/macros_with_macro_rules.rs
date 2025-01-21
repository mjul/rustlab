//! Examples of macros

mod foo {
    macro_rules! string_struct {
        ($struct_name:ident) => {
            #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
            #[doc= concat!(stringify!($struct_name), " value object.")]
            pub(crate) struct $struct_name(String);

            impl From<&str> for $struct_name {
                #[doc= concat!("Create [", stringify!($struct_name), "] from a value.")]
                fn from(value: &str) -> Self {
                    Self(String::from(value))
                }
            }
        };
    }

    // This works since the macro is in this module
    string_struct!(FooString);

    // Export the macro
    #[macro_export]
    macro_rules! exported_string_struct {
        ($struct_name:ident) => {
            #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
            #[doc= concat!(stringify!($struct_name), " value object.")]
            pub(crate) struct $struct_name(String);

            impl From<&str> for $struct_name {
                #[doc= concat!("Create [", stringify!($struct_name), "] from a value.")]
                fn from(value: &str) -> Self {
                    Self(String::from(value))
                }
            }
        };
    }
}

// here we cannot use string_struct!
// This does not compile:
// foo::string_struct!(BarString);

// This does compile, since there is a macro_export attribute on the macro
// note that the macro has been exported into the top level namespace
crate::exported_string_struct!(BarString);

fn bar() {
    println!("Hello {}", BarString::from("world").0);
}
