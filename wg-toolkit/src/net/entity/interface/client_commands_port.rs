


#[derive(Debug)]
pub enum Client {
    CommandResponse {
        request_id: u16,
        result_id: u16,
        error: String,
    },
    CommandResponseExt {
        request_id: u16,
        result_id: u16,
        error: String,
        ext: String,
    }
}


#[derive(Debug)]
pub enum Server {
    CommandStr {
        request_id: u16,
        command: u16,
        value: String,
    },
    CommandInt {
        request_id: u16,
        command: u16,
        value: i64,
    },
    CommandInt2 {
        request_id: u16,
        command: u16,
        value0: i64,
        value1: i64,
    },
    CommandInt3 {
        request_id: u16,
        command: u16,
        value0: i64,
        value1: i64,
        value2: i64,
    },
    CommandInt4 {
        request_id: u16,
        command: u16,
        value0: i64,
        value1: i64,
        value2: i64,
        value3: i64,
    },
    CommandInt2Str {
        request_id: u16,
        command: u16,
        value0: i64,
        value1: i64,
        value2: String,
    },
    CommandIntArr {
        request_id: u16,
        command: u16,
        value: Vec<i32>,
    },
    CommandIntStr {
        request_id: u16,
        command: u16,
        value0: i64,
        value1: String,
    },
    CommandIntStrArr {
        request_id: u16,
        command: u16,
        value0: i64,
        value1: Vec<String>,
    },
    CommandIntArrStrArr {
        request_id: u16,
        command: u16,
        value0: Vec<i64>,
        value1: Vec<String>,
    },
    CommandStrArr {
        request_id: u16,
        command: u16,
        value: Vec<String>,
    },
}


/// From https://github.com/StranikS-Scan/WorldOfTanks-Decompiled/blob/1.19.1/source/res/scripts/common/accountcommands.py
#[derive(Debug)]
#[repr(u16)]
pub enum Command {
    SyncData                = 100,
    Equip                   = 101,
    EquipOptDev             = 102,
    EquipShells             = 103,
    EquipEqs                = 104,
    EquipTman               = 105,
    Repair                  = 106,
    VehicleSettings         = 107,
    SetAndFillLayouts       = 108,
    SellC11nItems           = 117,
    BuyC11nItems            = 118,
    VehicleApplyOutfit      = 119,
    ResetC11nItemsNovelty   = 120,
    SetActiveVehicleSeason  = 121,
    SelectPotatovQuests     = 124,
    GetPotatovQuestReward   = 125,
    BuyPotatovQuestTile     = 126,
    BuyPotatovQuestSlot     = 127,
    ResetPotatovQueueTile   = 128,
    PausePotatovQueueTile   = 129,
    TmanAddSkill            = 151,
    TmanDropSkills          = 152,
}
