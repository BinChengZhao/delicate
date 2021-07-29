use super::prelude::*;
// TDD.
// operates!("task" , 1 , )
// operates!( name, session(user-id, user-name) , operation_type, values, column_comment);
// operates!( name, session , operation_type, values);

// TODO: Just demo .
#[macro_export]
macro_rules! operates {
    (target: $target:expr,  session: $session:expr , operation_type: $operation_type:expr , $values:expr ) => {
        // $crate::event!(target: $target, parent: $parent, $crate::Level::INFO, { $($field)* }, $($arg)*)
    };
}
