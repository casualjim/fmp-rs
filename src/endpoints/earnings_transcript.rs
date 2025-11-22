use crate::client::FmpHttpClient;
use crate::errors::FmpResult;
use crate::types::earnings_transcript::{
  AvailableTranscriptSymbol, EarningTranscript, LatestEarningTranscript, LatestTranscriptsParams, TranscriptDate,
  TranscriptDatesParams, TranscriptParams,
};

pub async fn latest_transcripts(
  http: &FmpHttpClient,
  params: LatestTranscriptsParams,
) -> FmpResult<Vec<LatestEarningTranscript>> {
  http.get_json("/earning-call-transcript-latest", &params).await
}

pub async fn transcript(http: &FmpHttpClient, params: TranscriptParams) -> FmpResult<Vec<EarningTranscript>> {
  http.get_json("/earning-call-transcript", &params).await
}

pub async fn transcript_dates(http: &FmpHttpClient, params: TranscriptDatesParams) -> FmpResult<Vec<TranscriptDate>> {
  http.get_json("/earning-call-transcript-dates", &params).await
}

pub async fn available_transcript_symbols(http: &FmpHttpClient) -> FmpResult<Vec<AvailableTranscriptSymbol>> {
  http.get_json("/earnings-transcript-list", &()).await
}
