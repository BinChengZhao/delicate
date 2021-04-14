#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate diesel_migrations;

pub(crate) mod models;
pub(crate) mod schema;

pub mod db;

#[macro_use]
pub(crate) mod macros;

pub(crate) use {cfg_mysql_support, cfg_postgres_support};

use self::diesel::prelude::*;
use diesel::query_dsl::RunQueryDsl;
use models::*;
use schema::posts::dsl::*;

fn main() {
    db::init();

    let connection = db::establish_connection();
    // create_post(&connection, "title", "body", 1);
    delete_post(&connection, 1);

    let results = posts
        .filter(published.eq(1))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}

pub fn create_post<'a>(
    conn: &PgConnection,
    title_str: &'a str,
    body_str: &'a str,
    id_num: i64,
) -> usize {
    use schema::posts;

    let new_post = NewPost {
        id: id_num,
        title: title_str,
        body: body_str,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post")
}

pub fn update_post<'a>(conn: &PgConnection, id_num: i64) -> usize {
    use schema::posts;

    diesel::update(posts::table)
        .filter(posts::id.eq(id_num))
        .set(published.eq(1))
        .execute(conn)
        .unwrap()
}

pub fn update_post_tilte<'a>(conn: &PgConnection, id_num: i64) -> usize {
    diesel::update(posts.find(id_num))
        .set(title.eq("update"))
        .execute(conn)
        .unwrap()
}

pub fn delete_post<'a>(conn: &PgConnection, id_num: i64) -> usize {
    diesel::delete(posts.find(id_num)).execute(conn).unwrap()
}
