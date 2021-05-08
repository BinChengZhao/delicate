use super::prelude::*;

impl<T> QueryFragment<Mysql> for Paginated<T>
where
    T: QueryFragment<Mysql>,
{
    fn walk_ast(&self, mut out: AstPass<Mysql>) -> QueryResult<()> {
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
    type SqlType = (T::SqlType);
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
    pub fn per_page(self, per_page: i64) -> Self {
        Paginated { per_page, ..self }
    }
}

impl<T> Paginated<T> {
    fn load_pages<U>(self, conn: &MysqlConnection) -> QueryResult<Vec<U>>
    where
        Self: LoadQuery<MysqlConnection, U>,
    {
        // let per_page = self.per_page;
         self.load::<U>(conn)
        // let results = self.load::<U>(conn)?;

        // let total = *results.get(0).map(|(_, total)| total).unwrap_or(&0);
        // let records = results.into_iter().map(|(record, _)| record).collect();
        // let total_pages = (total as f64 / per_page as f64).ceil() as i64;
    }
}
