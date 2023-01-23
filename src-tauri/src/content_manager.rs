use crate::{
    directory::{create_base_dirs, extension_from_path, get_dir_path, Dir},
    file_utils::{get_duration, get_image_dimensions, get_video_dimensions},
    foc_error::{FocError, Result},
    models::{
        metadata::{self, ContentType, Metadata},
        smart_folder::validate_folder_path,
    },
    searcher,
    web_extension::Request,
};
use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor, Seek},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use memmap2::MmapOptions;
use mime::Mime;
use sha2::{Digest, Sha256};
use thumbnailer::{create_thumbnails, ThumbnailSize};

pub async fn check_file_integrity() -> Result<()> {
    create_base_dirs()?;

    /* let all_metadata = database::get_all_metadata().await?; */

    // TODO: implement file integrity however
    // TODO: we could only verify new hashes if last_modified changed from our internal record therefore skipping hash processing
    /* all_metadata.par_iter().for_each(|metadata| {
        let path = Path::new(&metadata.path);

        if !path.exists() {
            /* let _ = database::delete_metadata(metadata.uuid.as_str()).await; */
            // TODO: handle this error somehow and display on frontend
            println!("File at path {} does not exist", &metadata.path);
        }
    }); */

    /* let (tx, rx) = channel();

    // TODO: watcher for real time
    let mut watcher = watcher(tx, Duration::from_secs(2)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch(
            "C:\\Users\\Caleidon\\Desktop\\watchme",
            RecursiveMode::Recursive,
        )
        .unwrap();

    loop {
        match rx.recv() {
            Ok(event) => println!("File changed: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    } */

    Ok(())
}

/* fn validate_file(path: &Path) {
    if !path.is_file() {
        // what if it isnt a file ?
    }
    if !path.exists() {
        // file no longer exists, mark it as deleted
    }

    // check system last modified time
} */

#[tauri::command]
pub async fn add_files(file_paths: Vec<String>) -> Result<()> {
    let mut metadata_to_index: Vec<Metadata> = Vec::with_capacity(file_paths.len());
    // TODO: this can definitely be multithreaded with rayon, also check other loops
    for file_path in file_paths {
        let content_type: ContentType;
        let extension: Option<String>;

        let file_kind_option = infer::get_from_path(&file_path)?;

        if let Some(file_kind) = file_kind_option {
            // TODO: make this extension into lowecase so its case insensitive (maybe this is automatic? test it)
            content_type = ContentType::from_kind(&file_kind)?;
            extension = Some(file_kind.extension().to_string());
        } else {
            content_type = ContentType::Other;
            extension = extension_from_path(&file_path);
        }

        let file = File::open(&file_path)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };

        let file_hash: String = Sha256::digest(&mmap)
            .to_vec()
            .iter()
            .map(ToString::to_string)
            .collect();

        let file_name_no_ext = Path::new(&file_path)
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .into_owned();

        let width_and_height: Option<(i32, i32)> = match content_type {
            ContentType::Image => get_image_dimensions(&file_path),
            ContentType::Video => get_video_dimensions(&file_path),
            _ => None,
        };

        let duration = match content_type {
            ContentType::Audio => get_duration(&file_path),
            _ => None,
        };

        let timestamp_created = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as i32;

        let metadata = Metadata {
            hash: file_hash,
            name: file_name_no_ext,
            path: file_path,
            content_type,
            status: metadata::Status::Valid,
            timestamp_created,
            timestamp_modified: timestamp_created,
            extension,
            tags: None,
            notes: None,
            width: width_and_height.map(|(width, _height)| width),
            height: width_and_height.map(|(_width, height)| height),
            duration,
        };

        let metadata_for_database = metadata.clone();
        metadata::insert_metadata(metadata_for_database).await?;

        generate_thumbnail_from_file(&metadata, &file)?;
        metadata_to_index.push(metadata);
    }

    searcher::index_metadata(&metadata_to_index)?;

    Ok(())
}

