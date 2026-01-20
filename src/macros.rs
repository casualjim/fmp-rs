/// Macro to generate extension traits for FMP API endpoints.
///
/// This macro generates a trait and its implementation for `FmpHttpClient`, providing
/// methods that call the underlying HTTP client with predefined API paths.
///
/// # Syntax
///
/// ```text
/// define_api_trait!(
///   /// Documentation for the trait (optional)
///   TraitName,
///   method_name -> "/api-path" -> (ParamType) -> ReturnType,
///   another_method -> "/another-path" -> (AnotherParamType) -> AnotherReturnType,
/// );
/// ```
///
/// # Example
///
/// In your endpoint module:
/// ```rust,ignore
/// use crate::macros::define_api_trait;
/// use crate::types::quotes::{QuoteParams, StockQuote};
///
/// define_api_trait!(
///   /// Quote API endpoints
///   QuotesApi,
///   quote -> "/quote" -> (QuoteParams) -> Vec<StockQuote>,
/// );
/// ```
///
/// Users can then call:
/// ```rust,ignore
/// use fmp::QuotesApi;
///
/// let quotes = client.quote(params).await?;
/// ```
macro_rules! define_api_trait {
  (
    $(#[$attr:meta])*
    $trait_name:ident,
    $(
      $fn_name:ident -> $path:literal -> $param_ty:tt -> $return_ty:ty
    ),* $(,)?
  ) => {
    $(#[$attr])*
    pub trait $trait_name {
      $(
        fn $fn_name(&self, params: $param_ty) -> impl std::future::Future<Output = crate::errors::FmpResult<$return_ty>> + Send;
      )*
    }

    impl $trait_name for crate::client::FmpHttpClient {
      $(
        fn $fn_name(&self, params: $param_ty) -> impl std::future::Future<Output = crate::errors::FmpResult<$return_ty>> + Send {
          async move { self.get_json($path, &params).await }
        }
      )*
    }
  };
}

pub(crate) use define_api_trait;
