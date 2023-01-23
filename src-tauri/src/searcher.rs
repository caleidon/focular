use crate::{
    directory::{get_dir_path, Dir},
    foc_error::Result,
    models::metadata::{self, get_metadata_by_hash, Metadata},
};
use once_cell::sync::OnceCell;
use std::{collections::VecDeque, sync::Mutex};
use tantivy::directory::MmapDirectory;
use tantivy::{
    collector::TopDocs,
    query::{BooleanQuery, Occur, QueryClone},
    IndexReader, IndexWriter,
};
use tantivy::{query::FuzzyTermQuery, schema::*};
use tantivy::{Index, ReloadPolicy};

static READER_INSTANCE: OnceCell<IndexReader> = OnceCell::new();
static WRITER_INSTANCE: OnceCell<Mutex<IndexWriter>> = OnceCell::new();

fn get_index() -> Result<Index> {
    let index_path = get_dir_path(Dir::Index)?;
    let mmap_dir = MmapDirectory::open(&index_path).unwrap();
    let index = Index::open(mmap_dir)?;
    Ok(index)
}

pub async fn init_reader() -> Result<()> {
    let index_path = get_dir_path(Dir::Index)?;
    let mmap_dir = MmapDirectory::open(&index_path).unwrap();

    let index: Index = if Index::exists(&mmap_dir)? {
        Index::open(mmap_dir)?
    } else {
        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("hash", STRING | STORED);
        schema_builder.add_text_field("name", TEXT);
        schema_builder.add_text_field("tags", TEXT);
        schema_builder.add_text_field("notes", TEXT);
        let schema = schema_builder.build();

        Index::create_in_dir(&index_path, schema)?
    };

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;

    let writer = index.writer(50_000_000)?;

    let _ = READER_INSTANCE.set(reader);
    let _ = WRITER_INSTANCE.set(Mutex::new(writer));

    Ok(())
}

pub fn index_metadata(metadata: &[Metadata]) -> Result<()> {
    let index = get_index()?;
    let reader = READER_INSTANCE.get().unwrap();
    let mutex_writer = WRITER_INSTANCE.get().unwrap();
    let mut writer = mutex_writer.lock().unwrap();

    let hash_field = index.schema().get_field("hash").unwrap();
    let name_field = index.schema().get_field("name").unwrap();
    let tags_field = index.schema().get_field("tags").unwrap();
    let notes_field = index.schema().get_field("notes").unwrap();

    for metadata in metadata {
        let mut doc = Document::new();

        doc.add_text(hash_field, &metadata.hash);
        doc.add_text(name_field, &metadata.name);
        if let Some(notes) = &metadata.notes {
            doc.add_text(notes_field, &notes);
        }
        if let Some(tags) = &metadata.tags {
            for tag in tags.iter() {
                doc.add_text(tags_field, &tag);
            }
        }

        writer.add_document(doc)?;
        println!("Indexed: {}", metadata.name);
    }

    writer.commit()?;
    reader.reload()?;

    Ok(())
}

pub fn delete_metadata(hashes: &Vec<String>) -> Result<()> {
    let index = get_index()?;
    let reader = READER_INSTANCE.get().unwrap();
    let mutex_writer = WRITER_INSTANCE.get().unwrap();
    let mut writer = mutex_writer.lock().unwrap();
    let hash_field = index.schema().get_field("hash").unwrap();

    for hash in hashes {
        let metadata_hash = Term::from_field_text(hash_field, hash);
        writer.delete_term(metadata_hash);
    }

    writer.commit()?;
    reader.reload()?;

    Ok(())
}

