#![allow(dead_code)]
pub const CARRIAGE_RETURN: &str = "\r\n";
pub const DOUBLE_CARRIAGE_RETURN: &str = "\r\n\r\n";
pub const DEFAULT_PORT: &str = "7878";
pub const DEFAULT_HOST: &str = "localhost";
pub const EMPTY_STRING: &str = "";
pub const SINGLE_SPACE: &str = " ";
pub const MIDDLEWARE_LIMIT: usize = 5;
pub const HANDLER_LIMIT: usize = 4;
pub const DEBUG_FILE_PATH: &str = "./debug_file.txt";

// supported static file types
pub const CONTENT_TEXT_HTML: &str = "text/html";
pub const CONTENT_PNG: &str = "image/png";
pub const CONTENT_JPG: &str = "image/jpg";
pub const CONTENT_JPEG: &str = "image/jpeg";
pub const CONTENT_RS: &str = "testing `fail`";
