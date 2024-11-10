
use super::super::alias::*;
use super::interface::*;

/// Entity 0x01
/// Methods for Account on base component
pub enum AccountMethod { 
    Chat(ChatMethod),
    PlayerMessenger_chat2(PlayerMessenger_chat2Method),
    ClientCommandsPort(ClientCommandsPortMethod),
    AccountAvatar(AccountAvatarMethod),
    AccountAuthTokenProvider(AccountAuthTokenProviderMethod),
    AccountPrebattle(AccountPrebattleMethod),
    AccountGlobalMapConnector(AccountGlobalMapConnectorMethod),
    AccountUnitClient(AccountUnitClientMethod),
    AccountUnitBrowser(AccountUnitBrowserMethod),
    AccountDebugger(AccountDebuggerMethod),
    makeDenunciation(DB_ID, i32, i8), // idx(30)
    banUnbanUser(DB_ID, u8, u32, String, i8), // idx(31)
    requestToken(u16, u8), // idx(32)
    logStreamCorruption(i16, i32, i32, i32, i32), // idx(36)
    setKickAtTime(i64, String, String), // idx(37)
}

// 0: accountUnitBrowser_unsubscribe [Fixed(0)] @ ["AccountUnitBrowser"]
// 1: onStreamComplete [Fixed(3)] @ ["Chat"]
// 2: requestToken [Fixed(3)] @ ["AccountAuthTokenProvider"]
// 3: accountUnitBrowser_subscribe [Fixed(3)] @ ["AccountUnitBrowser"]
// 4: requestToken [Fixed(3)] @ []
// 5: doCmdNoArgs [Fixed(4)] @ ["ClientCommandsPort"]
// 6: accountUnitBrowser_doCmd [Fixed(4)] @ ["AccountUnitBrowser"]
// 7: accountUnitBrowser_recenter [Fixed(7)] @ ["AccountUnitBrowser"]
// 8: doCmdInt [Fixed(12)] @ ["ClientCommandsPort"]
// 9: makeDenunciation [Fixed(13)] @ []
// 10: accountUnitClient_join [Fixed(16)] @ ["AccountUnitClient"]
// 11: logStreamCorruption [Fixed(18)] @ []
// 12: doCmdInt2 [Fixed(20)] @ ["ClientCommandsPort"]
// 13: accountDebugger_registerDebugTaskResult [Fixed(20)] @ ["AccountDebugger"]
// 14: doCmdInt3 [Fixed(28)] @ ["ClientCommandsPort"]
// 15: doCmdInt4 [Fixed(28)] @ ["ClientCommandsPort"]
// 16: ackCommand [Fixed(33)] @ ["Chat"]
// 17: chatCommandFromClient [Variable(Variable8)] @ ["Chat"]
// 18: inviteCommand [Variable(Variable8)] @ ["Chat"]
// 19: messenger_onActionByClient_chat2 [Variable(Variable8)] @ ["PlayerMessenger_chat2"]
// 20: doCmdStr [Variable(Variable8)] @ ["ClientCommandsPort"]
// 21: doCmdInt2Str [Variable(Variable8)] @ ["ClientCommandsPort"]
// 22: doCmdInt3Str [Variable(Variable8)] @ ["ClientCommandsPort"]
// 23: doCmdIntArr [Variable(Variable8)] @ ["ClientCommandsPort"]
// 24: doCmdIntStr [Variable(Variable8)] @ ["ClientCommandsPort"]
// 25: doCmdIntStrArr [Variable(Variable8)] @ ["ClientCommandsPort"]
// 26: doCmdIntArrStrArr [Variable(Variable8)] @ ["ClientCommandsPort"]
// 27: doCmdStrArr [Variable(Variable8)] @ ["ClientCommandsPort"]
// 28: accountAvatar_sendAccountStats [Variable(Variable8)] @ ["AccountAvatar"]
// 29: accountPrebattle_createTraining [Variable(Variable8)] @ ["AccountPrebattle"]
// 30: accountPrebattle_createDevPrebattle [Variable(Variable8)] @ ["AccountPrebattle"]
// 31: accountPrebattle_sendPrebattleInvites [Variable(Variable8)] @ ["AccountPrebattle"]
// 32: accountGlobalMapConnector_callGlobalMapMethod [Variable(Variable8)] @ ["AccountGlobalMapConnector"]
// 33: accountUnitClient_create [Variable(Variable8)] @ ["AccountUnitClient"]
// 34: accountUnitClient_doCmd [Variable(Variable8)] @ ["AccountUnitClient"]
// 35: accountUnitClient_sendInvites [Variable(Variable8)] @ ["AccountUnitClient"]
// 36: accountUnitClient_setRosterSlots [Variable(Variable8)] @ ["AccountUnitClient"]
// 37: accountDebugger_sendDebugTaskResultChunk [Variable(Variable8)] @ ["AccountDebugger"]
// 38: banUnbanUser [Variable(Variable8)] @ []
// 39: setKickAtTime [Variable(Variable8)] @ []

