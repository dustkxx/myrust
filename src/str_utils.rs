    #[allow(dead_code)]
    pub fn split_words(s: &str) -> Vec<&str> {
        let mut result: Vec<&str> = Vec::new();
        let bytes = s.as_bytes();
        let mut space_idx: usize = 0;
        let mut last_char: u8 = b' ';
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                // If the current character is a space
                // and the last character was not a space, we have a word
                // so we push the word to the result vector
                if last_char != b' ' {
                    result.push(&s[space_idx..i]);
                    space_idx = i + 1;
                } else {
                    // If the last character was a space, we just update the space index
                    // to the current index
                    // This is to handle multiple spaces between words
                    // and to avoid pushing empty strings to the result vector
                    space_idx = space_idx + 1;
                }
            }
            last_char = item;
        }
        result.push(&s[space_idx..]); // Push the last word
        return result;
    }


    #[allow(dead_code)]
    pub fn traits(s:&str) ->String {
        // 处理字符串
        // 将字符串中的空格替换为下划线
        // 处理字符串
        let mut result = String::new();
        let bytes = s.as_bytes();
        for &item in bytes.iter() {
            if item == b' ' {
                result.push_str("zhonghua");
            } else {
                result.push_str("zhonghua");
            }
        }
        return result;
    }
