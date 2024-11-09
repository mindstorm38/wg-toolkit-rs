use super::super::alias::*;

/// Methods for Wheels on base component
pub enum WheelsMethod { 
}

/// Methods for VehiclesSpawnListStorage_Avatar on base component
pub enum VehiclesSpawnListStorage_AvatarMethod { 
}

/// Methods for VehicleRemovalController_Avatar on base component
pub enum VehicleRemovalController_AvatarMethod { 
}

/// Methods for VehicleObserver on base component
pub enum VehicleObserverMethod { 
}

/// Methods for VehicleHealthBroadcastListenerComponent_Avatar on base component
pub enum VehicleHealthBroadcastListenerComponent_AvatarMethod { 
}

/// Methods for VehicleAIProxy on base component
pub enum VehicleAIProxyMethod { 
}

/// Methods for TriggersController_Avatar on base component
pub enum TriggersController_AvatarMethod { 
}

/// Methods for TransactionUser on base component
pub enum TransactionUserMethod { 
}

/// Methods for ThrottledMethods on base component
pub enum ThrottledMethodsMethod { 
}

/// Methods for TeamHealthBar_Avatar on base component
pub enum TeamHealthBar_AvatarMethod { 
}

/// Methods for TeamBase_Vehicle on base component
pub enum TeamBase_VehicleMethod { 
}

/// Methods for StepRepairPoint_Vehicle on base component
pub enum StepRepairPoint_VehicleMethod { 
}

/// Methods for SmokeController_Vehicle on base component
pub enum SmokeController_VehicleMethod { 
}

/// Methods for SessionTracker on base component
pub enum SessionTrackerMethod { 
}

/// Methods for ServerSideReplays on base component
pub enum ServerSideReplaysMethod { 
}

/// Methods for Sector_Vehicle on base component
pub enum Sector_VehicleMethod { 
}

/// Methods for SectorBase_Vehicle on base component
pub enum SectorBase_VehicleMethod { 
}

/// Methods for RespawnController_Vehicle on base component
pub enum RespawnController_VehicleMethod { 
}

/// Methods for RespawnController_Avatar on base component
pub enum RespawnController_AvatarMethod { 
    respawnController_requestRespawnGroupChange(u8), // idx(0)
    respawnController_chooseVehicleForRespawn(u16), // idx(1)
    respawnController_performRespawn(), // idx(2)
    respawnController_chooseRespawnZone(Vec3), // idx(10)
    respawnController_switchSetup(u16, u8, u8), // idx(11)
}

/// Methods for RepairBase_Vehicle on base component
pub enum RepairBase_VehicleMethod { 
}

/// Methods for RecoveryMechanic_Vehicle on base component
pub enum RecoveryMechanic_VehicleMethod { 
}

/// Methods for RecoveryMechanic_Avatar on base component
pub enum RecoveryMechanic_AvatarMethod { 
}

/// Methods for QuestProcessor on base component
pub enum QuestProcessorMethod { 
}

/// Methods for ProtectionZone_Vehicle on base component
pub enum ProtectionZone_VehicleMethod { 
}

/// Methods for ProtectionZoneController_Avatar on base component
pub enum ProtectionZoneController_AvatarMethod { 
}

/// Methods for PlayerMessenger_chat2 on base component
pub enum PlayerMessenger_chat2Method { 
    messenger_onActionByClient_chat2(i16, u16, GENERIC_MESSENGER_ARGS_chat2), // idx(1)
}

/// Methods for PlayLimits on base component
pub enum PlayLimitsMethod { 
}

/// Methods for PlaneTrajectoryArenaInfo on base component
pub enum PlaneTrajectoryArenaInfoMethod { 
}

/// Methods for Perks_Vehicle on base component
pub enum Perks_VehicleMethod { 
}

/// Methods for Invoicing on base component
pub enum InvoicingMethod { 
}

/// Methods for InvitationsClient on base component
pub enum InvitationsClientMethod { 
}

/// Methods for Invitations on base component
pub enum InvitationsMethod { 
}

/// Methods for InterclusterSender on base component
pub enum InterclusterSenderMethod { 
}

/// Methods for Harm on base component
pub enum HarmMethod { 
}

/// Methods for EntityTrap on base component
pub enum EntityTrapMethod { 
}

/// Methods for DestructibleEntity_Vehicle on base component
pub enum DestructibleEntity_VehicleMethod { 
}

/// Methods for DestructibleEntity_Avatar on base component
pub enum DestructibleEntity_AvatarMethod { 
}

/// Methods for Destructible on base component
pub enum DestructibleMethod { 
}

/// Methods for DefenderBonusController_Vehicle on base component
pub enum DefenderBonusController_VehicleMethod { 
}

/// Methods for ControlPoint on base component
pub enum ControlPointMethod { 
}

