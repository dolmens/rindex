use rindex::{
    column::GenericColumnReader,
    document::Document,
    index::PostingIterator,
    query::Term,
    schema::{SchemaBuilder, COLUMN, INDEXED},
    table::{Table, TableSettings},
    DocId,
};

fn get_all_docs(posting_iter: &mut dyn PostingIterator) -> Vec<DocId> {
    let mut docids = vec![];
    let mut docid = 0;
    loop {
        match posting_iter.seek(docid) {
            Some(seeked) => {
                docids.push(seeked);
                docid = seeked + 1;
            }
            None => break,
        }
    }

    docids
}

pub fn main() {
    let mut schema_builder = SchemaBuilder::new();
    schema_builder.add_text_field("title".to_string(), COLUMN | INDEXED);
    let schema = schema_builder.build();
    let settings = TableSettings::new();
    let table = Table::open_in(schema, settings, "./testdata");

    let mut writer = table.writer();

    let mut doc1 = Document::new();
    doc1.add_field("title".to_string(), "hello world");
    writer.add_doc(&doc1);

    let mut doc2 = Document::new();
    doc2.add_field("title".to_string(), "world peace");
    writer.add_doc(&doc2);

    let reader = table.reader();
    let index_reader = reader.index_reader();

    let term = Term::new("title".to_string(), "hello".to_string());
    let mut posting_iter = index_reader.lookup(&term).unwrap();
    let docids = get_all_docs(&mut *posting_iter);
    assert_eq!(docids, vec![0]);

    let term = Term::new("title".to_string(), "world".to_string());
    let mut posting_iter = index_reader.lookup(&term).unwrap();
    let docids = get_all_docs(&mut *posting_iter);
    assert_eq!(docids, vec![0, 1]);

    let term = Term::new("title".to_string(), "peace".to_string());
    let mut posting_iter = index_reader.lookup(&term).unwrap();
    let docids = get_all_docs(&mut *posting_iter);
    assert_eq!(docids, vec![1]);

    let column_reader = reader.column_reader();
    let title_column_reader = column_reader
        .typed_column::<String, GenericColumnReader<_>>("title")
        .unwrap();
    assert_eq!(title_column_reader.get(0), Some("hello world".to_string()));
    assert_eq!(title_column_reader.get(1), Some("world peace".to_string()));
}
