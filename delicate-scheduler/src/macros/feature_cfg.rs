//! This macro module provides the declaration macros used for the conditional compilation of lib.

macro_rules! cfg_mysql_support {
    ($($item:item)*) => {
        $(
            // As long as the features contains mysql-support, it will compile.
            #[cfg(DB_MYSQL)]
            $item

        )*
    }
}

macro_rules! cfg_postgres_support {
    ($($item:item)*) => {
        $(
            // As long as the features contains postgres-support, it will compile.
            #[cfg(DB_POSTGRES)]
            $item

        )*
    }
}

#[allow(unused_macros)]
macro_rules! cfg_auth_casbin {
    ($($item:item)*) => {
        $(
            // As long as the features contains casbin-auth, it will compile.
            #[cfg(AUTH_CASBIN)]
            #[allow(dead_code)]
            $item

        )*
    }
}
