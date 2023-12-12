use std::cell::RefCell;

use std::io::Cursor;
use std::path::PathBuf;
use std::rc::Rc;

use msfs::network::*;

use crate::dispatcher::{Dispatcher, Task, TaskStatus};
use crate::download::zip_handler::{BatchReturn, ZipFileHandler};

pub struct DownloadOptions {
    batch_size: usize,
}

#[derive(PartialEq, Eq, Clone)]
pub enum DownloadStatus {
    NoDownload,
    Downloading,
    CleaningDestination,
    Extracting,
    Failed(String),
}

pub struct NavdataDownloader {
    zip_handler: RefCell<Option<ZipFileHandler<Cursor<Vec<u8>>>>>,
    download_status: RefCell<DownloadStatus>,
    options: RefCell<DownloadOptions>,
    task: RefCell<Option<Rc<RefCell<Task>>>>,
}

impl NavdataDownloader {
    pub fn new() -> Self {
        NavdataDownloader {
            zip_handler: RefCell::new(None),
            download_status: RefCell::new(DownloadStatus::NoDownload),
            options: RefCell::new(DownloadOptions { batch_size: 10 }), // default batch size
            task: RefCell::new(None),
        }
    }

    pub fn on_update(&self) {
        // Check if we failed and need to send an error message
        if let Some(message) = self.check_failed_and_get_message() {
            self.report_error(message);
            self.reset_download();
            return;
        }

        if self.should_extract_next_batch() {
            match self.unzip_batch(self.options.borrow().batch_size) {
                Ok(BatchReturn::Finished) => {
                    println!("[WASM] Finished extracting");
                    // Scope to drop the borrow so we can reset the download
                    {
                        let borrowed_task = self.task.borrow();
                        if (*borrowed_task).is_none() {
                            println!("[WASM] Request is none");
                            return;
                        }
                        let mut borrowed_task = borrowed_task.as_ref().unwrap().borrow_mut();
                        borrowed_task.status = TaskStatus::Success(None);
                    }

                    self.reset_download();
                }
                Ok(BatchReturn::MoreFilesToDelete) => {
                    self.download_status
                        .replace(DownloadStatus::CleaningDestination);

                    let borrowed_zip_handler = self.zip_handler.borrow();
                    if let Some(zip_handler) = borrowed_zip_handler.as_ref() {
                        self.send_progress_update(Some(zip_handler.deleted), None, None);
                    }
                }
                Ok(BatchReturn::MoreFilesToUnzip) => {
                    self.download_status.replace(DownloadStatus::Extracting);

                    let borrowed_zip_handler = self.zip_handler.borrow();
                    if let Some(zip_handler) = borrowed_zip_handler.as_ref() {
                        self.send_progress_update(
                            None,
                            Some(zip_handler.zip_file_count),
                            Some(zip_handler.current_file_index),
                        );
                    }
                }
                Err(e) => {
                    println!("[WASM] Failed to unzip: {}", e);
                    self.report_error(e.to_string());
                    self.reset_download();
                }
            }
        }
    }

    pub fn set_download_options(self: &Rc<Self>, task: Rc<RefCell<Task>>) {
        {
            let json = task.borrow().data.clone();
            // Get batch size, if it fails to parse then just return
            let batch_size = match json["batchSize"].as_u64() {
                Some(batch_size) => batch_size as usize,
                None => return,
            };

            // Set the options (only batch size for now)
            let mut options = self.options.borrow_mut();
            options.batch_size = batch_size;
        }
        task.borrow_mut().status = TaskStatus::Success(None);
    }

