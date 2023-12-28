use std::collections::HashMap;

use crate::{index::IndexMerger, DocId};

use super::{PrimaryKeyIndexSegmentData, PrimaryKeyIndexSerializerWriter};

#[derive(Default)]
pub struct PrimaryKeyIndexMerger {}

impl IndexMerger for PrimaryKeyIndexMerger {
    fn merge(
        &self,
        directory: &std::path::Path,
        index: &crate::schema::Index,
        segments: &[&dyn crate::index::IndexSegmentData],
        _doc_counts: &[usize],
    ) {
        let path = directory.join(index.name());
        let mut writer = PrimaryKeyIndexSerializerWriter::new(path);
        let mut keys = HashMap::<String, DocId>::new();
        for &segment in segments {
            let segment_data = segment.downcast_ref::<PrimaryKeyIndexSegmentData>().unwrap();
            for (key, &docid) in segment_data.keys.iter() {
                keys.insert(key.clone(), docid);
            }
        }

        for (key, docid) in keys {
            writer.write(&key, docid);
        }
    }
}