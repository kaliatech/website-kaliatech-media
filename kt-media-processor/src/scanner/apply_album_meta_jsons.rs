use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use std::boxed::Box;
use std::rc::Rc;

use crate::model;
use std::error::Error;

pub fn apply_album_meta_jsons(
    media_album_metas: &mut HashMap<String, Rc<RefCell<model::MediaAlbumMeta>>>,
    media_album_jsons: &mut HashMap<String, Rc<RefCell<model::MediaAlbumMeta>>>,
) -> Result<(), Box<dyn Error>> {
    //let mut to_remove = Vec::new();

    for (path, media_album_json) in &*media_album_jsons {
        // let formatted_path = format!(
        //     "{}{}",
        //     input_path.to_str().unwrap_or_default(),
        //     path.as_str()
        // );
        //let file_path = Path::new(&formatted_path);
        //let parent_path = format!("/{}", &file_path.parent().unwrap().to_string_lossy());

        let parent_sub_path = path.trim_end_matches("/album-meta.json");

        // let album_meta_overrides: model::MediaAlbumMeta = read_object_from_json_file(file_path)?;
        // let album_meta = media_album_metas.entry(parent_sub_path.to_string());

        // Override total (optional)
        let media_album_json = media_album_json.borrow();
        if let Some(title) = media_album_json.title.clone() {
            if let Entry::Occupied(mut o) = media_album_metas.entry(parent_sub_path.to_string()) {
                o.get_mut().borrow_mut().title = Some(title);
            }
        }

        // Override lastmodified (optional)

        // Override ordinal (optional)
        if let Some(ordinal) = media_album_json.ordinal {
            if let Entry::Occupied(mut o) = media_album_metas.entry(parent_sub_path.to_string()) {
                o.get_mut().borrow_mut().ordinal = Some(ordinal);
            }
        }

        // Override descr (optional)
    }

    return Ok(());
}