    /// Starts the download process
    pub fn download(self: &Rc<Self>, task: Rc<RefCell<Task>>) {
        // Silently fail if we are already downloading (maybe we should send an error message?)
        if *self.download_status.borrow() == DownloadStatus::Downloading {
            println!("[WASM] Already downloading");
            return;
        } else {
            println!("[WASM] Downloading");
            self.download_status.replace(DownloadStatus::Downloading);
            self.send_progress_update(None, None, None);
        }
        self.task.borrow_mut().replace(task.clone());

        let json = task.borrow().data.clone();

        let url = json["url"].as_str().unwrap_or_default().to_owned();

        // Check if json has "folder"
        let folder = json["folder"].as_str().unwrap_or_default().to_owned();

        // Make sure we have both a url and a folder
        if url.is_empty() || folder.is_empty() {
            self.download_status.replace(DownloadStatus::Failed(
                "No url or folder provided".to_string(),
            ));
            return;
        }

        // Create the request
        let captured_self = self.clone();
        println!("[WASM] Creating request");
        match NetworkRequestBuilder::new(&url)
            .unwrap()
            .with_callback(move |network_request, status_code| {
                captured_self.request_finished_callback(network_request, status_code, folder)
            })
            .get()
        {
            Some(_) => (),
            None => {
                self.download_status.replace(DownloadStatus::Failed(
                    "Failed to create request".to_string(),
                ));
            }
        }
    }

    /// Sends a status update to the client
    fn send_progress_update(
        &self,
        deleted: Option<usize>,
        total_to_unzip: Option<usize>,
        unzipped: Option<usize>,
    ) {
        let status = self.download_status.borrow();
        let data = serde_json::json!({
            // Match enum on the JS side (this is bad)
            "phase": match *status {
                DownloadStatus::Downloading => 0,
                DownloadStatus::CleaningDestination => 1,
                DownloadStatus::Extracting => 2,
                _ => return, // Don't send an update if we are not downloading (this should never happen)
            },
            "deleted": deleted,
            "total_to_unzip": total_to_unzip,
            "unzipped": unzipped,
        });
        Dispatcher::send_event("DownloadProgress", Some(data));
    }

    fn request_finished_callback(&self, request: NetworkRequest, status_code: i32, folder: String) {
        // Fail if the status code is not 200
        if status_code != 200 {
            self.download_status.replace(DownloadStatus::Failed(format!(
                "Failed to download file. Status code: {}",
                status_code
            )));
            return;
        }

        let path = PathBuf::from(format!("\\work/{}", folder));

        // Check the data from the request
        let data = request.data();
        if data.is_none() {
            self.download_status
                .replace(DownloadStatus::Failed("No data received".to_string()));
            return;
        }
        // Extract the data from the request (safe to unwrap since we already checked if data was none)
        let data = data.unwrap();
        let cursor = Cursor::new(data);
        let zip = zip::ZipArchive::new(cursor);
        if zip.is_err() {
            self.download_status.replace(DownloadStatus::Failed(format!(
                "Failed to create zip archive: {}",
                zip.err().unwrap()
            )));
            return;
        }
        // Unwrap is safe since we already checked if it was an error
        let zip = zip.unwrap();

        // Create the zip handler
        let handler = ZipFileHandler::new(zip, path);
        let mut zip_handler = self.zip_handler.borrow_mut();
        *zip_handler = Some(handler);
    }

    pub fn unzip_batch(
        &self,
        batch_size: usize,
    ) -> Result<BatchReturn, Box<dyn std::error::Error>> {
        let mut zip_handler = self.zip_handler.borrow_mut();

        let handler = zip_handler
            .as_mut()
            .ok_or_else(|| "Zip handler not found".to_string())?;
        let res = handler.unzip_batch(batch_size)?;

        Ok(res)
    }

    pub fn reset_download(&self) {
        // Use the take method to replace the current value with None and drop the old value.
        self.zip_handler.borrow_mut().take();

        // Clear our task
        self.task.replace(None);
    }

    fn check_failed_and_get_message(&self) -> Option<String> {
        let borrowed_status = self.download_status.borrow();
        if let DownloadStatus::Failed(ref message) = *borrowed_status {
            Some(message.clone())
        } else {
            None
        }
    }

    fn report_error(&self, message: String) {
        let borrowed_task = self.task.borrow();
        if (*borrowed_task).is_none() {
            println!("[WASM] Task is none");
            return;
        }
        let mut borrowed_task = borrowed_task.as_ref().unwrap().borrow_mut();
        borrowed_task.status = TaskStatus::Failure(message.clone());
    }

    fn should_extract_next_batch(&self) -> bool {
        let borrowed_zip_handler = self.zip_handler.borrow();
        if let Some(zip_handler) = borrowed_zip_handler.as_ref() {
            zip_handler.zip_file_count > zip_handler.current_file_index
        } else {
            // If there is no zip handler, we are not downloading and we don't need to do anything
            false
        }
    }
}
