fn main() {
    struct SpreadsheetCell {
        int: i32,
        float: f64,
        text: String,
    }

    let spreadsheet = SpreadsheetCell {
        int: 10,
        float: 12.2,
        text: String::from("nice"),
    };

    match &spreadsheet.float {
        
    }
}