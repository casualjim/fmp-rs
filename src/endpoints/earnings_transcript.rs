use crate::macros::define_api_trait;
use crate::types::earnings_transcript::{
  AvailableTranscriptSymbol, EarningTranscript, LatestEarningTranscript, LatestTranscriptsParams, TranscriptDate,
  TranscriptDatesParams, TranscriptParams,
};

define_api_trait!(
  /// API endpoints for earnings_transcript.
  EarningsTranscriptApi,
  latest_transcripts -> "/earning-call-transcript-latest" -> LatestTranscriptsParams  -> Vec<LatestEarningTranscript>,
  transcript -> "/earning-call-transcript" -> TranscriptParams  -> Vec<EarningTranscript>,
  transcript_dates -> "/earning-call-transcript-dates" -> TranscriptDatesParams  -> Vec<TranscriptDate>,
  available_transcript_symbols -> "/earnings-transcript-list" -> () -> Vec<AvailableTranscriptSymbol>,
);
