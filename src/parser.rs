use crate::pattern::PatternElement;

pub struct Parser {}

impl<'input> Parser {
    pub fn parse<E>(input: &'input str, placeholders: bool) -> Vec<PatternElement<'input, E>>
    where
        E: TryFrom<char>,
    {
        let mut elements = vec![];

        let mut idx = 0;

        let mut current_token: Option<(char, usize)> = None;
        let mut current_literal_start = 0;

        let chars: Vec<char> = input.chars().collect();

        while let Some(ch) = chars.get(idx) {
            if let Some((curr_ch, count)) = current_token {
                if curr_ch == *ch {
                    //XXX: Increase count
                    idx += 1;
                    continue;
                } else {
                    current_token = None;
                    current_literal_start = idx;
                    let token = Self::get_token(curr_ch, count).unwrap();
                    elements.push(token);
                }
            }
            if !placeholders && Self::is_token(*ch) {
                if current_literal_start != idx {
                    let s = unsafe { input.get_unchecked(current_literal_start..idx) };
                    if s.contains('\'') {
                        elements.push(PatternElement::Literal(s.replace('\'', "").into()));
                    } else {
                        elements.push(PatternElement::Literal(s.into()));
                    }
                }
                current_token = Some((*ch, 1));
                idx += 1;
                current_literal_start = idx;
                continue;
            }
            if *ch == '{' {
                if current_literal_start != idx {
                    let s = unsafe { input.get_unchecked(current_literal_start..idx) };
                    if s.contains('\'') {
                        elements.push(PatternElement::Literal(s.replace('\'', "").into()));
                    } else {
                        elements.push(PatternElement::Literal(s.into()));
                    }
                }
                idx += 1;
                let p = chars.get(idx).unwrap();
                idx += 1; // 0 | 1
                idx += 1; // }
                if *p == '0' {
                    elements.push(PatternElement::Placeholder(0));
                } else if *p == '1' {
                    elements.push(PatternElement::Placeholder(1));
                }
                current_literal_start = idx;
                continue;
            }
            idx += 1;
        }

        if let Some((ch, count)) = current_token.take() {
            let token = Self::get_token(ch, count).unwrap();
            elements.push(token);
        } else if idx != current_literal_start {
            let s = unsafe { input.get_unchecked(current_literal_start..idx) };
            if s.contains('\'') {
                elements.push(PatternElement::Literal(s.replace('\'', "").into()));
            } else {
                elements.push(PatternElement::Literal(s.into()));
            }
        }

        elements
    }

    fn is_token(ch: char) -> bool {
        match ch {
            'H' => true,
            'h' => true,
            'm' => true,
            's' => true,
            'z' => true,
            'y' => true,
            'M' => true,
            'd' => true,
            _ => false,
        }
    }

    fn get_token<E>(ch: char, _count: usize) -> Option<PatternElement<'input, E>>
    where
        E: TryFrom<char>,
    {
        if let Ok(element) = ch.try_into() {
            Some(PatternElement::Element(element))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::datetime::types::*;

    #[test]
    fn parser_test() {
        let elements = Parser::parse("h:mm", false);
        assert_eq!(
            elements,
            vec![
                PatternElement::Element(TimePatternElement::Hour),
                PatternElement::Literal(":".into()),
                PatternElement::Element(TimePatternElement::Minute),
            ]
        );

        let elements = Parser::parse("h:mm zzzz", false);
        assert_eq!(
            elements,
            vec![
                PatternElement::Element(TimePatternElement::Hour),
                PatternElement::Literal(":".into()),
                PatternElement::Element(TimePatternElement::Minute),
                PatternElement::Literal(" ".into()),
                PatternElement::Element(TimePatternElement::Timezone),
            ]
        );

        let elements = Parser::parse("h:m:s", false);
        assert_eq!(
            elements,
            vec![
                PatternElement::Element(TimePatternElement::Hour),
                PatternElement::Literal(":".into()),
                PatternElement::Element(TimePatternElement::Minute),
                PatternElement::Literal(":".into()),
                PatternElement::Element(TimePatternElement::Second),
            ]
        );

        let elements = Parser::parse("y/M/d", false);
        assert_eq!(
            elements,
            vec![
                PatternElement::Element(DatePatternElement::Year),
                PatternElement::Literal("/".into()),
                PatternElement::Element(DatePatternElement::Month),
                PatternElement::Literal("/".into()),
                PatternElement::Element(DatePatternElement::Day),
            ]
        );

        let elements = Parser::parse("d M, y", false);
        assert_eq!(
            elements,
            vec![
                PatternElement::Element(DatePatternElement::Day),
                PatternElement::Literal(" ".into()),
                PatternElement::Element(DatePatternElement::Month),
                PatternElement::Literal(", ".into()),
                PatternElement::Element(DatePatternElement::Year),
            ]
        );

        let elements = Parser::parse::<TimePatternElement>("{1} 'at' {0}", true);
        assert_eq!(
            elements,
            vec![
                PatternElement::Placeholder(1),
                PatternElement::Literal(" at ".into()),
                PatternElement::Placeholder(0),
            ]
        );

        let elements = Parser::parse::<TimezonePatternElement>("{0} Time", true);
        assert_eq!(
            elements,
            vec![
                PatternElement::Placeholder(0),
                PatternElement::Literal(" Time".into()),
            ]
        );

        let elements = Parser::parse::<DateTimePatternElement>("{0} ({1})", true);
        assert_eq!(
            elements,
            vec![
                PatternElement::Placeholder(0),
                PatternElement::Literal(" (".into()),
                PatternElement::Placeholder(1),
                PatternElement::Literal(")".into()),
            ]
        );
    }
}
