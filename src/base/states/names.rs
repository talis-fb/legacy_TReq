#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum StatesNames {
    Default,
    DefaultEditMode,
    TabList,
    Url,
    RequestHeaders,
    RequestBody,
    ResponseHeader,
    ResponseBody,
    Log,
    Empty,
}
