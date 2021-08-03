use std::path::PathBuf;

pub enum BufferSource {
    FilePath(PathBuf),
    Internal(String)
}
