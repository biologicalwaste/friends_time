use time::UtcOffset;
use time::macros::format_description;

pub fn offset_parser(offset: &str) -> Result<UtcOffset, String> {
    let format = format_description!("[offset_hour]:[offset_minute]");    
    let new_offset = match UtcOffset::parse(&offset, format) {
        Ok(offset_new) => Ok(offset_new),
        Err(_) => {
            Err("Did not parse correctly.".to_string())
        }
    };

    return new_offset;
}