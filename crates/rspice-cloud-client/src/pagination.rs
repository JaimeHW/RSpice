use std::{error::Error, fmt};

use rspice_cloud_contract::Page;

const DEFAULT_PAGE_SIZE: u8 = 50;
const MAX_PAGE_SIZE: usize = 100;
const MAX_CURSOR_BYTES: usize = 256;

pub(crate) fn valid_page_shape<T>(page: &Page<T>, requested_limit: u8) -> bool {
    valid_page_parts(
        page.items.len(),
        page.next_cursor.as_deref(),
        requested_limit,
    )
}

pub(crate) fn valid_page_parts(
    item_count: usize,
    next_cursor: Option<&str>,
    requested_limit: u8,
) -> bool {
    item_count <= usize::from(requested_limit) && next_cursor.is_none_or(valid_cursor)
}

fn valid_cursor(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_CURSOR_BYTES
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
}

/// A validated request for one cursor-paginated collection page.
#[derive(Clone, Copy)]
pub struct PageRequest<'a> {
    limit: u8,
    cursor: Option<&'a str>,
}

impl<'a> PageRequest<'a> {
    /// Creates a page request with a 1 through 100 item limit.
    pub fn new(limit: usize, cursor: Option<&'a str>) -> Result<Self, PaginationError> {
        if limit == 0 || limit > MAX_PAGE_SIZE {
            return Err(PaginationError::InvalidLimit);
        }
        if let Some(value) = cursor
            && !valid_cursor(value)
        {
            return Err(PaginationError::InvalidCursor);
        }
        Ok(Self {
            limit: u8::try_from(limit).expect("validated page limits fit in u8"),
            cursor,
        })
    }

    /// Creates the first page of a collection.
    pub fn first(limit: usize) -> Result<Self, PaginationError> {
        Self::new(limit, None)
    }

    /// Creates a continuation page from a server-issued cursor.
    pub fn after(limit: usize, cursor: &'a str) -> Result<Self, PaginationError> {
        Self::new(limit, Some(cursor))
    }

    pub(crate) fn limit(self) -> u8 {
        self.limit
    }

    pub(crate) fn cursor(self) -> Option<&'a str> {
        self.cursor
    }
}

impl Default for PageRequest<'_> {
    fn default() -> Self {
        Self {
            limit: DEFAULT_PAGE_SIZE,
            cursor: None,
        }
    }
}

impl fmt::Debug for PageRequest<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PageRequest")
            .field("limit", &self.limit)
            .field("has_cursor", &self.cursor.is_some())
            .finish()
    }
}

/// Reason a collection page request was rejected locally.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PaginationError {
    /// The requested limit was outside the API's inclusive 1 through 100 bound.
    InvalidLimit,
    /// The cursor was empty, oversized, or not unpadded URL-safe base64.
    InvalidCursor,
}

impl fmt::Display for PaginationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidLimit => "page limit must be between 1 and 100",
            Self::InvalidCursor => "page cursor is not a bounded URL-safe base64 value",
        })
    }
}

impl Error for PaginationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_requests_match_the_server_bounds() {
        assert_eq!(PageRequest::default().limit(), 50);
        assert!(PageRequest::first(1).is_ok());
        assert!(PageRequest::after(100, "YWJjLTEyMw").is_ok());
        assert_eq!(
            PageRequest::first(0).expect_err("zero is invalid"),
            PaginationError::InvalidLimit
        );
        assert_eq!(
            PageRequest::first(101).expect_err("over max is invalid"),
            PaginationError::InvalidLimit
        );
        assert_eq!(
            PageRequest::after(50, "cursor=").expect_err("padding is invalid"),
            PaginationError::InvalidCursor
        );

        assert!(valid_page_shape(
            &Page::<()> {
                items: Vec::new(),
                next_cursor: Some("YWJjLTEyMw".to_owned()),
            },
            50,
        ));
        assert!(!valid_page_shape(
            &Page {
                items: vec![(); 2],
                next_cursor: None,
            },
            1,
        ));
        assert!(!valid_page_shape(
            &Page::<()> {
                items: Vec::new(),
                next_cursor: Some("cursor=".to_owned()),
            },
            50,
        ));
    }
}
