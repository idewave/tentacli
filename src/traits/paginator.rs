use std::io::{Error, ErrorKind};

pub trait Paginator {
    fn get_page_number(
        index: usize,
        items_count: usize,
        per_page: usize
    ) -> Result<usize, Error> {
        let total_pages = 0..=items_count / per_page;
        for page in total_pages.rev() {
            if index >= per_page * page {
                return Ok(page);
            }
        }

        Err(Error::new(
            ErrorKind::Other,
            format!("Cannot get page number for {} ({}/{})", index, items_count, per_page))
        )
    }

    fn get_relative_index(page: usize, per_page: usize, index: usize) -> Result<usize, Error> {
        let start = page * per_page;
        let end = (page + 1) * per_page - 1;

        let mut counter = 0;
        for i in start..=end {
            if index == i {
                return Ok(counter);
            }
            counter += 1;
        }

        Err(Error::new(
            ErrorKind::Other,
            format!(
                "Cannot get relative index for {} ({}/{}) on range {}..{}",
                index, page, per_page, start, end
            ))
        )
    }
}