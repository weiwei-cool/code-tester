use std::path::{Path, PathBuf};
use std::{env, fs};

pub trait PreparationMethod {
    fn get_type(&self) -> FileType;
    fn get_content(&self) -> Result<String, String>;
    fn get_file_name(&self) -> String;
    fn get_file_path(&self) -> String;
}

#[derive(Clone)]
pub enum FileType {
    Text,
    Binary,
    Python,
}

pub struct File{
    path: String,
    file_name: String,
    file_type: Option<FileType>,
    content: Option<String>,
}

impl File{
    pub fn new(path:&String, file_type: FileType) -> Self {
        let file_path = Path::new(&path);
        if !file_path.exists() {
            panic!("File does not exist!")
        }
        let path = path.to_string();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        match file_type {
            FileType::Text => {
                let text = fs::read(&path).unwrap();
                let text = String::from_utf8(text).expect("Can't parse text file!");
                let file_name = file_name.to_string();
                Self {
                    path,
                    file_name,
                    file_type: Some(file_type),
                    content: Some(text),
                }
            }
            _ => {
                let file_name = file_name.to_string();
                Self {
                    path,
                    file_name,
                    file_type: Some(file_type),
                    content: None,
                }
            }
        }
    }
    pub fn form_string(context: String) -> Self {
        Self {
            path: "".to_string(),
            file_name: "".to_string(),
            file_type: Option::from(FileType::Text),
            content: Option::from(context),
        }
    }
}

impl PreparationMethod for File{
    fn get_type(&self) -> FileType {
        self.file_type.clone().unwrap()
    }

    fn get_content(&self) -> Result<String, String> {
        match self.file_type.clone().unwrap() {
            FileType::Binary => {
                Err("".to_string())
            }
            _ => {
                Ok(self.content.clone().unwrap().to_string())
            }
        }
    }

    fn get_file_name(&self) -> String {
        self.file_name.clone()
    }

    fn get_file_path(&self) -> String {
        let current_dir = env::current_dir().expect("无法获取当前目录");
        let absolute_path = if Path::new(&self.path).is_absolute() {
            PathBuf::from(&self.path)
        } else {
            let mut abs_path = current_dir;
            abs_path.push(&self.path);
            abs_path
        };
        absolute_path.to_str().unwrap().to_string()
    }
}