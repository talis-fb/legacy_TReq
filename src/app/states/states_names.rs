#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum StatesNames {
    Default,
    TabList,
    Url,
    RequestHeaders,
    RequestBody,
    ResponseHeader,
    ResponseBody,
    Log,
    Empty,
}
