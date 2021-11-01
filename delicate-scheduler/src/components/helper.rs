use super::prelude::*;

pub(crate) async fn handle_response<F: Future<Output = Result<RequestResponse, RequestError>>,
                                    T: DeserializeOwned + Trial>(
    request_all: JoinAll<F>)
    -> Vec<T> {
    let response_json_all: JoinAll<_> = request_all.await
                                                   .into_iter()
                                                   .map(|response| match response {
                                                       Ok(r) => Some(r.json::<T>()),
                                                       Err(e) => {
                                                           error!("SendRequestError : {}", e);
                                                           None
                                                       },
                                                   })
                                                   .filter(|r| r.is_some())
                                                   .map(|r| r.expect(""))
                                                   .collect();

    response_json_all.await
                     .into_iter()
                     .map(|json_result| {
                         match json_result {
                             Err(ref e) => {
                                 error!("Payload errors: {}", e);
                             },
                             Ok(ref json) if json.is_err() => {
                                 error!("Customized error messages: {}", json.get_msg());
                             },
                             _ => {},
                         }
                         json_result
                     })
                     .filter_map(|r| r.ok())
                     .collect::<Vec<T>>()
}
