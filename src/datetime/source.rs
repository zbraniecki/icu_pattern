pub struct SourceTimezone<'input> {
    pub format: &'input str,
    pub hour_format: &'input str,
    pub fallback_format: &'input str,
}

pub struct SourceDate<'input> {
    pub format: &'input str,
    pub date_combination: &'input str,
}

pub struct SourceTime<'input> {
    pub format: &'input str,
}

pub struct SourceData<'input> {
    pub time: SourceTime<'input>,
    pub date: SourceDate<'input>,
    pub timezone: SourceTimezone<'input>,
}
