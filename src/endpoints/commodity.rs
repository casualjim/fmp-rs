use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::commodity::Commodity;

pub async fn commodity_list(http: &FmpHttpClient) -> FmpResult<Vec<Commodity>> {
  http.get_json("/commodity-list", &()).await
}
