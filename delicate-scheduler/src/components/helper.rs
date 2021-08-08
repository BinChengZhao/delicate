use super::prelude::*;

pub(crate) async fn handle_response<T: DeserializeOwned + Trial>(
    request_all: JoinAll<SendClientRequest>,
) -> Vec<T> {
    let response_json_all: JoinAll<JsonBody<Decompress<Payload>, T>> = request_all
        .await
        .into_iter()
        .map(|response| match response {
            Ok(mut r) => Some(r.json::<T>()),
            Err(e) => {
                error!("{}", e);
                None
            }
        })
        .filter(|r| r.is_some())
        .map(|r| r.expect(""))
        .collect::<Vec<JsonBody<Decompress<Payload>, T>>>()
        .into_iter()
        .collect();

    response_json_all
        .await
        .into_iter()
        .map(|json_result| {
            match json_result {
                Err(ref e) => {
                    error!("{}", e);
                }
                Ok(ref json) if json.is_err() => {
                    error!("{}", json.get_msg());
                }
                _ => {}
            }
            json_result
        })
        .filter_map(|r| r.ok())
        .collect::<Vec<T>>()
}
