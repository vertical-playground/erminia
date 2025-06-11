pub struct Lexer<'a> {
    content: &'a str,
    cursor: usize,
    line: usize, 
}

impl Default for Lexer<'_> {
    fn default() -> Self {
        Lexer {
            content: "",
            cursor: 0,
            line: 0
        }
    }
}

impl<'a> Lexer<'a> {

    fn new(
        content: &'a str,
    ) -> Lexer<'a> {
        Lexer {
            content: content, 
            cursor: 0,
            line: 0
        }
    }

    fn set_content(&mut self, content: &'a str) {
        self.content = content;
    }

    fn get_content(&self) -> &'a str {
        self.content 
    }

    fn next_char(&mut self) -> Option<char> {
        let next = self.content[self.get_cursor()..].chars().next();
        match next {
            Some(n) => {
                self.cursor += 1;
                Some(n)
            },
            None => None
        }
    }

    fn isspace(&mut self, c: char) -> bool {
        c == ' '
    }

    fn starts_with<'b>(&mut self, prefix: &'b str) -> bool {
        let prefix_len: usize = prefix.len();

        if prefix_len == 0 {
            return false;
        }

        if self.get_cursor() + prefix_len - 1 >= self.get_content().len() {
            return false;
        }

        false
    }

    fn get_cursor(&self) -> usize { self.cursor }

}
