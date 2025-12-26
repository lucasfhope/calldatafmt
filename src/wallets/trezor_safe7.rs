use anyhow::Result;
use super::utils;

const COLS: usize = 18;
const LINES: usize = 7;
const PAGE_SIZE: usize = COLS * LINES;

const ELIPSES: &str = "...";

pub fn display_trezor_safe7_calldata(calldata_in: &str) -> Result<()> {
    let calldata = utils::clean_and_validate_calldata(calldata_in)?;
    let calldata = calldata.to_ascii_lowercase();

    let pages = format_page_data(&calldata);
    let num_pages = pages.len();

    println!("-----------------------------\n");
    for (idx, page) in pages.iter().enumerate() {
        print_page(page, (idx as u128) + 1, num_pages as u128);
    }

    Ok(())
}

fn print_page(page_data: &str, page_num: u128, num_pages: u128) {

    // Force wrap at 18 chars/line like the device
    for line in page_data.as_bytes().chunks(COLS) {
        let s = std::str::from_utf8(line).unwrap_or("");
        println!("{}", s);
    }
    println!("\n{}/{}", page_num, num_pages);
    println!("-----------------------------\n");
}

fn format_page_data(data: &str) -> Vec<String> {
    let mut pages: Vec<String> = Vec::new();

    if data.is_empty() {
        return pages;
    }

    let elipses_length: usize = ELIPSES.len(); 

    // Fits entirely on page 1, no markers needed
    if data.len() <= PAGE_SIZE {
        pages.push(data.to_string());
        return pages;
    }

    // Page 1 has elipses at the end
    let first_payload: usize = PAGE_SIZE - elipses_length;

    let mut start: usize = 0;
    let mut end: usize = first_payload.min(data.len());

    let mut first = String::new();
    first.push_str(&data[start..end]);
    first.push_str(ELIPSES);
    pages.push(first);

    start = end;

    // Middle pages have elipses at the beginning and end
    let middle_payload: usize = PAGE_SIZE - (2 * elipses_length);

    // Last page: "..." + payload (no trailing marker)
    let last_payload: usize = PAGE_SIZE - elipses_length;

    while start < data.len() {
        let remaining: usize = data.len() - start;

        // Last page
        if remaining <= last_payload {
            let mut last = String::new();
            last.push_str(ELIPSES);
            last.push_str(&data[start..]);
            pages.push(last);
            break;
        }

        // Middle page
        let take: usize = middle_payload.min(remaining);
        end = start + take;

        let mut mid = String::new();
        mid.push_str(ELIPSES);
        mid.push_str(&data[start..end]);
        mid.push_str(ELIPSES);
        pages.push(mid);

        start = end;
    }

    pages
}
