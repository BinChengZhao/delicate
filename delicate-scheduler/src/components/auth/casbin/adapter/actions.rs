use crate::prelude::*;

use super::models::{CasbinRule, NewCasbinRule};
use crate::db::schema;

use diesel::{
    self, BoolExpressionMethods, Connection as DieselConnection, ExpressionMethods, QueryDsl,
    RunQueryDsl,
};

pub fn remove_policy(
    conn: db::PoolConnection,
    pt: &str,
    rule: Vec<String>,
) -> AuthServiceResult<bool> {
    use schema::casbin_auth::dsl::*;

    let rule = normalize_casbin_auth(rule, 0);

    let filter = ptype
        .eq(pt)
        .and(v0.eq(&rule[0]))
        .and(v1.eq(&rule[1]))
        .and(v2.eq(&rule[2]))
        .and(v3.eq(&rule[3]))
        .and(v4.eq(&rule[4]))
        .and(v5.eq(&rule[5]));
    Ok(diesel::delete(casbin_auth.filter(filter))
        .execute(&conn)
        .map(|n| n == 1)?)
}

pub fn remove_policies(
    conn: db::PoolConnection,
    pt: &str,
    rules: Vec<Vec<String>>,
) -> AuthServiceResult<bool> {
    use schema::casbin_auth::dsl::*;

    Ok(conn.transaction::<_, DieselError, _>(|| {
        for rule in rules {
            let rule = normalize_casbin_auth(rule, 0);

            let filter = ptype
                .eq(pt)
                .and(v0.eq(&rule[0]))
                .and(v1.eq(&rule[1]))
                .and(v2.eq(&rule[2]))
                .and(v3.eq(&rule[3]))
                .and(v4.eq(&rule[4]))
                .and(v5.eq(&rule[5]));

            match diesel::delete(casbin_auth.filter(filter)).execute(&conn) {
                Ok(n) if n == 1 => continue,
                _ => return Err(DieselError::RollbackTransaction),
            }
        }

        Ok(true)
    })?)
}

pub fn remove_filtered_policy(
    conn: db::PoolConnection,
    pt: &str,
    field_index: usize,
    field_values: Vec<String>,
) -> AuthServiceResult<bool> {
    use schema::casbin_auth::dsl::*;

    let field_values = normalize_casbin_auth(field_values, field_index);

    let boxed_query = if field_index == 5 {
        diesel::delete(casbin_auth.filter(ptype.eq(pt).and(eq_empty!(&field_values[0], v5))))
            .into_boxed()
    } else if field_index == 4 {
        diesel::delete(
            casbin_auth.filter(
                ptype
                    .eq(pt)
                    .and(eq_empty!(&field_values[0], v4))
                    .and(eq_empty!(&field_values[1], v5)),
            ),
        )
        .into_boxed()
    } else if field_index == 3 {
        diesel::delete(
            casbin_auth.filter(
                ptype
                    .eq(pt)
                    .and(eq_empty!(&field_values[0], v3))
                    .and(eq_empty!(&field_values[1], v4))
                    .and(eq_empty!(&field_values[2], v5)),
            ),
        )
        .into_boxed()
    } else if field_index == 2 {
        diesel::delete(
            casbin_auth.filter(
                ptype
                    .eq(pt)
                    .and(eq_empty!(&field_values[0], v2))
                    .and(eq_empty!(&field_values[1], v3))
                    .and(eq_empty!(&field_values[2], v4))
                    .and(eq_empty!(&field_values[3], v5)),
            ),
        )
        .into_boxed()
    } else if field_index == 1 {
        diesel::delete(
            casbin_auth.filter(
                ptype
                    .eq(pt)
                    .and(eq_empty!(&field_values[0], v1))
                    .and(eq_empty!(&field_values[1], v2))
                    .and(eq_empty!(&field_values[2], v3))
                    .and(eq_empty!(&field_values[3], v4))
                    .and(eq_empty!(&field_values[4], v5)),
            ),
        )
        .into_boxed()
    } else {
        diesel::delete(
            casbin_auth.filter(
                ptype
                    .eq(pt)
                    .and(eq_empty!(&field_values[0], v0))
                    .and(eq_empty!(&field_values[1], v1))
                    .and(eq_empty!(&field_values[2], v2))
                    .and(eq_empty!(&field_values[3], v3))
                    .and(eq_empty!(&field_values[4], v4))
                    .and(eq_empty!(&field_values[5], v5)),
            ),
        )
        .into_boxed()
    };

    Ok(boxed_query.execute(&conn).map(|n| n >= 1)?)
}

pub(crate) fn clear_policy(conn: db::PoolConnection) -> AuthServiceResult<()> {
    use schema::casbin_auth::dsl::casbin_auth;
    Ok(diesel::delete(casbin_auth).execute(&conn).map(|_| ())?)
}

pub(crate) fn save_policy(
    conn: db::PoolConnection,
    rules: Vec<NewCasbinRule>,
) -> AuthServiceResult<()> {
    use schema::casbin_auth::dsl::casbin_auth;

    Ok(conn.transaction::<_, DieselError, _>(|| {
        if diesel::delete(casbin_auth).execute(&conn).is_err() {
            return Err(DieselError::RollbackTransaction);
        }

        Ok(diesel::insert_into(casbin_auth)
            .values(&rules)
            .execute(&*conn)
            .and_then(|n| {
                if n == rules.len() {
                    Ok(())
                } else {
                    Err(DieselError::RollbackTransaction)
                }
            }))
    })??)
}

pub(crate) fn load_policy(conn: db::PoolConnection) -> AuthServiceResult<Vec<CasbinRule>> {
    use schema::casbin_auth::dsl::casbin_auth;

    Ok(casbin_auth.load::<CasbinRule>(&conn)?)
}

pub(crate) fn add_policy(
    conn: db::PoolConnection,
    new_rule: NewCasbinRule,
) -> AuthServiceResult<bool> {
    use schema::casbin_auth::dsl::casbin_auth;

    Ok(diesel::insert_into(casbin_auth)
        .values(&new_rule)
        .execute(&conn)
        .map(|n| n == 1)?)
}

pub(crate) fn add_policies(
    conn: db::PoolConnection,
    new_rules: Vec<NewCasbinRule>,
) -> AuthServiceResult<bool> {
    use schema::casbin_auth::dsl::casbin_auth;

    Ok(conn.transaction::<_, DieselError, _>(|| {
        diesel::insert_into(casbin_auth)
            .values(&new_rules)
            .execute(&*conn)
            .and_then(|n| {
                if n == new_rules.len() {
                    Ok(true)
                } else {
                    Err(DieselError::RollbackTransaction)
                }
            })
            .map_err(|_| DieselError::RollbackTransaction)
    })?)
}

fn normalize_casbin_auth(mut rule: Vec<String>, field_index: usize) -> Vec<String> {
    rule.resize(6 - field_index, String::from(""));
    rule
}
