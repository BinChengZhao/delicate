use crate::prelude::*;

// In order to be compatible with the front-end components,
// some of the return values have to be named with small humps when they are returned
// which is very sad.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PaginateData<T> {
    data_source: Vec<T>,
    pagination: Pagination,
}

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Pagination {
    total: i64,
    page_size: i64,
}

impl<T> PaginateData<T> {
    pub(crate) fn set_data_source(mut self, data_source: Vec<T>) -> Self {
        self.data_source = data_source;
        self
    }

    pub(crate) fn set_page_size(mut self, per_page: i64) -> Self {
        self.pagination.page_size = per_page;
        self
    }

    pub(crate) fn set_total(mut self, total: i64) -> Self {
        self.pagination.total = (total as f64 / self.pagination.page_size as f64).ceil() as i64;
        self
    }
}

impl<T> Default for PaginateData<T> {
    fn default() -> Self {
        let data_source = Vec::<T>::new();
        let pagination = Pagination::default();

        PaginateData {
            data_source,
            pagination,
        }
    }
}
