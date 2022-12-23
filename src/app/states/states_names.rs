#[derive(PartialEq, Eq, Clone, Debug)]
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