/// Methods for ClientCommandsPort on base component
pub enum ClientCommandsPortMethod { 
    doCmdNoArgs(i16, i16), // idx(0)
    doCmdStr(i16, i16, String), // idx(1)
    doCmdInt(i16, i16, i64), // idx(2)
    doCmdInt2(i16, i16, i64, i64), // idx(3)
    doCmdInt3(i16, i16, i64, i64, i64), // idx(4)
    doCmdInt4(i16, i16, i64, i64, i32, i32), // idx(5)
    doCmdInt2Str(i16, i16, i64, i64, String), // idx(6)
    doCmdInt3Str(i16, i16, i64, i64, i64, String), // idx(7)
    doCmdIntArr(i16, i16, Vec<i32>), // idx(8)
    doCmdIntStr(i16, i16, i64, String), // idx(9)
    doCmdIntStrArr(i16, i16, i64, Vec<String>), // idx(10)
    doCmdIntArrStrArr(i16, i16, Vec<i64>, Vec<String>), // idx(11)
    doCmdStrArr(i16, i16, Vec<String>), // idx(12)
}

/// Methods for Chat on base component
pub enum ChatMethod { 
    chatCommandFromClient(i64, u8, OBJECT_ID, i64, i16, String, String), // idx(4)
    inviteCommand(i64, u8, i8, String, i64, i16, String, String), // idx(6)
    ackCommand(i64, u8, f64, i64, i64), // idx(7)
    onStreamComplete(i16, BOOL), // idx(10)
}

/// Methods for BattleResultProcessor on base component
pub enum BattleResultProcessorMethod { 
}

/// Methods for BattleFeedback on base component
pub enum BattleFeedbackMethod { 
}

/// Methods for AvatarObserver on base component
pub enum AvatarObserverMethod { 
}

/// Methods for AvatarEpic on base component
pub enum AvatarEpicMethod { 
    enableFrontLineDevInfo(BOOL), // idx(0)
}

/// Methods for AvatarCreator on base component
pub enum AvatarCreatorMethod { 
}

/// Methods for AccountVersion on base component
pub enum AccountVersionMethod { 
}

/// Methods for AccountUnitRemote on base component
pub enum AccountUnitRemoteMethod { 
}

/// Methods for AccountUnitClient on base component
pub enum AccountUnitClientMethod { 
    accountUnitClient_create(i32, i32, i32, String, String), // idx(0)
    accountUnitClient_join(i32, u64, i32), // idx(1)
    accountUnitClient_doCmd(i32, OBJECT_ID, i32, u64, i32, String), // idx(2)
    accountUnitClient_sendInvites(i32, u64, Vec<DB_ID>, String), // idx(3)
    accountUnitClient_setRosterSlots(i32, u64, Vec<i32>, Vec<String>), // idx(4)
}

/// Methods for AccountUnitBrowser on base component
pub enum AccountUnitBrowserMethod { 
    accountUnitBrowser_subscribe(i16, BOOL), // idx(0)
    accountUnitBrowser_unsubscribe(), // idx(1)
    accountUnitBrowser_recenter(i32, i16, BOOL), // idx(2)
    accountUnitBrowser_doCmd(i32), // idx(3)
}

/// Methods for AccountUnitAssembler on base component
pub enum AccountUnitAssemblerMethod { 
}

/// Methods for AccountUnit on base component
pub enum AccountUnitMethod { 
}

/// Methods for AccountSysMessenger on base component
pub enum AccountSysMessengerMethod { 
}

/// Methods for AccountSpaProcessor on base component
pub enum AccountSpaProcessorMethod { 
}

/// Methods for AccountPrebattle on base component
pub enum AccountPrebattleMethod { 
    accountPrebattle_createTraining(i32, i32, BOOL, String), // idx(6)
    accountPrebattle_createDevPrebattle(u8, u8, i32, i32, String), // idx(7)
    accountPrebattle_sendPrebattleInvites(Vec<i64>, String), // idx(9)
}

/// Methods for AccountIGRProcessing on base component
pub enum AccountIGRProcessingMethod { 
}

/// Methods for AccountGlobalMapConnector on base component
pub enum AccountGlobalMapConnectorMethod { 
    accountGlobalMapConnector_callGlobalMapMethod(u64, i32, i64, String), // idx(0)
}

/// Methods for AccountEditor on base component
pub enum AccountEditorMethod { 
}

/// Methods for AccountDebugger on base component
pub enum AccountDebuggerMethod { 
    accountDebugger_registerDebugTaskResult(i64, i32, i64), // idx(1)
    accountDebugger_sendDebugTaskResultChunk(i64, i64, String), // idx(2)
}

/// Methods for AccountClan on base component
pub enum AccountClanMethod { 
}

/// Methods for AccountAvatar on base component
pub enum AccountAvatarMethod { 
    accountAvatar_sendAccountStats(u32, Vec<String>), // idx(4)
}

/// Methods for AccountAuthTokenProviderClient on base component
pub enum AccountAuthTokenProviderClientMethod { 
}

/// Methods for AccountAuthTokenProvider on base component
pub enum AccountAuthTokenProviderMethod { 
    requestToken(u16, u8), // idx(0)
}

/// Methods for AccountAdmin on base component
pub enum AccountAdminMethod { 
}

