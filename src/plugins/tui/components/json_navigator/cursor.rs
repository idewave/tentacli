use std::collections::HashMap;
use std::mem;
use serde_json::Value;

use crate::plugins::tui::components::json_navigator::helpers::format_json;

#[derive(Default)]
pub struct JsonCursor {
    pub origin: Value,
    pub state: State,
    pub spans: HashMap<String, ValueSpan>,
}

impl JsonCursor {
    pub fn set_json(&mut self, json_string: &str) -> anyhow::Result<()> {
        if json_string.trim().is_empty() {
            anyhow::bail!("JSON string is empty !");
        }

        let origin: Value = serde_json::from_str(json_string)?;

        let keys = match &origin {
            Value::Object(map) => NodeKeys::Object(map.keys().cloned().collect()),
            Value::Array(arr) => NodeKeys::Array(arr.len()),
            _ => anyhow::bail!("Primitive value cannot be used with this iterator !")
        };

        self.origin = origin;
        self.state = State::new(keys)?;
        self.spans = build_all_value_spans(&format_json(json_string), &self.origin);

        Ok(())
    }

    pub fn current_path(&self) -> Option<String> {
        let index = self.state.current_index?;

        let mut path = self.state.path.clone();

        let segment = match &self.state.current_keys {
            NodeKeys::Object(keys) => keys.get(index)?.clone(),
            NodeKeys::Array(_) => index.to_string(),
            _ => return None,
        };

        path.push(segment);
        Some(path.join("/"))
    }

    pub fn current_span(&self) -> Option<ValueSpan> {
        let meta = self.current_metadata()?;
        let path = if meta.path.is_empty() {
            meta.key
        } else {
            format!("{}/{}", meta.path, meta.key)
        };

        self.spans.get(&path).copied()
    }

    pub fn step_forward(&mut self) -> Option<Metadata> {
        let next = match self.state.current_index {
            Some(i) if i < self.state.max_index => i + 1,
            Some(_) => return None,
            None => 0,
        };

        self.state.current_index = Some(next);
        self.current_metadata()
    }

    pub fn step_backward(&mut self) -> Option<Metadata> {
        let prev = match self.state.current_index {
            Some(i) if i > 0 => i - 1,
            Some(_) => return None,
            None => self.state.max_index,
        };

        self.state.current_index = Some(prev);
        self.current_metadata()
    }

    fn current_metadata(&self) -> Option<Metadata> {
        let index = self.state.current_index?;

        match &self.state.current_keys {
            NodeKeys::Object(keys) => Some(Metadata {
                key: keys.get(index)?.to_string(),
                path: self.state.path.join("/"),
            }),
            NodeKeys::Array(_) => Some(Metadata {
                key: index.to_string(),
                path: self.state.path.join("/"),
            }),
            _ => None,
        }
    }

    pub fn step_into(&mut self) {
        let index = match self.state.current_index {
            Some(i) => i,
            None => return,
        };

        let segment = match &self.state.current_keys {
            NodeKeys::Object(keys) => keys.get(index).cloned(),
            NodeKeys::Array(_) => Some(index.to_string()),
            _ => None,
        };

        let Some(segment) = segment else { return };

        let mut path = self.state.path.clone();
        path.push(segment);

        let ptr = format!("/{}", path.join("/"));
        let Some(value) = self.origin.pointer(&ptr) else { return };

        if matches!(value, Value::Object(_) | Value::Array(_)) {
            self.state.extend_path();
            self.update_state();
        }
    }

    pub fn step_out(&mut self) {
        let Some(last) = self.state.path.pop() else {
            return;
        };

        self.update_state();

        let idx = match &self.state.current_keys {
            NodeKeys::Object(keys) => {
                keys.iter().position(|k| k == &last)
            }
            NodeKeys::Array(_) => {
                last.parse::<usize>().ok()
            }
            _ => None,
        };

        self.state.current_index = idx;
    }

    fn update_state(&mut self) {
        let keys = self.get_keys();
        if keys != NodeKeys::None {
            self.state.update_state(keys);
        }
    }