/// Entity 0x02
/// Methods for Avatar on base component
pub enum AvatarMethod { 
    Chat(ChatMethod),
    PlayerMessenger_chat2(PlayerMessenger_chat2Method),
    ClientCommandsPort(ClientCommandsPortMethod),
    RespawnController_Avatar(RespawnController_AvatarMethod),
    AvatarEpic(AvatarEpicMethod),
    logLag(), // idx(1)
    setClientReady(), // idx(3)
    leaveArena(), // idx(4)
    onLoginToCellFailed(), // idx(5)
    confirmBattleResultsReceiving(), // idx(7)
    makeDenunciation(OBJECT_ID, i32, i8), // idx(8)
    banUnbanUser(DB_ID, u8, u32, String, i8), // idx(9)
    requestToken(u16, u8), // idx(10)
    sendAccountStats(u32, Vec<String>), // idx(12)
    setClientCtx(String), // idx(13)
    vehicle_teleport(Vec3, f32), // idx(16)
    vehicle_replenishAmmo(), // idx(17)
    setDevelopmentFeature(OBJECT_ID, String, i32, String), // idx(23)
    addBotToArena(String, u8, String, Vec3, u8), // idx(24)
    receiveFakeShot(i32, f32, Vec3, Vec3, u8), // idx(25)
    logStreamCorruption(i16, i32, i32, i32, i32), // idx(26)
}

// 0: respawnController_performRespawn [Fixed(0)] @ ["RespawnController_Avatar"]
// 1: logLag [Fixed(0)] @ []
// 2: setClientReady [Fixed(0)] @ []
// 3: leaveArena [Fixed(0)] @ []
// 4: onLoginToCellFailed [Fixed(0)] @ []
// 5: confirmBattleResultsReceiving [Fixed(0)] @ []
// 6: vehicle_replenishAmmo [Fixed(0)] @ []
// 7: respawnController_requestRespawnGroupChange [Fixed(1)] @ ["RespawnController_Avatar"]
// 8: enableFrontLineDevInfo [Fixed(1)] @ ["AvatarEpic"]
// 9: respawnController_chooseVehicleForRespawn [Fixed(2)] @ ["RespawnController_Avatar"]
// 10: onStreamComplete [Fixed(3)] @ ["Chat"]
// 11: requestToken [Fixed(3)] @ []
// 12: doCmdNoArgs [Fixed(4)] @ ["ClientCommandsPort"]
// 13: respawnController_switchSetup [Fixed(4)] @ ["RespawnController_Avatar"]
// 14: makeDenunciation [Fixed(9)] @ []
// 15: doCmdInt [Fixed(12)] @ ["ClientCommandsPort"]
// 16: respawnController_chooseRespawnZone [Fixed(12)] @ ["RespawnController_Avatar"]
// 17: vehicle_teleport [Fixed(16)] @ []
// 18: logStreamCorruption [Fixed(18)] @ []
// 19: doCmdInt2 [Fixed(20)] @ ["ClientCommandsPort"]
// 20: doCmdInt3 [Fixed(28)] @ ["ClientCommandsPort"]
// 21: doCmdInt4 [Fixed(28)] @ ["ClientCommandsPort"]
// 22: ackCommand [Fixed(33)] @ ["Chat"]
// 23: receiveFakeShot [Fixed(33)] @ []
// 24: chatCommandFromClient [Variable(Variable8)] @ ["Chat"]
// 25: inviteCommand [Variable(Variable8)] @ ["Chat"]
// 26: messenger_onActionByClient_chat2 [Variable(Variable8)] @ ["PlayerMessenger_chat2"]
// 27: doCmdStr [Variable(Variable8)] @ ["ClientCommandsPort"]
// 28: doCmdInt2Str [Variable(Variable8)] @ ["ClientCommandsPort"]
// 29: doCmdInt3Str [Variable(Variable8)] @ ["ClientCommandsPort"]
// 30: doCmdIntArr [Variable(Variable8)] @ ["ClientCommandsPort"]
// 31: doCmdIntStr [Variable(Variable8)] @ ["ClientCommandsPort"]
// 32: doCmdIntStrArr [Variable(Variable8)] @ ["ClientCommandsPort"]
// 33: doCmdIntArrStrArr [Variable(Variable8)] @ ["ClientCommandsPort"]
// 34: doCmdStrArr [Variable(Variable8)] @ ["ClientCommandsPort"]
// 35: banUnbanUser [Variable(Variable8)] @ []
// 36: sendAccountStats [Variable(Variable8)] @ []
// 37: setClientCtx [Variable(Variable8)] @ []
// 38: setDevelopmentFeature [Variable(Variable8)] @ []
// 39: addBotToArena [Variable(Variable8)] @ []

/// Entity 0x03
/// Methods for ArenaInfo on base component
pub enum ArenaInfoMethod { 
}


