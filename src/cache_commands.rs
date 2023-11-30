pub enum CommandType
{
    DataLoadRefference,
    DataStoreRefference,
    InstructionLoadRefference,
}

pub struct Command
{
    pub command_type    : CommandType,
    pub address         : u32,
    pub trace_message   : Option<String>,
}