pub fn update_metadata(metadata: &Metadata) -> Result<()> {
    let index = get_index()?;
    let reader = READER_INSTANCE.get().unwrap();
    let mutex_writer = WRITER_INSTANCE.get().unwrap();
    let mut writer = mutex_writer.lock().unwrap();

    let hash_field = index.schema().get_field("hash").unwrap();
    let name_field = index.schema().get_field("name").unwrap();
    let tags_field = index.schema().get_field("tags").unwrap();
    let notes_field = index.schema().get_field("notes").unwrap();

    let metadata_hash = Term::from_field_text(hash_field, &metadata.hash);
    let mut doc = Document::new();
    doc.add_text(hash_field, &metadata.hash);
    doc.add_text(name_field, &metadata.name);
    if let Some(notes) = &metadata.notes {
        doc.add_text(notes_field, &notes);
    }
    if let Some(tags) = &metadata.tags {
        for tag in tags.iter() {
            doc.add_text(tags_field, &tag);
        }
    }
    writer.delete_term(metadata_hash);
    writer.add_document(doc)?;

    writer.commit()?;
    reader.reload()?;

    Ok(())
}

#[tauri::command]
pub async fn search_content(query: &str) -> Result<Vec<Metadata>> {
    let reader = READER_INSTANCE.get().unwrap();
    let index = get_index()?;
    let schema = index.schema();

    let hash_field = schema.get_field("hash").unwrap();
    let name_field = schema.get_field("name").unwrap();
    let tags_field = schema.get_field("tags").unwrap();
    let notes_field = schema.get_field("notes").unwrap();

    let searcher = reader.searcher();

    // TODO: add searching by smart folder or all search
    // TODO: search for regex with RegexQuery (according to chillfish)
    // TODO: we can make these settings into options from the Preferences menu
    let desired_amount_of_docs: u8 = 30;
    let number_of_fuzzy_iterations: u8 = 3;
    let mut previous_doc_adresses = VecDeque::with_capacity(desired_amount_of_docs as usize);
    let query_no_extra_spaces = remove_all_extra_whitespace(query);

    println!("=============================================");

    for search_iteration in 0..number_of_fuzzy_iterations {
        let mut sub_queries = Vec::new();
        for word in query_no_extra_spaces.split(' ') {
            let name_term = Term::from_field_text(name_field, word);
            let tags_term = Term::from_field_text(tags_field, word);
            let notes_term = Term::from_field_text(notes_field, word);

            let by_name = FuzzyTermQuery::new_prefix(name_term, search_iteration, true);
            let by_tag = FuzzyTermQuery::new_prefix(tags_term, search_iteration, true);
            let by_notes = FuzzyTermQuery::new_prefix(notes_term, search_iteration, true);

            let by_name_tag_notes = BooleanQuery::new(vec![
                (Occur::Should, by_name.box_clone()),
                (Occur::Should, by_tag.box_clone()),
                (Occur::Should, by_notes.box_clone()),
            ]);
            sub_queries.push((Occur::Should, by_name_tag_notes.box_clone()))
        }

        let found_documents = searcher.search(
            &BooleanQuery::new(sub_queries),
            &(TopDocs::with_limit(desired_amount_of_docs.into())),
        )?;

        for (.., doc_address) in found_documents {
            if !previous_doc_adresses.contains(&doc_address) {
                previous_doc_adresses.push_back(doc_address);
                let doc = searcher.doc(doc_address)?;
                let file_hash = doc.get_first(hash_field).unwrap().as_text().unwrap();

                let metadata = get_metadata_by_hash(file_hash).await?;
                println!("In iteration {} found {}", search_iteration, metadata.name);
            }
        }
    }

    let mut found_metadata = Vec::with_capacity(previous_doc_adresses.len());

    for doc_address in previous_doc_adresses {
        let doc = searcher.doc(doc_address)?;
        let file_hash = doc.get_first(hash_field).unwrap().as_text().unwrap();

        let metadata = metadata::get_metadata_by_hash(file_hash).await?;
        found_metadata.push(metadata);
    }

    Ok(found_metadata)
}

