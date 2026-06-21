/// alias for the https status code
/// usize: status code for the repsonse,
/// str: corasponding string for status code
pub type StatusCode = (usize, &'static str);
/// number of https status codes
const N_HTTP_STATUS_CODES: usize = 59;
pub struct HttpStatusCodes {
    codes: [StatusCode; N_HTTP_STATUS_CODES],
}

#[allow(unused)]
impl HttpStatusCodes {
    const fn new() -> Self {
        HttpStatusCodes {
            codes: [
                (100, "Continue"),
                (101, "Switching Protocols"),
                (102, "Processing"),
                (103, "Early Hints"),
                (200, "OK"),
                (201, "Created"),
                (202, "Accepted"),
                (203, "Non-Authoritative Information"),
                (204, "No Content"),
                (205, "Reset Content"),
                (206, "Partial Content"),
                (207, "Multi-Status"),
                (208, "Already Reported"),
                (226, "IM Used"),
                (300, "Multiple Choices"),
                (301, "Moved Permanently"),
                (302, "Found"),
                (303, "See Other"),
                (304, "Not Modified"),
                (307, "Temporary Redirect"),
                (308, "Permanent Redirect"),
                (400, "Bad Request"),
                (401, "Unauthorized"),
                (402, "Payment Required"),
                (403, "Forbidden"),
                (404, "Not Found"),
                (405, "Method Not Allowed"),
                (406, "Not Acceptable"),
                (407, "Proxy Authentication Required"),
                (408, "Request Timeout"),
                (409, "Conflict"),
                (410, "Gone"),
                (411, "Length Required"),
                (412, "Precondition Failed"),
                (413, "Content Too Large"),
                (414, "URI Too Long"),
                (415, "Unsupported Media Type"),
                (416, "Range Not Satisfiable"),
                (417, "Expectation Failed"),
                (421, "Misdirected Request"),
                (422, "Unprocessable Content"),
                (423, "Locked"),
                (424, "Failed Dependency"),
                (425, "Too Early"),
                (426, "Upgrade Required"),
                (428, "Precondition Required"),
                (429, "Too Many Requests"),
                (431, "Request Header Fields Too Large"),
                (451, "Unavailable for Legal Reasons"),
                (500, "Internal Server Error"),
                (501, "Not Implemented"),
                (502, "Bad Gateway"),
                (503, "Service Unavailable"),
                (504, "Gateway Timeout"),
                (505, "HTTP Version Not Supported"),
                (506, "Variant Also Negotiates"),
                (507, "Insufficient Storage"),
                (508, "Loop Detected"),
                (511, "Network Authentication Required"),
            ],
        }
    }
    pub fn has(self, val: usize) -> bool {
        for (c, _) in self.codes.into_iter() {
            if c == val {
                return true;
            } else {
                continue;
            }
        }
        return false;
    }

    /// return the status code and reason based off of given status code
    pub fn get(self, code: usize) -> Result<StatusCode, String> {
        for (c, f) in self.codes.into_iter() {
            if code == c {
                return Ok((c, f));
            } else {
                continue;
            }
        }
        return Err("invalid status code was given".to_string());
    }
}

pub const HTTP_STATUS_CODES: HttpStatusCodes = HttpStatusCodes::new();