/// Entity 0x04
/// Methods for ClientSelectableObject on base component
pub enum ClientSelectableObjectMethod { 
}


/// Entity 0x05
/// Methods for HangarVehicle on base component
pub enum HangarVehicleMethod { 
}


/// Entity 0x06
/// Methods for Vehicle on base component
pub enum VehicleMethod { 
}


/// Entity 0x07
/// Methods for AreaDestructibles on base component
pub enum AreaDestructiblesMethod { 
}


/// Entity 0x08
/// Methods for OfflineEntity on base component
pub enum OfflineEntityMethod { 
}


/// Entity 0x09
/// Methods for Flock on base component
pub enum FlockMethod { 
}


/// Entity 0x0A
/// Methods for FlockExotic on base component
pub enum FlockExoticMethod { 
}


/// Entity 0x0B
/// Methods for Login on base component
pub enum LoginMethod { 
}


/// Entity 0x0C
/// Methods for DetachedTurret on base component
pub enum DetachedTurretMethod { 
}


/// Entity 0x0D
/// Methods for DebugDrawEntity on base component
pub enum DebugDrawEntityMethod { 
}


/// Entity 0x0E
/// Methods for ClientSelectableCameraObject on base component
pub enum ClientSelectableCameraObjectMethod { 
}


/// Entity 0x0F
/// Methods for ClientSelectableCameraVehicle on base component
pub enum ClientSelectableCameraVehicleMethod { 
}


/// Entity 0x10
/// Methods for ClientSelectableWebLinksOpener on base component
pub enum ClientSelectableWebLinksOpenerMethod { 
}


/// Entity 0x11
/// Methods for ClientSelectableEasterEgg on base component
pub enum ClientSelectableEasterEggMethod { 
}


/// Entity 0x12
/// Methods for EmptyEntity on base component
pub enum EmptyEntityMethod { 
}


/// Entity 0x13
/// Methods for LimitedVisibilityEntity on base component
pub enum LimitedVisibilityEntityMethod { 
}


/// Entity 0x14
/// Methods for HeroTank on base component
pub enum HeroTankMethod { 
}


/// Entity 0x15
/// Methods for PlatoonTank on base component
pub enum PlatoonTankMethod { 
}


/// Entity 0x16
/// Methods for PlatoonLighting on base component
pub enum PlatoonLightingMethod { 
}


/// Entity 0x17
/// Methods for SectorBase on base component
pub enum SectorBaseMethod { 
}


/// Entity 0x18
/// Methods for Sector on base component
pub enum SectorMethod { 
}


/// Entity 0x19
/// Methods for DestructibleEntity on base component
pub enum DestructibleEntityMethod { 
}


/// Entity 0x1A
/// Methods for StepRepairPoint on base component
pub enum StepRepairPointMethod { 
}


/// Entity 0x1B
/// Methods for ProtectionZone on base component
pub enum ProtectionZoneMethod { 
}


/// Entity 0x1C
/// Methods for HangarPoster on base component
pub enum HangarPosterMethod { 
}


/// Entity 0x1D
/// Methods for TeamInfo on base component
pub enum TeamInfoMethod { 
}


/// Entity 0x1E
/// Methods for AvatarInfo on base component
pub enum AvatarInfoMethod { 
}


/// Entity 0x1F
/// Methods for ArenaObserverInfo on base component
pub enum ArenaObserverInfoMethod { 
}


/// Entity 0x20
/// Methods for AreaOfEffect on base component
pub enum AreaOfEffectMethod { 
}


/// Entity 0x21
/// Methods for AttackBomber on base component
pub enum AttackBomberMethod { 
}


/// Entity 0x22
/// Methods for AttackArtilleryFort on base component
pub enum AttackArtilleryFortMethod { 
}


/// Entity 0x23
/// Methods for PersonalDeathZone on base component
pub enum PersonalDeathZoneMethod { 
}


/// Entity 0x24
/// Methods for ClientSelectableRankedObject on base component
pub enum ClientSelectableRankedObjectMethod { 
}


/// Entity 0x25
/// Methods for SimulatedVehicle on base component
pub enum SimulatedVehicleMethod { 
}


/// Entity 0x26
/// Methods for ClientSelectableHangarsSwitcher on base component
pub enum ClientSelectableHangarsSwitcherMethod { 
}


/// Entity 0x27
/// Methods for StaticDeathZone on base component
pub enum StaticDeathZoneMethod { 
}


/// Entity 0x28
/// Methods for BasicMine on base component
pub enum BasicMineMethod { 
}


/// Entity 0x29
/// Methods for ApplicationPoint on base component
pub enum ApplicationPointMethod { 
}


/// Entity 0x2A
/// Methods for NetworkEntity on base component
pub enum NetworkEntityMethod { 
}


/// Entity 0x2B
/// Methods for Comp7Lighting on base component
pub enum Comp7LightingMethod { 
}


