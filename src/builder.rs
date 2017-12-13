#![allow(dead_code)]

pub enum Parameter<T> {
    Required,
    Optional(T),
    Provided(T),
}

impl <T: Clone> Parameter<T> {
    pub fn resolve(&self) -> T {
        match self {
            &Parameter::Required => { panic!(); },
            &Parameter::Optional(ref v) => v.clone(),
            &Parameter::Provided(ref v) => v.clone(),
        }
    }
}

#[macro_export]
macro_rules! builder {
    ( $builder_name:ident => $struct_name:ident {
        $( $field_name:ident : $field_type:ty = $field_default:expr, )*
    } ) => {
        pub struct $struct_name {
            $( $field_name: $field_type, )*
        }

        pub struct $builder_name {
            $( $field_name: Parameter<$field_type>, )*
        }

        impl $builder_name {
            pub fn new() -> $builder_name {
                $builder_name {
                    $( $field_name: $field_default, )*
                }
            }

            $(
                pub fn $field_name(&mut self, v: $field_type) -> &mut Self {
                    match self.$field_name {
                        Parameter::Provided(_) => { panic!(); },
                        _ => { self.$field_name = Provided(v); },
                    }
                    self
                }
            )*

            pub fn build(&self) -> $struct_name {
                $(
                    let $field_name = self.$field_name.resolve();
                )*

                $struct_name {
                    $( $field_name, )*
                }
            }
        }
    };
}
