pub mod output;
mod schema;
mod source;
pub mod types;

use crate::{parser::Parser, pattern::PatternElement};
use schema::*;
use source::*;
use types::*;

pub struct DateTimeData {
    data: DataSchema,
}

impl Default for DateTimeData {
    fn default() -> Self {
        let source_data = SourceData {
            time: SourceTime {
                format: "h:mm zzzz".to_string(),
            },
            date: SourceDate {
                format: "MMMM d, y".to_string(),
                date_combination: "{1} 'at' {0}".to_string(),
            },
            timezone: SourceTimezone {
                format: "{0} Time".to_string(),
                hour_format: "+HH:mm".to_string(),
                fallback_format: "{1} ({0})".to_string(),
            },
        };
        let data = DataSchema {
            time: Time {
                format: Parser::parse(&source_data.time.format, false),
            },
            date: Date {
                format: Parser::parse(&source_data.date.format, false),
                date_combination: Parser::parse(&source_data.date.date_combination, true),
            },
            timezone: Timezone {
                format: Parser::parse(&source_data.timezone.format, true),
                hour_format: Parser::parse(&source_data.timezone.hour_format, false),
                fallback_format: Parser::parse(&source_data.timezone.fallback_format, true),
            },
        };
        Self { data }
    }
}

impl DateTimeData {
    pub fn get_datetime_pattern(&self) -> &Vec<PatternElement<DateTimePatternElement>> {
        &self.data.date.date_combination
    }

    pub fn get_date_pattern(&self) -> &Vec<PatternElement<DatePatternElement>> {
        &self.data.date.format
    }

    pub fn get_time_pattern(&self) -> &Vec<PatternElement<TimePatternElement>> {
        &self.data.time.format
    }

    pub fn get_timezone_pattern(
        &self,
        variant: TimezonePatternVariant,
    ) -> (
        &Vec<PatternElement<TimezonePatternElement>>,
        Option<TimezonePatternPlaceholderScheme>,
    ) {
        match variant {
            TimezonePatternVariant::Format => (
                &self.data.timezone.format,
                Some(TimezonePatternPlaceholderScheme::Name),
            ),
            TimezonePatternVariant::HourFormat => (&self.data.timezone.hour_format, None),
            TimezonePatternVariant::FallbackFormat => (
                &self.data.timezone.fallback_format,
                Some(TimezonePatternPlaceholderScheme::NameOffset),
            ),
        }
    }
}
