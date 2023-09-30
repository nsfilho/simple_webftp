use super::helpers::rocket::ApiError;
use super::Args;
use chrono::prelude::*;
use rocket::{form::Form, fs::TempFile, serde::json::Json, serde::Serialize, State};
use std::path::Path;
use tracing::info;

#[derive(FromForm)]
pub struct FileUploadForm<'f> {
    pub file: TempFile<'f>,
}

#[derive(Serialize, Debug)]
pub struct FileUploadedSucessfully {
    pub file_name: String,
}

// take the extension of the file TempFile
fn get_extension(file: &TempFile) -> Option<String> {
    if let Some(file_name) = file.raw_name() {
        let file_name = String::from(file_name.dangerous_unsafe_unsanitized_raw().as_str());
        let extension = Path::new(&file_name).extension();
        if let Some(extension) = extension {
            return Some(extension.to_string_lossy().to_string());
        }
    }
    None
}

/** Route to upload files */
#[post("/upload", format = "multipart/form-data", data = "<form>")]
pub async fn file_upload(
    mut form: Form<FileUploadForm<'_>>,
    args: &State<Args>,
) -> Result<Json<FileUploadedSucessfully>, ApiError> {
    if let Some(file_name) = form.file.name() {
        if file_name.len() == 0 {
            return Err(ApiError {
                message: "Error uploading file".to_string(),
                error: "No file name, because is empty".to_string(),
            });
        }

        let file_name = Path::new(&args.folder).join(format!(
            "{}.{}",
            file_name,
            get_extension(&form.file).unwrap_or("".to_string())
        ));

        info!(
            "Uploading: destination = {}, len: {}",
            file_name.to_string_lossy(),
            form.file.len()
        );

        let result = form.file.persist_to(file_name.clone()).await;
        match result {
            Ok(_) => Ok(Json(FileUploadedSucessfully {
                file_name: file_name.to_string_lossy().to_string(),
            })),
            Err(e) => {
                return Err(ApiError {
                    message: e.to_string(),
                    error: "Error uploading file".to_string(),
                });
            }
        }
    } else {
        return Err(ApiError {
            message: "Error uploading file".to_string(),
            error: "No file name, because it is not informed".to_string(),
        });
    }
}

#[derive(Serialize, Debug)]
pub struct FileListSucessfullyInfo {
    name: String,
    size: u64,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "modifiedAt")]
    modified_at: String,
}

/** Route to return a list of files */
#[get("/list")]
pub async fn file_list(args: &State<Args>) -> Result<Json<Vec<FileListSucessfullyInfo>>, ApiError> {
    let mut result: Vec<FileListSucessfullyInfo> = vec![];
    if let Ok(mut entries) = rocket::tokio::fs::read_dir(args.folder.to_owned()).await {
        while let Ok(entry) = entries.next_entry().await {
            if let Some(entry) = entry {
                if let Ok(meta_data) = entry.metadata().await {
                    let created_at: DateTime<Utc> = meta_data.created().unwrap().into();
                    let modified_at: DateTime<Utc> = meta_data.modified().unwrap().into();
                    result.push(FileListSucessfullyInfo {
                        name: entry.file_name().to_string_lossy().to_string(),
                        size: meta_data.len(),
                        created_at: created_at.format("%+").to_string(),
                        modified_at: modified_at.format("%+").to_string(),
                    });
                }
            } else {
                break;
            }
        }
    } else {
        return Err(ApiError {
            message: format!("Error reading folder: {}", args.folder.to_string_lossy()),
            error: "Error reading folder".to_string(),
        });
    }
    Ok(Json(result))
}
