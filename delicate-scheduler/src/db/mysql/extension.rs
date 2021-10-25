use super::prelude::*;

impl<T> QueryFragment<Mysql> for Paginated<T>
where
    T: QueryFragment<Mysql>,
{
    fn walk_ast(&self, mut out: AstPass<'_, Mysql>) -> QueryResult<()> {
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(" LIMIT ");
        out.push_bind_param::<sql_types::BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        let offset = (self.page - 1) * self.per_page;
        out.push_bind_param::<sql_types::BigInt, _>(&offset)?;
        Ok(())
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = T::SqlType;
}

impl<T> RunQueryDsl<MysqlConnection> for Paginated<T> {}

pub(crate) trait Paginate: AsQuery + Sized {
    fn paginate(self, page: i64) -> Paginated<Self::Query> {
        Paginated {
            query: self.as_query(),
            page,
            per_page: DEFAULT_PER_PAGE,
        }
    }
}

impl<T: AsQuery> Paginate for T {}

const DEFAULT_PER_PAGE: i64 = 10;

#[derive(QueryId)]
pub struct Paginated<T> {
    query: T,
    page: i64,
    per_page: i64,
}

impl<T> Paginated<T> {
    pub fn set_per_page(self, per_page: i64) -> Self {
        Paginated { per_page, ..self }
    }
}

impl<T> Paginated<T> {
    #[allow(dead_code)]
    fn load_pages<U>(self, conn: &MysqlConnection) -> QueryResult<Vec<U>>
    where
        Self: LoadQuery<MysqlConnection, U>,
    {
        self.load::<U>(conn)
    }
}
