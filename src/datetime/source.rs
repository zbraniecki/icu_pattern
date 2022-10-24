pub struct SourceTimezone {
    pub format: String,
    pub hour_format: String,
    pub fallback_format: String,
    pub name: String,
}

pub struct SourceDate {
    pub format: String,
    pub date_combination: String,
}

pub struct SourceTime {
    pub format: String,
}

pub struct SourceData {
    pub time: SourceTime,
    pub date: SourceDate,
    pub timezone: SourceTimezone,
}
