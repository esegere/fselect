use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

use app_dirs::{AppInfo, AppDataType, app_root};

const APP_INFO: AppInfo = AppInfo {
    name: "fselect",
    author: "jhspetersson",
};

const CONFIG_FILE: &str = "config.toml";

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Config {
    pub no_color : Option<bool>,
    pub gitignore: Option<bool>,
    pub hgignore: Option<bool>,
    pub dockerignore: Option<bool>,
    pub is_zip_archive : Vec<String>,
    pub is_archive : Vec<String>,
    pub is_audio : Vec<String>,
    pub is_book : Vec<String>,
    pub is_doc : Vec<String>,
    pub is_image : Vec<String>,
    pub is_source : Vec<String>,
    pub is_video : Vec<String>,
    pub default_file_size_format : Option<String>,
    #[serde(skip_serializing, default = "get_false")]
    pub debug : bool,
    #[serde(skip)]
    save : bool,
}

fn get_false() -> bool {
    false
}

impl Config {
    pub fn new() -> Result<Config, String> {
        let mut config_file;

        if let Some(cf) = Config::get_current_dir_config() {
            config_file = cf;
        } else {
            let config_dir = app_root(AppDataType::UserConfig, &APP_INFO);

            if config_dir.is_err() {
                return Ok(Config::default());
            }

            config_file = config_dir.unwrap();
            config_file.push(CONFIG_FILE);

            if !config_file.exists() {
                return Ok(Config::default());
            }
        }

        Config::from(config_file)
    }

    pub fn from(config_file: PathBuf) -> Result<Config, String> {
        if let Ok(mut file) = fs::File::open(&config_file) {
            let mut contents = String::new();
            if let Ok(_) = file.read_to_string(&mut contents) {
                match toml::from_str(&contents) {
                    Ok(config) => Ok(config),
                    Err(err) => Err(err.to_string())
                }
            } else {
                Err("Could not read config file. Using default settings.".to_string())
            }
        } else {
            Err("Could not open config file. Using default settings.".to_string())
        }
    }

    fn get_current_dir_config() -> Option<PathBuf> {
        if let Ok(mut pb) = std::env::current_exe() {
            pb.pop();
            pb.push(CONFIG_FILE);
            if pb.exists() {
                return Some(pb);
            }
        }

        None
    }

    pub fn save(&self) {
        if !self.save {
            return;
        }

        let config_dir = app_root(AppDataType::UserConfig, &APP_INFO);

        if config_dir.is_err() {
            return;
        }

        let mut config_file = config_dir.unwrap();
        config_file.push(CONFIG_FILE);

        if config_file.exists() {
            return;
        }

        let toml = toml::to_vec(&self).unwrap();

        if let Ok(mut file) = fs::File::create(&config_file) {
            let _ = file.write_all(&toml);
        }
    }

    pub fn default() -> Config {
        Config {
            no_color : Some(false),
            gitignore : Some(false),
            hgignore : Some(false),
            dockerignore : Some(false),
            is_zip_archive : vec![String::from(".zip"), String::from(".jar"), String::from(".war"), String::from(".ear")],
            is_archive : vec![String::from(String::from(".7z")), String::from(String::from(".bz2")), String::from(String::from(".bzip2")), String::from(String::from(".gz")), String::from(String::from(".gzip")), String::from(String::from(".lz")), String::from(String::from(".rar")), String::from(String::from(".tar")), String::from(".xz"), String::from(".zip")],
            is_audio : vec![String::from(".aac"), String::from(".aiff"), String::from(".amr"), String::from(".flac"), String::from(".gsm"), String::from(".m4a"), String::from(".m4b"), String::from(".m4p"), String::from(".mp3"), String::from(".ogg"), String::from(".wav"), String::from(".wma")],
            is_book : vec![String::from(".azw3"), String::from(".chm"), String::from(".djvu"), String::from(".epub"), String::from(".fb2"), String::from(".mobi"), String::from(".pdf")],
            is_doc : vec![String::from(".accdb"), String::from(".doc"), String::from(".docm"), String::from(".docx"), String::from(".dot"), String::from(".dotm"), String::from(".dotx"), String::from(".mdb"), String::from(".ods"), String::from(".odt"), String::from(".pdf"), String::from(".potm"), String::from(".potx"), String::from(".ppt"), String::from(".pptm"), String::from(".pptx"), String::from(".rtf"), String::from(".xlm"), String::from(".xls"), String::from(".xlsm"), String::from(".xlsx"), String::from(".xlt"), String::from(".xltm"), String::from(".xltx"), String::from(".xps")],
            is_image : vec![String::from(".bmp"), String::from(".gif"), String::from(".heic"), String::from(".jpeg"), String::from(".jpg"), String::from(".png"), String::from(".psb"), String::from(".psd"), String::from(".tiff"), String::from(".webp")],
            is_source : vec![String::from(".asm"), String::from(".bas"), String::from(".c"), String::from(".cc"), String::from(".ceylon"), String::from(".clj"), String::from(".coffee"), String::from(".cpp"), String::from(".cs"), String::from(".d"), String::from(".dart"), String::from(".elm"), String::from(".erl"), String::from(".go"), String::from(".groovy"), String::from(".h"), String::from(".hh"), String::from(".hpp"), String::from(".java"), String::from(".js"), String::from(".jsp"), String::from(".kt"), String::from(".kts"), String::from(".lua"), String::from(".nim"), String::from(".pas"), String::from(".php"), String::from(".pl"), String::from(".pm"), String::from(".py"), String::from(".rb"), String::from(".rs"), String::from(".scala"), String::from(".swift"), String::from(".tcl"), String::from(".vala"), String::from(".vb")],
            is_video : vec![String::from(".3gp"), String::from(".avi"), String::from(".flv"), String::from(".m4p"), String::from(".m4v"), String::from(".mkv"), String::from(".mov"), String::from(".mp4"), String::from(".mpeg"), String::from(".mpg"), String::from(".webm"), String::from(".wmv")],
            default_file_size_format : Some(String::new()),
            debug : false,
            save : true,
        }
    }
}