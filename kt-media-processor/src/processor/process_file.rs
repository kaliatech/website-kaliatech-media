use std::error::Error;
use std::path::Path;

use std::fs;
use std::io::Write;

use crate::model;
use std::cell::Ref;
use std::cell::RefCell;
use std::rc::Rc;

pub fn process_file(
    file_src_path: &Path,
    output_path: &Path,
    media_file_meta: Ref<model::MediaFileMeta>,
    media_file: Ref<model::MediaFile>,
) -> Result<(), Box<dyn Error>> {
    //todo!();

    let file_500x500_path = output_path.join(format!(
        "{}.500x500.jpg",
        &media_file
            .path
            .strip_prefix("/")
            .unwrap_or(&media_file.path)
    ));
    dbg!(file_src_path.to_str());
    dbg!(file_500x500_path.to_str());
    return Ok(());
}
