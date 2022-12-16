use std::io::{Write};
use serde_json::ser::{PrettyFormatter, Serializer};

#[derive(Default)]
pub struct JsonFormatter {
    pretty: PrettyFormatter<'static>,
}

impl JsonFormatter {
    pub fn init() -> Serializer<Vec<u8>, Self> {
        let buffer = Vec::new();
        Serializer::with_formatter(buffer, JsonFormatter::default())
    }
}

impl serde_json::ser::Formatter for JsonFormatter {
    fn begin_object<W>(&mut self, writer: &mut W) -> std::io::Result<()> where W: ?Sized + Write {
        self.pretty.begin_object(writer)
    }

    fn end_object<W>(&mut self, writer: &mut W) -> std::io::Result<()> where W: ?Sized + Write {
        self.pretty.end_object(writer)
    }

    fn begin_object_key<W>(
        &mut self,
        writer: &mut W,
        first: bool
    ) -> std::io::Result<()> where W: ?Sized + Write {
        self.pretty.begin_object_key(writer, first)
    }

    fn end_object_value<W>(&mut self, writer: &mut W) -> std::io::Result<()> where W: ?Sized + Write {
        self.pretty.end_object_value(writer)
    }
}