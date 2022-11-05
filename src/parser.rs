use crate::pattern::PatternElement;

pub struct Parser {}

impl Parser {
    pub fn parse<E>(input: &str, placeholders: bool) -> Vec<PatternElement<E>>
    where
        E: TryFrom<char>,
    {
        let mut elements = vec![];

        let mut idx = 0;

        let mut current_token: Option<(char, usize)> = None;
        let mut current_literal = String::new();

        let chars: Vec<char> = input.chars().collect();

        while let Some(ch) = chars.get(idx) {
            if let Some((curr_ch, count)) = current_token {
                if curr_ch == *ch {
                    //XXX: Increase count
                    idx += 1;
                    continue;
                } else {
                    current_token = None;
                    let token = Self::get_token(curr_ch, count).unwrap();
                    elements.push(token);
                }
            }
            if !placeholders && Self::is_token(*ch) {
                if !current_literal.is_empty() {
                    elements.push(PatternElement::Literal(current_literal.replace('\'', "")));
                    current_literal.clear();
                }
                current_token = Some((*ch, 1));
                idx += 1;
                continue;
            }
            if *ch == '{' {
                if !current_literal.is_empty() {
                    elements.push(PatternElement::Literal(current_literal.replace('\'', "")));
                    current_literal.clear();
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
                continue;
            }
            current_literal.push(*ch);
            idx += 1;
        }

        if let Some((ch, count)) = current_token.take() {
            let token = Self::get_token(ch, count).unwrap();
            elements.push(token);
        }
        if !current_literal.is_empty() {
            elements.push(PatternElement::Literal(current_literal.replace('\'', "")));
            current_literal.clear();
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

    fn get_token<E>(ch: char, _count: usize) -> Option<PatternElement<E>>
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

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let pattern = Parser::parse("h:mm", false);
//         assert_eq!(
//             pattern.elements,
//             vec![
//                 PatternElement::Element(TimePatternElement::Hour),
//                 PatternElement::Literal(":".to_string()),
//                 PatternElement::Element(TimePatternElement::Minute),
//             ]
//         );
//
//         let pattern = Parser::parse("h:mm zzzz", false);
//         assert_eq!(
//             pattern.elements,
//             vec![
//                 PatternElement::Element(TimePatternElement::Hour),
//                 PatternElement::Literal(":".to_string()),
//                 PatternElement::Element(TimePatternElement::Minute),
//                 PatternElement::Literal(" ".to_string()),
//                 PatternElement::Element(TimePatternElement::Timezone),
//             ]
//         );
//
//         let pattern = Parser::parse("h:m:s", false);
//         assert_eq!(
//             pattern.elements,
//             vec![
//                 PatternElement::Element(TimePatternElement::Hour),
//                 PatternElement::Literal(":".to_string()),
//                 PatternElement::Element(TimePatternElement::Minute),
//                 PatternElement::Literal(":".to_string()),
//                 PatternElement::Element(TimePatternElement::Second),
//             ]
//         );
//
//         let pattern = Parser::parse("y/M/d", false);
//         assert_eq!(
//             pattern.elements,
//             vec![
//                 PatternElement::Element(DatePatternElement::Year),
//                 PatternElement::Literal("/".to_string()),
//                 PatternElement::Element(DatePatternElement::Month),
//                 PatternElement::Literal("/".to_string()),
//                 PatternElement::Element(DatePatternElement::Day),
//             ]
//         );
//
//         let pattern = Parser::parse("d M, y", false);
//         assert_eq!(
//             pattern.elements,
//             vec![
//                 PatternElement::Element(DatePatternElement::Day),
//                 PatternElement::Literal(" ".to_string()),
//                 PatternElement::Element(DatePatternElement::Month),
//                 PatternElement::Literal(", ".to_string()),
//                 PatternElement::Element(DatePatternElement::Year),
//             ]
//         );
//
//         let pattern = Parser::parse::<Never>("{1} 'at' {0}", true);
//         assert_eq!(
//             pattern.elements,
//             vec![
//                 PatternElement::Placeholder(1),
//                 PatternElement::Literal(" at ".to_string()),
//                 PatternElement::Placeholder(0),
//             ]
//         );
//
//         let pattern = Parser::parse::<Never>("{0} Time", true);
//         assert_eq!(
//             pattern.elements,
//             vec![
//                 PatternElement::Placeholder(0),
//                 PatternElement::Literal(" Time".to_string()),
//             ]
//         );
//
//         let pattern = Parser::parse::<Never>("{0} ({1})", true);
//         assert_eq!(
//             pattern.elements,
//             vec![
//                 PatternElement::Placeholder(0),
//                 PatternElement::Literal(" (".to_string()),
//                 PatternElement::Placeholder(1),
//                 PatternElement::Literal(")".to_string()),
//             ]
//         );
//     }
// }
