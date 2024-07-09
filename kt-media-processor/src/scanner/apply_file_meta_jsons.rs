use std::cell::RefCell;
use std::collections::HashMap;

use std::boxed::Box;
use std::rc::Rc;

use crate::model;
use std::error::Error;

use indexmap::IndexMap;

pub fn apply_file_meta_jsons(
    media_file_metas: &mut IndexMap<String, Rc<RefCell<model::MediaFileMeta>>>,
    media_file_jsons: &mut HashMap<String, Rc<RefCell<model::MediaFileMeta>>>,
) -> Result<(), Box<dyn Error>> {
    //let mut to_remove = Vec::new();

    for (path, media_file_json) in &*media_file_jsons {
        let media_file_meta = media_file_metas.get_mut(path);
        if media_file_meta.is_none() {
            continue;
        }

        let media_file_meta1 = media_file_meta.unwrap();
        let media_file_meta2 = media_file_meta1.as_ref();
        let mut media_file_meta3 = media_file_meta2.borrow_mut();

        // Override title (optional)
        let media_file_json = media_file_json.borrow();
        if let Some(title) = &media_file_json.title {
            // Since title is of type `Option<String>`, clone it to avoid moving
            media_file_meta3.title = Some(title.clone());
        }

        // Override lastmodified (optional)

        // Override ordinal (optional)
        // if let Some(ordinal) = media_album_json.ordinal {
        //     if let Entry::Occupied(mut o) = media_album_metas.entry(parent_sub_path.to_string()) {
        //         o.get_mut().borrow_mut().ordinal = Some(ordinal);
        //     }
        // }

        // Override descr (optional)
    }

    return Ok(());
}
