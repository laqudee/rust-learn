#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    NoSupport,
}

impl From<Version> for String {
    fn from(version: Version) -> Self {
        return match version {
            Version::V1_1 => "HTTP/1.1".to_string(),
            Version::V2_0 => "HTTP/2.0".to_string(),
            Version::NoSupport => "HTTP/3.0".to_string(),
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::http_response::{response_header::Version, state_code::HttpStateCode};

    #[test]
    fn version_to_string() {
        assert_eq!(String::from(Version::V1_1), "HTTP/1.1");
        assert_eq!(String::from(Version::V2_0), "HTTP/2.0");
        assert_eq!(String::from(Version::NoSupport), "HTTP/3.0");
    }

    #[test]
    fn http_state_code_text() {
        assert_eq!(String::from(HttpStateCode::StatusOK), "OK");
    }

    #[test]
    fn http_state_code_to_u16() {
        assert_eq!(u16::from(HttpStateCode::StatusOK), 200);
    }
}