    fn get_keys(&self) -> NodeKeys {
        let joined = self.state.path.join("/");

        let current_node = if joined.is_empty() {
            Some(&self.origin)
        } else {
            let ptr = format!("/{}", joined);
            self.origin.pointer(&ptr)
        };

        match current_node {
            Some(Value::Object(map)) => NodeKeys::Object(map.keys().cloned().collect()),
            Some(Value::Array(arr)) => NodeKeys::Array(arr.len()),
            _ => NodeKeys::None,
        }
    }

    pub fn reset(&mut self) {
        self.origin = Value::Null;
        self.state = State::default();
        self.spans.clear();
    }

    pub fn is_at_root(&self) -> bool {
        self.state.path.is_empty()
    }
}

#[derive(Debug, Default)]
pub struct Metadata {
    pub key: String,
    pub path: String,
}

#[derive(Default, Debug)]
pub struct State {
    path: Vec<String>,
    current_keys: NodeKeys,
    current_index: Option<usize>,
    max_index: usize,
}

impl State {
    pub fn path_len(&self) -> usize {
        self.path.len()
    }

    fn apply_keys(&mut self, keys: NodeKeys) -> anyhow::Result<()> {
        let max_index = match &keys {
            NodeKeys::Object(map) if !map.is_empty() => map.len() - 1,
            NodeKeys::Array(len) if *len > 0 => len - 1,
            _ => anyhow::bail!("Empty or primitive value cannot be navigated"),
        };

        self.current_keys = keys;
        self.max_index = max_index;
        self.current_index = None;

        Ok(())
    }

    fn new(keys: NodeKeys) -> anyhow::Result<Self> {
        let mut state = Self::default();
        state.apply_keys(keys)?;
        Ok(state)
    }

    fn update_state(&mut self, keys: NodeKeys) {
        let _ = self.apply_keys(keys);
    }

    pub fn extend_path(&mut self) {
        let Some(index) = mem::take(&mut self.current_index) else { return };

        match &self.current_keys {
            NodeKeys::Object(current_keys) => {
                if let Some(key) = current_keys.get(index) {
                    self.path.push(key.to_string());
                }
            },
            NodeKeys::Array(_) => {
                self.path.push(index.to_string());
            },
            _ => {}
        }
    }
}

#[derive(Default, PartialEq, Debug)]
enum NodeKeys {
    #[default]
    None,
    Object(Vec<String>),
    Array(usize),
}

#[derive(Debug, Clone, Copy)]
pub struct ValueSpan {
    pub offset: usize,
    pub size: usize,
}

fn skip_ws(s: &str, mut i: usize) -> usize {
    while i < s.len() {
        match s.as_bytes()[i] {
            b' ' | b'\n' | b'\r' | b'\t' => i += 1,
            _ => break,
        }
    }
    i
}

fn parse_one_value_span(json: &str, start: usize) -> Option<(usize, usize)> {
    let start = skip_ws(json, start);

    let de = serde_json::Deserializer::from_str(&json[start..]);
    let mut iter = de.into_iter::<Value>();

    let _ = iter.next()?.ok()?;
    let consumed = iter.byte_offset();

    Some((start, start + consumed))
}

