use crate::macros::define_api_trait;
use crate::types::commodity::Commodity;

define_api_trait!(
  /// API endpoints for commodities.
  CommodityApi,
  commodity_list -> "/commodities-list" -> () -> Vec<Commodity>,
);