#[tauri::command]
pub async fn delete_content(hashes: Vec<String>) -> Result<()> {
    metadata::delete_metadata(&hashes).await?;
    searcher::delete_metadata(&hashes)?;

    // TODO: deletion could actually delete files on disk as an option
    /* fs::remove_file(hashes.path)?; */

    Ok(())
}

#[tauri::command]
pub async fn update_content(metadata: Metadata) -> Result<()> {
    println!("Updating {:?}", metadata.name);
    searcher::update_metadata(&metadata)?;
    metadata::update_metadata(metadata).await?;

    Ok(())
}

pub async fn add_url_from_extension(request: &Request<'_>) -> Result<()> {
    // TODO: we gotta make sure it's a proper website (perhaps theres some check for this) and not edge://

    let url_hash: String = Sha256::digest(&request.url)
        .to_vec()
        .iter()
        .map(ToString::to_string)
        .collect();

    let base64 = request
        .image
        .strip_prefix("data:image/png;base64,")
        .ok_or_else(|| FocError::Metadata("Couldn't strip prefix!".to_owned()))?;

    let reader = Cursor::new(base64::decode(base64)?);
    let width_and_height = create_thumbnail(url_hash.as_str(), reader, mime::IMAGE_PNG)?;

    let timestamp_created = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i32;

    let metadata = Metadata {
        hash: url_hash,
        name: request.name.to_owned(),
        path: request.url.to_owned(),
        content_type: ContentType::Link,
        status: metadata::Status::Valid,
        timestamp_created,
        timestamp_modified: timestamp_created,
        extension: None,
        tags: None,
        notes: None,
        width: Some(width_and_height.0),
        height: Some(width_and_height.1),
        duration: None,
    };

    let metadata_for_database = metadata.clone();

    metadata::insert_metadata(metadata_for_database).await?;
    searcher::index_metadata(&[metadata])?;

    // TODO: here we want to tell the extension that we added a new content and open a popup to ask the user if they wanna focus it

    Ok(())
}

fn generate_thumbnail_from_file(metadata: &Metadata, file: &File) -> Result<()> {
    if let ContentType::Image = metadata.content_type {
        let image_type = match metadata.extension.as_deref() {
            Some("png") => mime::IMAGE_PNG,
            Some("jpg") => mime::IMAGE_JPEG,
            Some("bmp") => mime::IMAGE_BMP,
            Some("webp") => mime::IMAGE_PNG,
            Some("gif") => return Ok(()),
            _ => {
                return Err(FocError::Metadata(
                    "Unsupported image type detected while generating thumbnail".to_owned(),
                ))
            }
        };

        let reader = BufReader::new(file);
        create_thumbnail(metadata.hash.as_str(), reader, image_type)?;
    }

    Ok(())
}

fn create_thumbnail<R>(name: &str, reader: R, image_type: Mime) -> Result<(i32, i32)>
where
    R: BufRead + Seek,
{
    let mut thumbnails = create_thumbnails(reader, image_type, [ThumbnailSize::Large])?;
    let thumbnail_raw = thumbnails.pop().unwrap();
    let mut thumbnail_buf = Cursor::new(Vec::new());
    thumbnail_raw.write_png(&mut thumbnail_buf)?;

    let thumbnail =
        image::load_from_memory_with_format(thumbnail_buf.get_ref(), image::ImageFormat::Png)
            .unwrap();

    let name = format!("{}.png", name);
    let thumbnail_dir = get_dir_path(Dir::Thumbnails)?.join(name);
    thumbnail.save(&thumbnail_dir)?;

    Ok((thumbnail.width() as i32, thumbnail.height() as i32))
}

#[tauri::command]
pub async fn get_all_tags() -> Result<Vec<String>> {
    let tags_raw = metadata::get_all_tags().await?;
    let mut all_tags: Vec<String> = tags_raw.into_iter().flatten().flatten().collect();
    all_tags.sort_unstable();
    all_tags.dedup();

    Ok(all_tags)
}
