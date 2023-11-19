use std::cmp::Ordering;

use ellipse::Ellipse;

pub fn get_column_string(text: &str, width: usize) -> String {
    let text_len = text.len();

    match text_len.cmp(&width) {
        Ordering::Equal => text.to_owned(),
        Ordering::Less => {
            let chars_left = width - text_len;
            let mut col_str = text.to_owned();
            for _ in 0..chars_left {
                    col_str.push(' ');
                }
                col_str
                // This code will be used to center the values by padding the data.
                // will probably need some sort of override for the description or other longer strings
                // let padding = ((width + 1) - text_len) / 2;
                // format!("{}{}{}"," ".repeat(padding), text, " ".repeat(padding))
        },
        Ordering::Greater => {
            if width == 0 {
                return "".to_owned();
            } else if width == 1 {
                return ".".to_owned();
            } else if width == 2 {
                return "..".to_owned();
            } else if width == 3 {
                return "...".to_owned();
            }
            let res = text.truncate_ellipse(width-3);
            res.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetest";

        let width = 0;

        assert_eq!(get_column_string(text4, width), "".to_owned());

        let width = 1;

        assert_eq!(get_column_string(text4, width), ".".to_owned());

        let width = 2;

        assert_eq!(get_column_string(text4, width), "..".to_owned());

        let width = 3;

        assert_eq!(get_column_string(text4, width), "...".to_owned());

        let width = 4;

        assert_eq!(get_column_string(text4, width), "t...".to_owned());

        let width = 6;

        assert_eq!(get_column_string(text1, width), "      ".to_owned());
        assert_eq!(get_column_string(text2, width), "test  ".to_owned());
        assert_eq!(get_column_string(text3, width), "testme".to_owned());
        assert_eq!(get_column_string(text4, width), "tes...".to_owned());
    } 
}