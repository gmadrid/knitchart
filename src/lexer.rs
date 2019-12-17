use std::iter::Peekable;
use std::iter::FromIterator;
use std::str::Chars;

//use crate::errors::*;

/*
  Tokens are easy:
    EOL      = an end-of-line 
    COMMENT  = '//'
    EQUALS   = '='
    IDENT    = [a-zA-Z][a-zA-Z0-9]*
    CHART    = 'CHART'
 */

pub struct Lexer {
    
}

#[derive(PartialEq, Debug)]
pub enum Token {
    Eol,
}

type Input<'a> = Peekable<Chars<'a>>;
    
    
fn skip_white(e: &mut Input)  {
    while let Some(ch) = e.peek() {
	if ch == &'\n' || !ch.is_ascii_whitespace() {
	    break;
	}
	e.next();
    }
}

fn match_eol(e: &mut Input) -> Option<Token> {
    skip_white(e);
    if e.peek() == Some(&'\n') {
	e.next();
	Some(Token::Eol)
    } else {
	None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    fn string_after_skipping(s: &str) -> String {
	let mut iter = s.chars().peekable();
	skip_white(&mut iter);
	String::from_iter(iter)
    }

    #[test]
    fn test_skip_white() {
	assert_eq!("candy", string_after_skipping("candy"));
	assert_eq!("candy", string_after_skipping("  candy"));
	assert_eq!("\n candy", string_after_skipping("  \n candy"));
	assert_eq!("\n   candy", string_after_skipping("\n   candy"));
    }

    #[test]
    fn test_eol() {
	assert_eq!(Some(Token::Eol), match_eol(&mut "\n".chars().peekable()));
	assert_eq!(None, match_eol(&mut "1".chars().peekable()));
	assert_eq!(None, match_eol(&mut " ".chars().peekable()));
	assert_eq!(None, match_eol(&mut "/".chars().peekable()));

	let mut iter = "   \nlicorice".chars().peekable();
    	assert_eq!(Some(Token::Eol), match_eol(&mut iter));
	assert_eq!("licorice", String::from_iter(iter));
    }
}
