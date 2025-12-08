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

#[rustler::nif(schedule = "DirtyCpu")]
fn worksheets(filename: String) -> VecResultTuple {
    match open_workbook_auto(&filename) {
        Err(ref error) => VecResultTuple {
            lhs: atoms::error(),
            rhs: ResultType::String(format!("{}", error)),
        },
        Ok(ref inner) => VecResultTuple {
            lhs: atoms::ok(),
            rhs: ResultType::StringVec(inner.sheet_names().to_owned()),
        },
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn parse(filename: String, worksheet: String) -> VecResultTuple {
    let mut excel = match open_workbook_auto(&filename) {
        Err(ref error) => {
            return VecResultTuple {
                lhs: atoms::error(),
                rhs: ResultType::String(format!("{}", error)),
            }
        }
        Ok(inner) => inner,
    };

    if let Ok(range) = excel.worksheet_range(&worksheet) {
        let row: Vec<Vec<String>> = range
            .rows()
            .into_iter()
            .enumerate()
            .map(|(_i, col)| col.iter().map(|c| c.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        VecResultTuple {
            lhs: atoms::ok(),
            rhs: ResultType::StringVecVec(row),
        }
    } else {
        VecResultTuple {
            lhs: atoms::error(),
            rhs: ResultType::String("Couldnt find the worksheet".to_string()),
        }
    }
}

rustler::init!("Elixir.Xler.Native");
