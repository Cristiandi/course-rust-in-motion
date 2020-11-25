fn save_status(text: &str) -> Result<i64, &'static str> {
    if text.len() > 200 {
        return Err("status text is too long.");
    }

    let record = save_to_database(text)?;

    return Ok(record.id);
}

fn save_to_database(txt: &str) -> Result<StatusRecord, &'static str> {
    // fake imp
    Err("database unavaliable")
}

struct StatusRecord {
    id: i64,
    text: String,
    created_at: std::time::Instant
}

fn main() {
    let test_text = String::from("divelo");

    let a = save_status(&test_text).unwrap();
}