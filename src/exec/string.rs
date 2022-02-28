//! String detection in binary data.


/// Iterator over all valid null-terminated strings (Cstr) found in the given bytes source.
pub struct CstrFinderIterator<'a> {
    image: &'a [u8],
    image_iter: std::iter::Enumerate<std::slice::Iter<'a, u8>>,
    str_start: usize,
    str_parsing: bool,
    str_length: u32,
    str_valid_count: u32
}

impl<'a> CstrFinderIterator<'a> {

    pub fn new(image: &'a [u8]) -> Self {
        Self {
            image_iter: image.iter().enumerate(),
            image,
            str_start: 0,
            str_parsing: false,
            str_length: 0,
            str_valid_count: 0
        }
    }

}

impl<'a> Iterator for CstrFinderIterator<'a> {

    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {

        while let Some((i, &b)) = self.image_iter.next() {
            if b != 0 {
                if !self.str_parsing {
                    self.str_start = i;
                    self.str_valid_count = 0;
                    self.str_length = 0;
                    self.str_parsing = true;
                }
                let c = b as char;
                if !c.is_ascii_control() && c.is_ascii() {
                    self.str_valid_count += 1;
                }
                self.str_length += 1;
            } else if self.str_parsing {
                self.str_parsing = false;
                if self.str_length > 1 {
                    let valid_chars_ratio = self.str_valid_count as f32 / self.str_length as f32;
                    if valid_chars_ratio > 0.8 {
                        let str_data = &self.image[self.str_start..i];
                        if let Ok(str_buf) = std::str::from_utf8(str_data) {
                            return Some((self.str_start, str_buf));
                        }
                    }
                }
            }
        }

        None

    }

}
