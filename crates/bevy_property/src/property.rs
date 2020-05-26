use crate::Properties;
use serde::Serialize;
use std::{
    any::Any,
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    hash::Hash,
};

pub trait Property: erased_serde::Serialize + Send + Sync + Any + AsProperties + 'static {
    fn any(&self) -> &dyn Any;
    fn any_mut(&mut self) -> &mut dyn Any;
    fn clone_prop(&self) -> Box<dyn Property>;
    fn set(&mut self, value: &dyn Property);
    fn apply(&mut self, value: &dyn Property);
}

erased_serde::serialize_trait_object!(Property);

pub trait AsProperties {
    fn as_properties(&self) -> Option<&dyn Properties>;
}

pub trait PropertyVal {
    fn val<T: 'static>(&self) -> Option<&T>;
    fn set_val<T: 'static>(&mut self, value: T);
}

impl PropertyVal for dyn Property {
    #[inline]
    fn val<T: 'static>(&self) -> Option<&T> {
        self.any().downcast_ref::<T>()
    }

    #[inline]
    fn set_val<T: 'static>(&mut self, value: T) {
        if let Some(prop) = self.any_mut().downcast_mut::<T>() {
            *prop = value;
        } else {
            panic!("prop value is not {}", std::any::type_name::<T>());
        }
    }
}

// used by impl_property
#[allow(unused_macros)]
macro_rules! as_item { ($i:item) => {$i} }

#[macro_export]
macro_rules! impl_property {
    ($ty:ident) => {
        impl Property for $ty {
            #[inline]
            fn any(&self) -> &dyn Any {
                self
            }
        
            #[inline]
            fn any_mut(&mut self) -> &mut dyn Any {
                self
            }
        
            #[inline]
            fn clone_prop(&self) -> Box<dyn Property> {
                Box::new(self.clone())
            }
        
            #[inline]
            fn apply(&mut self, value: &dyn Property) {
                self.set(value);
            }
        
            fn set(&mut self, value: &dyn Property) {
                let value = value.any();
                if let Some(prop) = value.downcast_ref::<Self>() {
                    *self = prop.clone();
                } else {
                    panic!("prop value is not {}", std::any::type_name::<Self>());
                }
            }
        }
        
        impl AsProperties for $ty {
            fn as_properties(&self) -> Option<&dyn Properties> {
                None
            }
        }
    };
    (@$trait_:ident [$($args:ident,)*] where [$($preds:tt)+]) => {
        impl_property! {
            @as_item
            impl<$($args),*> Property for $trait_<$($args),*> where $($args: ::std::any::Any + 'static,)*
            $($preds)* {
                #[inline]
                fn any(&self) -> &dyn Any {
                    self
                }
            
                #[inline]
                fn any_mut(&mut self) -> &mut dyn Any {
                    self
                }
            
                #[inline]
                fn clone_prop(&self) -> Box<dyn Property> {
                    Box::new(self.clone())
                }
            
                #[inline]
                fn apply(&mut self, value: &dyn Property) {
                    self.set(value);
                }
            
                fn set(&mut self, value: &dyn Property) {
                    let value = value.any();
                    if let Some(prop) = value.downcast_ref::<Self>() {
                        *self = prop.clone();
                    } else {
                        panic!("prop value is not {}", std::any::type_name::<Self>());
                    }
                }
            }
        }
    };
    (@as_item $i:item) => { $i };

    (
        $trait_:ident < $($args:ident),* $(,)* >
        where $($preds:tt)+
    ) => {
        impl_property! { @$trait_ [$($args,)*] where [$($preds)*] }
    };
}