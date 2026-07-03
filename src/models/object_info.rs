use crate::Result;
use iocore::Path;
use serde::{Deserialize, Serialize};
use serde_json::{Value, to_string_pretty};
use slugify_filenames::slugify_string;
use std::{
    fmt::{Debug, Display},
    iter::IntoIterator,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectInfo<
    T: Serialize + Clone + Debug + Display,
    // I: IntoIterator<Item = T, IntoIter = std::iter::Iterator<Item = T>>,
> {
    pub value: T,
    // pub versions: I,
}
impl<T> ObjectInfo<T>
where
    T: Serialize + Clone + Debug + Display,
{
    pub fn save(&self, path: Option<Path>) -> Result<(Path, String)> {
        let path = path.unwrap_or_else(|| {
            let slug = slugify_string(&self.value).unwrap_or_default();
            Path::new(format!("{slug}.json"))
        });
        let json_string = to_string_pretty(&self.value)?;
        let json = json_string.as_bytes().to_vec();
        path.write(&json)?;
        Ok((path, json_string))
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct ObjectInfo<T: Serialize + Clone + Debug + Display> {
//     value: T,
//     versions: Value,
// }
// impl ObjectInfo {
//     pub fn save(&self, path: Option<Path>) -> Result<(Path, String)> {
//         let path = path.unwrap_or_else(|| {
//             let slug = slugify_string(&self.request.url()).unwrap_or_default();
//             Path::new(format!("{slug}.json"))
//         });
//         let json_string = serde_json::to_string_pretty(&self.value)?;
//         let json = json_string.as_bytes().to_vec();
//         path.write(&json)?;
//         Ok((path, json_string))
//     }
// }
