use super::{OutputStream, Step};
use process_stream::ProcessItem;
use tokio_stream::StreamExt;

/// Ignore stream content til the line is empty
///
/// Return setps error if any
pub async fn consume_till_empty_line(stream: &mut OutputStream) -> Vec<Step> {
    let mut errors = vec![];
    while let Some(s) = stream.next().await {
        if let ProcessItem::Error(s) = s {
            let line = s.trim();
            if !line.is_empty() {
                errors.push(Step::Error(s));
            }
        } else if s.trim().is_empty() {
            break;
        }
    }
    errors
}

pub async fn get_commands_and_compile_errors(stream: &mut OutputStream) -> (String, Vec<Step>) {
    let mut steps = vec![];
    let mut command = String::default();
    while let Some(s) = stream.next().await {
        if let ProcessItem::Output(line) = s {
            let line = line.trim();

            if line.is_empty() {
                break;
            } else if line.starts_with("cd") {
                continue;
            }

            if line.contains("error:") {
                steps.extend(consume_errors(line, stream).await);
            };

            command = line.to_string();
        } else if let ProcessItem::Error(line) = s {
            if line.trim().is_empty() {
                continue;
            }
            steps.push(Step::Error(line));
        }
    }
    (command, steps)
}

/// consume_errors for line containing "error:"
pub async fn consume_errors(line: &str, stream: &mut OutputStream) -> Vec<Step> {
    // WARN: This might panic!
    fn format_line_source(line: &str) -> String {
        let mut parts = line.split(": error: ");
        let file_path = parts.next().unwrap();
        let msg = parts.next().unwrap();
        format!(
            "{}{} ({file_path})",
            (&msg[..1].to_string()).to_uppercase(),
            &msg[1..]
        )
    }

    let mut errors = vec![];

    errors.push(Step::Error(String::default()));
    errors.push(Step::Error(format_line_source(line)));

    while let Some(line) = stream.next().await {
        if line.is_empty() {
            break;
        }
        if line.contains("~~") {
            errors.push(Step::Error(line.to_string()));
            errors.push(Step::Error(String::default()));
        } else if line.starts_with("/") {
            errors.push(Step::Error(format_line_source(&line)));
        } else {
            errors.push(Step::Error(line.to_string()));
        }
    }
    errors
}

#[cfg(test)]
pub(crate) mod test {
    macro_rules! try_to_stream_test {
        ($t:ident, $text:literal) => {{
            use tap::Tap;
            use tokio_stream::StreamExt;
            let sample = $text
                .split("\n")
                .map(|s| process_stream::ProcessItem::Output(s.into()))
                .collect::<Vec<process_stream::ProcessItem>>();
            let mut stream = tokio_stream::iter(sample);
            let line = stream
                .next()
                .await
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
