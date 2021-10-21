use crate::prelude::*;

// In order to be compatible with the front-end components,
// some of the return values have to be named with small humps when they are returned
// which is very sad.

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PaginateData<T> {
    data_source: Vec<T>,
    state_desc: HashMap<&'static str, HashMap<usize, &'static str>>,
    pagination: Pagination,
}

// PaginateData.state_desc {
// "login" : { 1: "login sucess" }
// "task"  : { 2: "task running" }
//}

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

    pub(crate) fn set_state_desc<State: crate::db::state::DescribeState>(mut self) -> Self {
        self.state_desc.insert(State::state_name(), State::desc());
        self
    }

    pub(crate) fn set_total(mut self, total: i64) -> Self {
        self.pagination.total = total;
        self
    }
}

impl<T> Default for PaginateData<T> {
    fn default() -> Self {
        let data_source = Vec::<T>::new();
        let pagination = Pagination::default();
        let state_desc = HashMap::default();

        PaginateData {
            data_source,
            state_desc,
            pagination,
        }
    }
}
