#[derive(Debug)]
pub struct SourceCodeLocation {
    file_name: &'static str,
    line_number: u32,
    column_number: u32,
}

impl std::fmt::Display for SourceCodeLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.format())
    }
}

impl SourceCodeLocation {
    pub fn new(file_name: &'static str, line_number: u32, column_number: u32) -> Self {
        Self {
            file_name,
            line_number,
            column_number,
        }
    }
    pub fn file_name(&self) -> &'static str {
        self.file_name
    }
    pub fn line_number(&self) -> u32 {
        self.line_number
    }
    pub fn column_number(&self) -> u32 {
        self.column_number
    }
    fn format(&self) -> String {
        format!(
            "[{}] {}:{}",
            self.file_name, self.line_number, self.column_number
        )
    }
}

#[macro_export]
macro_rules! new {
    () => {
        source_code_location::SourceCodeLocation::new(file!(), line!(), column!())
    };
}

#[macro_export]
macro_rules! new_string {
    () => {
        concat!("[", file!(), "]", " ", line!(), ":", column!())
    };
}