/// Build map: "a/b/1/d" → (offset, size)
pub fn build_all_value_spans(
    json: &str,
    root: &Value,
) -> HashMap<String, ValueSpan> {
    let mut spans = HashMap::new();

    fn walk(
        json: &str,
        value: &Value,
        path: &mut Vec<String>,
        scan_pos: usize,
        spans: &mut HashMap<String, ValueSpan>,
    ) -> Option<()> {
        let path_str = path.join("/");

        if let Some((s, e)) = parse_one_value_span(json, scan_pos) {
            spans.insert(
                path_str.clone(),
                ValueSpan {
                    offset: s,
                    size: e - s,
                },
            );

            match value {
                Value::Object(map) => {
                    let mut pos = s + 1;

                    for (k, v) in map {
                        let key = format!("\"{}\"", k);
                        let key_rel = json[pos..].find(&key)?;
                        let key_start = pos + key_rel;

                        let colon_rel =
                            json[key_start + key.len()..].find(':')?;
                        let value_start = skip_ws(
                            json,
                            key_start + key.len() + colon_rel + 1,
                        );

                        path.push(k.clone());
                        walk(json, v, path, value_start, spans);
                        path.pop();

                        pos = value_start;
                        let (_, end) = parse_one_value_span(json, pos)?;
                        pos = skip_ws(json, end);
                        if json.as_bytes().get(pos) == Some(&b',') {
                            pos += 1;
                        }
                    }
                }

                Value::Array(arr) => {
                    let mut pos = s + 1;
                    pos = skip_ws(json, pos);

                    for (i, v) in arr.iter().enumerate() {
                        path.push(i.to_string());
                        walk(json, v, path, pos, spans);
                        path.pop();

                        let (_, end) = parse_one_value_span(json, pos)?;
                        pos = skip_ws(json, end);
                        if json.as_bytes().get(pos) == Some(&b',') {
                            pos += 1;
                        }
                    }
                }

                _ => {}
            }
        }

        Some(())
    }

    let mut path = Vec::new();
    walk(json, root, &mut path, 0, &mut spans);

    spans
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_JSON: &str = r#"
    {
        "field1": 10,
        "field2": [1, 2, 3],
        "field3": {
            "nested": true
        }
    }
    "#;

    fn make_cursor() -> JsonCursor {
        let mut cursor = JsonCursor::default();
        cursor.set_json(SAMPLE_JSON).unwrap();
        cursor
    }

    #[test]
    fn set_json_rejects_empty_string() {
        let mut cursor = JsonCursor::default();
        let err = cursor.set_json("").unwrap_err();
        assert!(
            err.to_string().contains("empty"),
            "expected empty JSON error, got: {err}"
        );
    }

    #[test]
    fn set_json_rejects_primitive_root() {
        let mut cursor = JsonCursor::default();
        let err = cursor.set_json("true").unwrap_err();
        assert!(
            err.to_string().contains("Primitive"),
            "expected primitive root error, got: {err}"
        );
    }

    #[test]
    fn forward_and_backward_navigation_on_root() {
        let mut cursor = make_cursor();

        let first = cursor.step_forward().unwrap();
        let second = cursor.step_forward().unwrap();
        let back = cursor.step_backward().unwrap();

        assert_eq!(first.key, back.key);
        assert_ne!(first.key, second.key);
    }

    #[test]
    fn current_path_on_object_root() {
        let mut cursor = make_cursor();

        cursor.step_forward();
        let path = cursor.current_path().unwrap();

        // root-level key should be just the key name
        assert!(
            path == "field1" || path == "field2" || path == "field3",
            "unexpected path: {path}"
        );
    }

    #[test]
    fn step_into_and_out_of_object() {
        let mut cursor = make_cursor();

        // move to "field3"
        loop {
            let meta = cursor.step_forward().unwrap();
            if meta.key == "field3" {
                break;
            }
        }

        // step into { "nested": true }
        cursor.step_into();

        let meta = cursor.step_forward().unwrap();
        assert_eq!(meta.key, "nested");
        assert_eq!(cursor.current_path().unwrap(), "field3/nested");

        // step back out
        cursor.step_out();
        let path = cursor.current_path().unwrap();
        assert_eq!(path, "field3");
    }

    #[test]
    fn spans_are_built_for_root_and_nested_values() {
        let cursor = make_cursor();

        // root span must exist
        let root_span = cursor.spans.get("").copied();
        assert!(root_span.is_some(), "root span missing");

        // nested span must exist
        let nested_span = cursor.spans.get("field3/nested").copied();
        assert!(nested_span.is_some(), "nested span missing");

        let span = nested_span.unwrap();
        assert!(span.size > 0, "span size must be non-zero");
    }

    #[test]
    fn reset_clears_state() {
        let mut cursor = make_cursor();
        cursor.reset();

        assert!(cursor.origin.is_null());
        assert!(cursor.spans.is_empty());
        assert!(cursor.state.current_index.is_none());
        assert!(cursor.state.path.is_empty());
    }
}
