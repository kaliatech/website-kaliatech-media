use std::cell::RefCell;

use std::collections::HashMap;

use std::boxed::Box;
use std::rc::Rc;

use crate::model;
use std::error::Error;
use std::io;

use indexmap::IndexMap;

pub fn apply_album_meta_jsons(
    media_album_metas: &mut IndexMap<String, Rc<RefCell<model::MediaAlbumMeta>>>,
    media_album_jsons: &mut HashMap<String, Rc<RefCell<model::MediaAlbumMeta>>>,
) -> Result<(), Box<dyn Error>> {
    //let mut to_remove = Vec::new();

    for (path, media_album_json) in &*media_album_jsons {
        let parent_sub_path = path.trim_end_matches("/album-meta.json");

        let album_meta = media_album_metas
            .get(&parent_sub_path.to_string())
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Album meta not found"))?;

        let album_json = media_album_json.borrow();

        //Override title (optional)
        if let Some(title) = &album_json.title {
            album_meta.borrow_mut().title = Some(title.clone());
        }

        // Override lastmodified (optional)

        // Override ordinal (optional)
        if let Some(ordinal) = &album_json.ordinal {
            album_meta.borrow_mut().ordinal = Some(*ordinal);
        }

        // Override descr (optional)
    }

    return Ok(());
}
