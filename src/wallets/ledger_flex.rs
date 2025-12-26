use anyhow::Result;
use super::utils;

pub fn display_ledger_flex_calldata(calldata_in: &str) -> Result<()> {
    let calldata = utils::clean_and_validate_calldata(calldata_in)?;

    let selector = &calldata[..8];
    let body = &calldata[8..];

    print_selector(selector);

    for chunk in body.as_bytes().chunks(64) {
        let parameter = std::str::from_utf8(chunk)?;
        print_parameter(parameter);
    }

    Ok(())
}

fn print_selector(selector: &str) {
    println!("-----------------------------");
    println!("Selector:\n");
    println!("{}\n", selector);
    println!("-----------------------------");
}

fn print_parameter(parameter: &str) {
    println!("Parameter:\n");
    print_formatted_parameter(&format_parameter(parameter));
    println!("-----------------------------");
}

/// Formats a single parameter that takes up one page
/// Assumes each parameter contains 32 bytes (64 char)
///
/// Formatting:
/// - Each 8 bytes is seperated by a colon (:)
/// - A section with only zeros is denoted as 00
/// - The first non-zero byte in a section has no leading 00s,
///   all bytes are displayed after the non-zero byte
///   Ex: 000000005f000000 -> 5f000000
/// - All non-numeric characters are capitalized
fn format_parameter(word: &str) -> String {
    let up = word.to_ascii_uppercase();
    let sections = [&up[0..16], &up[16..32], &up[32..48], &up[48..64]];
    let mut out: Vec<String> = Vec::with_capacity(4);

    for sec in sections {
        if is_all_zeros(sec) {
            out.push("00".to_string());
            continue;
        }
        out.push(trim_leading_zeros(sec));
    }
    out.join(":")
}

/// Will print a parameter and wrap to a newline after every 17 non-colon characters
fn print_formatted_parameter(formatted: &str) {
    let mut it = formatted.chars().peekable();
    let mut hex_count = 0usize;

    while let Some(ch) = it.next() {
        print!("{ch}");
        if ch != ':' {
            hex_count += 1;
        }
        if hex_count == 17 {
            if matches!(it.peek(), Some(':')) {
                print!("{}", it.next().unwrap());
            }
            println!();
            hex_count = 0;
        }
    }
    println!("\n");
}

fn is_all_zeros(sec: &str) -> bool {
    sec.chars().all(|c| c == '0')
}

fn trim_leading_zeros(sec: &str) -> String {
    let trimmed = sec.trim_start_matches("00");
    if trimmed.is_empty() {
        "00".to_string()
    } else {
        trimmed.to_string()
    }
}
