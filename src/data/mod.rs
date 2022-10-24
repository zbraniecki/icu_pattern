// pub mod parser;
// pub mod resolvers;
// pub mod source;
// pub mod structs;
// pub mod types;
//
// use crate::pattern::{Pattern, PatternElement};
// use source::*;
// use structs::*;
// use types::*;
// use parser::Parser;
//
// pub fn get_data() -> Data {
//     let source_data = SourceData {
//         time: SourceTime {
//             format: "h:mm zzzz".to_string(),
//         },
//         date: SourceDate {
//             format: "MMMM d, y".to_string(),
//             date_combination: "{1} 'at' {0}".to_string(),
//         },
//         timezone: SourceTimezone {
//             format: "{0} Time".to_string(),
//             hour_format: "+HH:mm".to_string(),
//             fallback_format: "{1} ({0})".to_string(),
//         },
//     };
//
//     Data {
//         time: Time {
//             format: Parser::parse(&source_data.time.format, false),
//         },
//         date: Date {
//             format: Parser::parse(&source_data.date.format, false),
//             date_combination: Parser::parse(&source_data.date.date_combination, true),
//         },
//         timezone: Timezone {
//             format: Pattern {
//                 elements: vec![
//                     PatternElement::Placeholder(0),
//                     PatternElement::Literal(" Time".to_string()),
//                 ],
//             },
//             hour_format: Pattern {
//                 elements: vec![
//                     PatternElement::Literal("+".to_string()),
//                     PatternElement::Element(TimePatternElement::Hour),
//                     PatternElement::Literal(":".to_string()),
//                     PatternElement::Element(TimePatternElement::Minute),
//                 ],
//             },
//             fallback_format: Pattern {
//                 elements: vec![
//                     PatternElement::Placeholder(1),
//                     PatternElement::Literal(" (".to_string()),
//                     PatternElement::Placeholder(0),
//                     PatternElement::Literal(")".to_string()),
//                 ],
//             },
//         },
//     }
// }
