#[derive(Debug)]
pub enum FileArg {
    Defaulted(String),
    Mapped(String, String),
}

impl FileArg {
    pub fn new(src: String) -> Self {
        match src.find(':') {
            Some(index) => {
                let (from, to) = src.split_at(index);

                Self::Mapped(from.to_string(), to[1..].to_string())
            }
            None => Self::Defaulted(src),
        }
    }
    pub fn get_read(&self) -> String {
        match self {
            FileArg::Defaulted(default) => default.clone(),
            FileArg::Mapped(from, _) => from.clone(),
        }
    }
    pub fn get_write(&self, fallback_ext: &str) -> String {
        match self {
            FileArg::Defaulted(default) => {
                let mut val = default.clone();
                val.push_str(fallback_ext);
                val
            }
            FileArg::Mapped(_, to) => to.clone(),
        }
    }
}
