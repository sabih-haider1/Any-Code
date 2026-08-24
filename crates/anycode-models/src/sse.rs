//! Minimal Server-Sent-Events line framing, shared by the OpenAI and Anthropic adapters
//! (both stream over SSE; a real crate for this is overkill for "split on blank lines
//! and strip a `data:` prefix"). Pure and unit-tested — the actual network plumbing that
//! feeds it bytes is thin glue not worth testing on its own.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SseEvent {
    pub event: Option<String>,
    pub data: String,
}

#[derive(Default)]
pub struct SseDecoder {
    buffer: String,
}

impl SseDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Feeds a chunk of the response body and returns every complete event it now
    /// contains. An event that straddles two chunks is held until the next call.
    pub fn push(&mut self, chunk: &str) -> Vec<SseEvent> {
        self.buffer.push_str(chunk);
        let mut events = Vec::new();

        while let Some(boundary) = self.buffer.find("\n\n") {
            let raw: String = self.buffer.drain(..boundary + 2).collect();
            let mut event_name = None;
            let mut data_lines = Vec::new();
            for line in raw.lines() {
                if let Some(rest) = line.strip_prefix("data:") {
                    data_lines.push(rest.trim_start());
                } else if let Some(rest) = line.strip_prefix("event:") {
                    event_name = Some(rest.trim().to_string());
                }
            }
            if !data_lines.is_empty() {
                events.push(SseEvent {
                    event: event_name,
                    data: data_lines.join("\n"),
                });
            }
        }
        events
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_single_data_only_event() {
        let mut decoder = SseDecoder::new();
        let events = decoder.push("data: {\"a\":1}\n\n");
        assert_eq!(
            events,
            vec![SseEvent {
                event: None,
                data: "{\"a\":1}".into()
            }]
        );
    }

    #[test]
    fn parses_named_events() {
        let mut decoder = SseDecoder::new();
        let events = decoder.push("event: message_start\ndata: {\"x\":true}\n\n");
        assert_eq!(
            events,
            vec![SseEvent {
                event: Some("message_start".into()),
                data: "{\"x\":true}".into()
            }]
        );
    }

    #[test]
    fn reassembles_an_event_split_across_two_chunks() {
        let mut decoder = SseDecoder::new();
        assert_eq!(decoder.push("data: {\"a\""), vec![]);
        let events = decoder.push(":1}\n\n");
        assert_eq!(
            events,
            vec![SseEvent {
                event: None,
                data: "{\"a\":1}".into()
            }]
        );
    }

    #[test]
    fn handles_multiple_events_in_one_chunk() {
        let mut decoder = SseDecoder::new();
        let events = decoder.push("data: one\n\ndata: two\n\n");
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].data, "one");
        assert_eq!(events[1].data, "two");
    }
}
