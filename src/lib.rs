#![deny(warnings)]

extern crate hyper;

use std::fmt;
use hyper::header::{parsing, Header, HeaderFormat};


/// The `Icy-MetaData` header.
///
/// The `Icy-MetaData` header is used to tell a server we want to receive icy
/// metadata in the stream.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IcyMetaData(pub bool);

impl Header for IcyMetaData {
    fn header_name() -> &'static str {
        "Icy-MetaData"
    }

    fn parse_header(raw: &[Vec<u8>]) -> Option<Self> {
        let parsed: Option<u32> = parsing::from_one_raw_str(raw);
        parsed.and_then(|num| Some(IcyMetaData(num > 0)))
    }
}

impl HeaderFormat for IcyMetaData {
    fn fmt_header(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match *self {
            IcyMetaData(false) => "0",
            IcyMetaData(true) => "1",
        })
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod IcyMetaData {
    mod tests {
        use hyper::header::{Header, HeaderFormatter};
        use super::super::IcyMetaData;

        #[test]
        fn test_format_header_disabled() {
            let header = IcyMetaData(false);
            let formatted = format!("{}", HeaderFormatter(&header));
            assert_eq!(formatted, "0");
        }

        #[test]
        fn test_format_header_enabled() {
            let header = IcyMetaData(true);
            let formatted = format!("{}", HeaderFormatter(&header));
            assert_eq!(formatted, "1");
        }

        #[test]
        fn test_parse_header_negative() {
            let header: Option<IcyMetaData> = Header::parse_header(&[b"-33".to_vec()]);
            if let Some(header) = header {
                assert!(false, "got value for negative: {:?}", header);
            }
        }

        #[test]
        fn test_parse_header_zero_is_disabled() {
            let a: IcyMetaData = Header::parse_header(&[b"0".to_vec()]).unwrap();
            assert_eq!(a, IcyMetaData(false));
        }

        #[test]
        fn test_parse_header_nonzero_is_enabled() {
            let a: IcyMetaData = Header::parse_header(&[b"1".to_vec()]).unwrap();
            assert_eq!(a, IcyMetaData(true));

            let a: IcyMetaData = Header::parse_header(&[b"2".to_vec()]).unwrap();
            assert_eq!(a, IcyMetaData(true));

            let a: IcyMetaData = Header::parse_header(&[b"1234".to_vec()]).unwrap();
            assert_eq!(a, IcyMetaData(true));
        }
    }
}


/// The `icy-metaint` header.
///
/// The `icy-metaint` header is used to tell a client the interval at which
/// icy metadata is inserted.  ICY metadata will be inserted after this number
/// of bytes.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IcyMetaInt(pub usize);

impl Header for IcyMetaInt {
    fn header_name() -> &'static str {
        "icy-metaint"
    }

    fn parse_header(raw: &[Vec<u8>]) -> Option<Self> {
        parsing::from_one_raw_str(raw).map(IcyMetaInt)
    }
}

impl HeaderFormat for IcyMetaInt {
    fn fmt_header(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let IcyMetaInt(num) = *self;
        write!(fmt, "{}", num)
    }
}


#[allow(non_snake_case)]
#[cfg(test)]
mod IcyMetaInt {
    mod tests {
        use hyper::header::{Header, HeaderFormatter};
        use super::super::IcyMetaInt;

        #[test]
        fn test_format_header() {
            let header = IcyMetaInt(0);
            let formatted = format!("{}", HeaderFormatter(&header));
            assert_eq!(formatted, "0");

            let header = IcyMetaInt(1);
            let formatted = format!("{}", HeaderFormatter(&header));
            assert_eq!(formatted, "1");

            let header = IcyMetaInt(1234);
            let formatted = format!("{}", HeaderFormatter(&header));
            assert_eq!(formatted, "1234");
        }

        #[test]
        fn test_parse_header_negative() {
            let header: Option<IcyMetaInt> = Header::parse_header(&[b"-33".to_vec()]);
            if let Some(header) = header {
                assert!(false, "got value for negative: {:?}", header);
            }
        }

        #[test]
        fn test_parse_header_zero() {
            let a: IcyMetaInt = Header::parse_header(&[b"0".to_vec()]).unwrap();
            assert_eq!(a, IcyMetaInt(0));
        }

        #[test]
        fn test_parse_header_nonzero() {
            let a: IcyMetaInt = Header::parse_header(&[b"1".to_vec()]).unwrap();
            assert_eq!(a, IcyMetaInt(1));

            let a: IcyMetaInt = Header::parse_header(&[b"2".to_vec()]).unwrap();
            assert_eq!(a, IcyMetaInt(2));

            let a: IcyMetaInt = Header::parse_header(&[b"1234".to_vec()]).unwrap();
            assert_eq!(a, IcyMetaInt(1234));
        }
    }
}
