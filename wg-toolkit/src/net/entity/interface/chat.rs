use super::ObjectId;


#[derive(Debug)]
pub enum Client {
    OnChatAction { data: Vec<u8> }
}

#[derive(Debug)]
pub enum Base {
    ChatCommandFromClient {
        request_id: u64,
        command: u8,
        channel_id: ObjectId,
        i64_arg: i64,
        i16_arg: i16,
        str_arg1: String,
        str_arg2: String,
    },
    InviteCommand {
        request_id: u64,
        command: InviteCommand,
        invite_type: InviteType,
        receiver_name: String,
        i64_arg: i64,
        i16_arg: i16,
        str_arg1: String,
        str_arg2: String,
    }
}


#[derive(Debug)]
#[repr(u8)]
pub enum InviteCommand {
    CreateInvite = 24,
    GetActiveInvites = 28,
    GetArchiveInvites = 29,
}

#[derive(Debug)]
#[repr(u8)]
pub enum InviteType {
    Barter          = 0x00,
    Team            = 0x01,
    Clan            = 0x02,
    TrainingRoom    = 0x03,
    PreBattle       = 0x04,
    Undefined       = 0xFF,
}
