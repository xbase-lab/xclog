use super::OutputStream;
use tokio_stream::StreamExt;

/// Ignore stream content til the line is empty
pub async fn consume_till_empty_line(stream: &mut OutputStream) {
    while let Ok(Some(line)) = stream.try_next().await {
        if line.trim().is_empty() {
            break;
        }
    }
}

#[cfg(test)]
pub(crate) mod test {
    macro_rules! try_to_stream_test {
        ($t:ident, $text:literal) => {{
            use tap::Tap;
            use tokio_stream::StreamExt;
            let sample = $text
                .split("\n")
                .map(ToString::to_string)
                .map(Ok)
                .collect::<Vec<Result<String, std::io::Error>>>();
            let mut stream = tokio_stream::iter(sample);
            let line = stream
                .next()
                .await
                .unwrap()
                .unwrap()
                .split_whitespace()
                .tap_mut(|s| {
                    s.next();
                })
                .as_str()
                .to_string();

            let step = $t::parse_from_stream(line, &mut stream).await;
            #[cfg(feature = "tracing")]
            tracing::info!("Result: {:#?}", step);
            step
        }};
    }

    pub(crate) use try_to_stream_test;

    macro_rules! to_stream_test {
        ($t:ident, $text:literal) => {{
            crate::parser::util::test::try_to_stream_test!($t, $text).unwrap()
        }};
    }

    pub(crate) use to_stream_test;
}
