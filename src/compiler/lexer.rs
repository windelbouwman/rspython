

use std::str::CharIndices;
use super::token::Tok;
use std::iter::FromIterator;
// use std::collections::HashMap;

pub struct Lexer<'input> {
    chars: CharIndices<'input>,
    at_begin_of_line: bool,
    nesting: usize, // Amount of parenthesis
    indentation_stack: Vec<usize>,
    chr0: Option<char>,
    chr1: Option<char>,
}

#[derive(Debug)]
pub enum LexicalError {

}

pub type Spanned<Tok> = Result<(usize, Tok), LexicalError>;

pub fn lex_source(source: &String) -> Vec<Tok> {
    let lexer = Lexer::new(source);
    Vec::from_iter(lexer.map(|x| x.unwrap().1))
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        let mut l = Lexer {
            chars: input.char_indices(),
            at_begin_of_line: true,
            nesting: 0,
            indentation_stack: vec![0],
            chr0: None,
            chr1: None,
        };
        l.next_char();
        l.next_char();
        l
    }

    // Lexer helper functions:
    fn lex_identifier(&mut self) -> Option<Spanned<Tok>> {
        while self.is_char() {
            self.next_char()
        }

        return Some(Ok((0, Tok::Name)))
    }

    fn lex_number(&mut self) -> Option<Spanned<Tok>> {
        while self.is_number() {
            self.next_char()
        }

        return Some(Ok((0, Tok::Number)))
    }

    fn lex_comment(&mut self) {
        while !self.is_end() {
            self.next_char()
        }
    }

    fn lex_string(&mut self) -> Option<Spanned<Tok>> {
        self.next_char();

        // TODO
        self.next_char();

        return Some(Ok((0, Tok::String)))
    }

    fn is_char(&self) -> bool {
        match self.chr0 {
            Some('a'...'z') => return true,
            _ => return false,
        }
    }

    fn is_number(&self) -> bool {
        match self.chr0 {
            Some('0'...'9') => return true,
            _ => return false,
        }
    }

    fn is_end(&self) -> bool {
        match self.chr0 {
            None => return true,
            _ => return false,
        }
    }

    fn next_char(&mut self) {
        self.chr0 = self.chr1;
        self.chr1 = self.chars.next().map(|x| x.1);
    }
}

/* Implement iterator pattern for the get_tok function.

Calling the next element in the iterator will yield the next lexical
token.
*/
impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok>;

    fn next(&mut self) -> Option<Self::Item> {
        // Idea: create some sort of hash map for single char tokens:
        // let mut X = HashMap::new();
        // X.insert('=', Tok::Equal);

        loop {

            if self.at_begin_of_line {
                self.at_begin_of_line = false;

                // Determine indentation:
                let mut col: usize = 0;
                loop {
                    match self.chr0 {
                        Some(' ') => {
                            self.next_char();
                            col += 1;
                        },
                        _ => {
                            break;
                        }
                    }
                }

                if self.nesting == 0 {
                    // Determine indent or dedent:
                    let current_indentation = *self.indentation_stack.last().unwrap();
                    if col == current_indentation {
                        // Same same
                    } else if col > current_indentation {
                        self.indentation_stack.push(col);
                        return Some(Ok((0, Tok::Indent)));
                    } else if col < current_indentation {
                        return Some(Ok((0, Tok::Dedent)));
                    }
                }
            }

            match self.chr0 {
                Some('0'...'9') => {
                    return self.lex_number()
                },
                // TODO: 'A'...'Z'
                Some('a'...'z') => {
                    return self.lex_identifier()
                },
                Some('#') => {
                    self.next_char();
                    self.lex_comment();
                    continue
                },
                Some('"') => {
                    self.next_char();
                    return self.lex_string();
                },
                Some('=') => {
                    //let V = X[&self.chr0.unwrap()];
                    self.next_char();
                    return Some(Ok((0, Tok::Equal)))
                    //return Some(Ok((0, V)))
                },
                Some('+') => {
                    self.next_char();
                    return Some(Ok((0, Tok::Plus)))
                },
                Some('-') => {
                    self.next_char();
                    return Some(Ok((0, Tok::Minus)))
                },
                Some('(') => {
                    self.next_char();
                    self.nesting += 1;
                    return Some(Ok((0, Tok::Lbrace)))
                },
                Some(')') => {
                    self.next_char();
                    self.nesting -= 1;
                    return Some(Ok((0, Tok::RBrace)))
                },
                Some('[') => {
                    self.next_char();
                    self.nesting += 1;
                    return Some(Ok((0, Tok::Lbrace)))
                },
                Some(']') => {
                    self.next_char();
                    self.nesting -= 1;
                    return Some(Ok((0, Tok::RBrace)))
                },
                Some(':') => {
                    self.next_char();
                    return Some(Ok((0, Tok::Colon)))
                },
                Some(',') => {
                    self.next_char();
                    return Some(Ok((0, Tok::Comma)))
                },
                Some('\n') => {
                    self.next_char();
                    self.at_begin_of_line = true;
                    return Some(Ok((0, Tok::Newline)))
                },
                None => return None,
                _ => {
                    self.next_char();
                    continue
                }, // Ignore all the rest..
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tok;
    use super::lex_source;

    #[test]
    fn test_line_comment() {
        let source = String::from(r"99232  # Bladibla");
        let tokens = lex_source(&source);
        assert_eq!(tokens, vec![Tok::Number]);
    }

    #[test]
    fn test_assignment() {
        let source = String::from(r"avariable = 99 + 2-0");
        let tokens = lex_source(&source);
        assert_eq!(tokens, vec![Tok::Name, Tok::Equal, Tok::Number, Tok::Plus, Tok::Number, Tok::Minus, Tok::Number]);
    }

    #[test]
    fn test_indentation() {
        let source = String::from("def foo():\n   return 99\n");
        let tokens = lex_source(&source);
        assert_eq!(tokens, vec![Tok::Name, Tok::Name, Tok::Lbrace, Tok::RBrace, Tok::Colon, Tok::Newline, Tok::Indent, Tok::Name, Tok::Number, Tok::Newline, Tok::Dedent]);
    }

    #[test]
    fn test_newline_in_brackets() {
        let source = String::from("x = [\n    1,2\n]\n");
        let tokens = lex_source(&source);
        assert_eq!(tokens, vec![Tok::Name, Tok::Equal, Tok::Lbrace, Tok::Newline, Tok::Number, Tok::Comma, Tok::Number, Tok::Newline, Tok::RBrace, Tok::Newline]);
    }
}
