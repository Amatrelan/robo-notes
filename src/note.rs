use std::io::Write;

use rand::distributions::DistString;

pub type Tag = String;

#[derive(Debug)]
pub struct NoteName {
    base: String,
    random: String,
    extension: String,
}

impl ToString for NoteName {
    #[tracing::instrument]
    fn to_string(&self) -> String {
        format!("{}_{}.{}", self.base, self.random, self.extension)
    }
}

impl NoteName {
    #[tracing::instrument]
    pub fn new(name: String) -> Self {
        tracing::info!("Creating new NoteName");
        let r = rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 7);

        Self {
            base: name,
            random: r,
            extension: "md".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Note {
    name: NoteName,
    pub content: Option<String>,
    tags: Option<Vec<Tag>>,
}

impl Note {
    #[tracing::instrument]
    pub fn new(name: &String, content: &Option<String>, tags: &Option<Vec<Tag>>) -> Self {
        tracing::info!("Creating new Note");
        let name = NoteName::new(name.clone());
        Self {
            name,
            content: content.clone(),
            tags: tags.clone(),
        }
    }

    pub fn get_tmp_path(&self) -> color_eyre::Result<std::path::PathBuf> {
        let mut p = std::path::PathBuf::from("/tmp")
            .join(env!("CARGO_PKG_NAME"))
            .join(self.name.to_string());

        let a = p.clone();
        let file_name = a.file_name().clone().unwrap();
        // Remove filename and extension path
        p.pop();

        tracing::debug!("Creating tmp dir all: {}", p.display());
        std::fs::create_dir_all(&p)?;

        Ok(p.join(file_name))
    }

    pub fn save(&self, root: &std::path::PathBuf) -> color_eyre::Result<()> {
        let path = root.join(self.name.to_string());

        // This is here so if user gives path as `some/asd.md` it also creates that `some` folder
        std::fs::create_dir_all(&path.parent().unwrap())?;
        tracing::info!("Saving file to {:?}", path);

        let mut file = std::fs::File::create(path)?;
        if let Some(content) = &self.content {
            file.write_all(content.as_bytes())?;
        } else {
            return Err(color_eyre::eyre::eyre!("File has no content"));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use pretty_assertions::assert_ne;

    use super::*;

    #[test]
    fn test_name() {
        let name = NoteName::new("example".to_string());

        assert_eq!(name.base, "example".to_string());
        assert_eq!(name.extension, "md".to_string());
        assert_ne!(
            name.to_string(),
            "example.md".to_string(),
            "NoteName.to_string() should not never be equal to default values without random"
        );
    }
}