// TODO: this is more accurate but we have exponentially more searching to do...
/* #[tauri::command]
pub async fn search_content(query: &str) -> Result<Vec<Metadata>> {
    let reader = READER_INSTANCE.get().unwrap();
    let index = get_index()?;
    let schema = index.schema();

    let hash_field = schema.get_field("hash").unwrap();
    let name_field = schema.get_field("name").unwrap();
    let tags_field = schema.get_field("tags").unwrap();
    let notes_field = schema.get_field("notes").unwrap();

    let searcher = reader.searcher();

    // TODO: add searching by smart folder or all search
    // TODO: search for regex with RegexQuery (according to chillfish)
    // TODO: we can make these settings into options from the Preferences menu
    let desired_amount_of_docs: u8 = 30;
    let number_of_fuzzy_iterations: u8 = 3;
    let mut previous_doc_adresses = VecDeque::with_capacity(desired_amount_of_docs as usize);
    let query_no_extra_spaces = remove_all_extra_whitespace(query);

    println!("=================================================");

    for search_iteration in 0..number_of_fuzzy_iterations {
        for word in query_no_extra_spaces.split(' ') {
            let name_term = Term::from_field_text(name_field, word);
            let tags_term = Term::from_field_text(tags_field, word);
            let notes_term = Term::from_field_text(notes_field, word);

            let by_name = FuzzyTermQuery::new_prefix(name_term, search_iteration, true);
            let by_tag = FuzzyTermQuery::new_prefix(tags_term, search_iteration, true);
            let by_notes = FuzzyTermQuery::new_prefix(notes_term, search_iteration, true);

            let by_name_tag_notes = BooleanQuery::new(vec![
                (Occur::Should, by_name.box_clone()),
                (Occur::Should, by_tag.box_clone()),
                (Occur::Should, by_notes.box_clone()),
            ]);

            let found_documents = searcher.search(
                &by_name_tag_notes,
                &(TopDocs::with_limit(desired_amount_of_docs.into())),
            )?;

            for (.., doc_address) in found_documents {
                if !previous_doc_adresses.contains(&doc_address) {
                    previous_doc_adresses.push_back(doc_address);
                    let doc = searcher.doc(doc_address)?;
                    let file_hash = doc.get_first(hash_field).unwrap().as_text().unwrap();

                    let metadata = get_metadata_by_hash(file_hash).await?;
                    println!(
                        "For word {} in iteration {} found {}",
                        word, search_iteration, metadata.name
                    );
                }
            }
        }
    }

    let mut found_metadata = Vec::with_capacity(previous_doc_adresses.len());

    for doc_address in previous_doc_adresses {
        let doc = searcher.doc(doc_address)?;
        let file_hash = doc.get_first(hash_field).unwrap().as_text().unwrap();

        let metadata = get_metadata_by_hash(file_hash).await?;
        found_metadata.push(metadata);
    }

    Ok(found_metadata)
} */

/* fn new_multiterms_query(terms: Vec<Term>) -> BooleanQuery {
    let occur_term_queries: Vec<(Occur, Box<dyn Query>)> = terms
        .into_iter()
        .map(|term| {
            let term_query: Box<dyn Query> =
                Box::new(TermQuery::new(term, IndexRecordOption::WithFreqs));
            (Occur::Should, term_query)
        })
        .collect();
    BooleanQuery::new(occur_term_queries)
} */

#[tauri::command]
pub async fn get_metadata_by_hashes(hashes: Vec<String>) -> Result<Vec<Metadata>> {
    let mut found_metadata = Vec::with_capacity(hashes.len());

    for hash in hashes {
        let metadata = metadata::get_metadata_by_hash(hash.as_str()).await?;
        found_metadata.push(metadata);
    }

    Ok(found_metadata)
}

fn remove_all_extra_whitespace(s: &str) -> String {
    let mut prev_is_whitespace = false;
    let no_extra_whitespaces: String = s
        .chars()
        .filter(move |c| {
            let is_whitespace = c.is_whitespace();
            let ret = !is_whitespace || !prev_is_whitespace;
            prev_is_whitespace = is_whitespace;
            ret
        })
        .collect();
    no_extra_whitespaces.trim().to_owned()
}
