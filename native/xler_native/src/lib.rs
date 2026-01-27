extern crate calamine;
extern crate lazy_static;
extern crate rustler;

use calamine::{open_workbook_auto, Reader};
use rustler::{Atom, NifTuple};

mod atoms {
    rustler::atoms! {
        ok,
        error,
    }
}

#[derive(NifTuple)]
struct VecResultTuple {
    lhs: Atom,
    rhs: ResultType,
}

#[derive(rustler::NifUntaggedEnum)]
enum ResultType {
    StringVec(Vec<String>),
    StringVecVec(Vec<Vec<String>>),
    String(String),
}

// Testable core logic (not NIF functions)
fn get_worksheets_impl(filename: &str) -> Result<Vec<String>, String> {
    match open_workbook_auto(filename) {
        Err(error) => Err(format!("{}", error)),
        Ok(inner) => Ok(inner.sheet_names().to_owned()),
    }
}

fn parse_worksheet_impl(filename: &str, worksheet: &str) -> Result<Vec<Vec<String>>, String> {
    let mut excel = match open_workbook_auto(filename) {
        Err(error) => return Err(format!("{}", error)),
        Ok(inner) => inner,
    };

    match excel.worksheet_range(worksheet) {
        Ok(range) => {
            let rows: Vec<Vec<String>> = range
                .rows()
                .map(|cols| cols.iter().map(|c| c.to_string()).collect())
                .collect();
            Ok(rows)
        }
        Err(_) => Err("Couldnt find the worksheet".to_string()),
    }
}

// NIF wrapper functions
#[rustler::nif(schedule = "DirtyCpu")]
fn worksheets(filename: String) -> VecResultTuple {
    match get_worksheets_impl(&filename) {
        Ok(sheets) => VecResultTuple {
            lhs: atoms::ok(),
            rhs: ResultType::StringVec(sheets),
        },
        Err(error) => VecResultTuple {
            lhs: atoms::error(),
            rhs: ResultType::String(error),
        },
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn parse(filename: String, worksheet: String) -> VecResultTuple {
    match parse_worksheet_impl(&filename, &worksheet) {
        Ok(data) => VecResultTuple {
            lhs: atoms::ok(),
            rhs: ResultType::StringVecVec(data),
        },
        Err(error) => VecResultTuple {
            lhs: atoms::error(),
            rhs: ResultType::String(error),
        },
    }
}

rustler::init!("Elixir.Xler.Native");

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // Helper to get test file path from Rust tests
    fn test_file_path(filename: &str) -> String {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let mut path = PathBuf::from(manifest_dir);
        path.push("../../test/data");
        path.push(filename);
        path.to_str().unwrap().to_string()
    }

    // Tests for get_worksheets_impl function
    #[test]
    fn test_worksheets_valid_file() {
        let result = get_worksheets_impl(&test_file_path("sample.xlsx"));
        assert!(result.is_ok());
        let sheets = result.unwrap();
        assert_eq!(sheets.len(), 1);
        assert_eq!(sheets[0], "Sheet1");
    }

    #[test]
    fn test_worksheets_nonexistent_file() {
        let result = get_worksheets_impl(&test_file_path("nonexistent.xlsx"));
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(!error.is_empty(), "Error message should not be empty");
    }

    #[test]
    fn test_worksheets_invalid_file() {
        let result = get_worksheets_impl(&test_file_path("invalid.txt"));
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(!error.is_empty(), "Error message should not be empty");
    }

    #[test]
    fn test_worksheets_multiple_sheets() {
        let result = get_worksheets_impl(&test_file_path("multiple_sheets.xlsx"));
        assert!(result.is_ok());
        let sheets = result.unwrap();
        assert_eq!(sheets.len(), 3);
        assert!(sheets.contains(&"Sheet1".to_string()));
        assert!(sheets.contains(&"Data".to_string()));
        assert!(sheets.contains(&"Summary".to_string()));
    }

    #[test]
    fn test_worksheets_empty_sheet() {
        let result = get_worksheets_impl(&test_file_path("empty_sheet.xlsx"));
        assert!(result.is_ok());
        let sheets = result.unwrap();
        assert_eq!(sheets.len(), 1);
        assert_eq!(sheets[0], "Sheet1");
    }

    // Tests for parse_worksheet_impl function
    #[test]
    fn test_parse_valid_worksheet() {
        let result = parse_worksheet_impl(&test_file_path("sample.xlsx"), "Sheet1");
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.len(), 701, "Expected 701 rows");
        assert!(data[0].len() > 0, "First row should have columns");
    }

    #[test]
    fn test_parse_nonexistent_file() {
        let result = parse_worksheet_impl(&test_file_path("nonexistent.xlsx"), "Sheet1");
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(!error.is_empty(), "Error message should not be empty");
    }

    #[test]
    fn test_parse_invalid_worksheet_name() {
        let result = parse_worksheet_impl(&test_file_path("sample.xlsx"), "NonExistentSheet");
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(!error.is_empty(), "Error message should not be empty");
    }

    #[test]
    fn test_parse_empty_worksheet() {
        let result = parse_worksheet_impl(&test_file_path("empty_sheet.xlsx"), "Sheet1");
        assert!(result.is_ok());
        // Empty sheet should parse successfully (even if empty)
        let _data = result.unwrap();
    }

    #[test]
    fn test_parse_single_cell() {
        let result = parse_worksheet_impl(&test_file_path("single_cell.xlsx"), "Sheet1");
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.len(), 1, "Should have one row");
        assert_eq!(data[0].len(), 1, "Should have one column");
        assert_eq!(data[0][0], "test", "Cell value should be 'test'");
    }

    #[test]
    fn test_parse_unicode_content() {
        let result = parse_worksheet_impl(&test_file_path("unicode_data.xlsx"), "Sheet1");
        assert!(result.is_ok());
        let data = result.unwrap();
        assert!(data.len() > 0, "Should have data");
        // Check that Unicode is preserved
        let has_unicode = data.iter().any(|row| {
            row.iter()
                .any(|cell| cell.contains("日本語") || cell.contains("🎉"))
        });
        assert!(has_unicode, "Unicode content should be preserved");
    }

    #[test]
    fn test_parse_multiple_sheets_individually() {
        let sheet1 = parse_worksheet_impl(&test_file_path("multiple_sheets.xlsx"), "Sheet1");
        let data = parse_worksheet_impl(&test_file_path("multiple_sheets.xlsx"), "Data");
        let summary = parse_worksheet_impl(&test_file_path("multiple_sheets.xlsx"), "Summary");

        assert!(sheet1.is_ok());
        assert!(data.is_ok());
        assert!(summary.is_ok());

        // Verify each has data
        assert!(sheet1.unwrap().len() > 0);
        assert!(data.unwrap().len() > 0);
        assert!(summary.unwrap().len() > 0);
    }

    #[test]
    fn test_parse_special_characters() {
        let result = parse_worksheet_impl(&test_file_path("special_chars.xlsx"), "Data & Stats");
        assert!(result.is_ok());
        let data = result.unwrap();
        assert!(data.len() > 0, "Should have data with special chars");
    }
}
