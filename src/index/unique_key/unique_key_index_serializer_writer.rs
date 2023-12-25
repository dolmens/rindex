use std::{fs::File, io::Write, path::Path};

use crate::DocId;

pub struct UniqueKeyIndexSerializerWriter {
    file: File,
}

impl UniqueKeyIndexSerializerWriter {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            file: File::create(path).unwrap(),
        }
    }

    pub fn write(&mut self, key: &str, docid: DocId) {
        writeln!(self.file, "{} {}", key, docid).unwrap();
    }
}
