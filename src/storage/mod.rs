use std::fs::File;
use std::path::{PathBuf};
use uuid::Uuid;

pub struct Storage {
    name: String
}

impl Storage {
    fn new(name: &'static str) -> Storage {
        Storage {
            name: String::from(name)
        }
    }

    fn app_dir(&self) -> std::io::Result<PathBuf> {
        let mut buf = PathBuf::new();
        if cfg!(windows) {
            let env = std::env::var("APPDATA").unwrap();
            buf.push(env);
            buf.push(self.name.clone());
        } else if cfg!(unix) {
            buf.push("/var/lib");
            buf.push(self.name.clone());
        }

        std::fs::create_dir_all(&buf)?;

        Ok(buf)
    }

    fn temp_dir(&self) -> std::io::Result<PathBuf> {
        let mut tmp = std::env::temp_dir();
        tmp.push(self.name.clone());

        std::fs::create_dir_all(&tmp)?;

        Ok(tmp)
    }

    fn temp_file(&self, extension: Option<&'static str>) -> std::io::Result<File> {
        let mut temp_dir = self.temp_dir()?;

        let id = base64::encode(Uuid::new_v4().to_string());

        let name = if extension.is_none() {
            format!("tmp_{}", id)
        } else {
            format!("tmp_{}.{}", id, extension.unwrap())
        };
        temp_dir.push(name);

        println!("{}", temp_dir.display());

        File::create(temp_dir)
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::Storage;

    #[test]
    fn returns_correct_app_dir() {
        let storage = Storage::new("test");
        let path = storage.app_dir().unwrap();
        println!("{}", path.display());
    }

    #[test]
    fn creates_temp_file() {
        let storage = Storage::new("test");
        let _tmp_file = storage.temp_file(Some("txt")).unwrap();
    }
}