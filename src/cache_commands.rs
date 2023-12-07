#[derive(Clone, Copy, Debug)]
pub enum CommandType
{
    DataLoadRefference,
    DataStoreRefference,
    InstructionLoadRefference,
}

#[derive(Clone, Copy, Debug)]
pub struct Command
{
    pub command_type    : CommandType,
    pub address         : u32,
    pub trace_message   : Option<String>,
}