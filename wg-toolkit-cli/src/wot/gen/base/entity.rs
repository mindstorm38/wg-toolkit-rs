use wgtk::net::app::common::element::Method;

use super::super::alias::*;
use super::interface::*;

/// Methods for Account on base component
pub enum AccountMethod { 
    Chat(ChatMethod),
    PlayerMessenger_chat2(PlayerMessenger_chat2Method),
    AccountEditor(AccountEditorMethod),
    TransactionUser(TransactionUserMethod),
    InterclusterSender(InterclusterSenderMethod),
    ClientCommandsPort(ClientCommandsPortMethod),
    AccountAdmin(AccountAdminMethod),
    AccountAvatar(AccountAvatarMethod),
    AccountClan(AccountClanMethod),
    AccountAuthTokenProvider(AccountAuthTokenProviderMethod),
    AccountAuthTokenProviderClient(AccountAuthTokenProviderClientMethod),
    BattleResultProcessor(BattleResultProcessorMethod),
    Invitations(InvitationsMethod),
    InvitationsClient(InvitationsClientMethod),
    Invoicing(InvoicingMethod),
    AccountPrebattle(AccountPrebattleMethod),
    AccountSpaProcessor(AccountSpaProcessorMethod),
    AccountIGRProcessing(AccountIGRProcessingMethod),
    SessionTracker(SessionTrackerMethod),
    AccountGlobalMapConnector(AccountGlobalMapConnectorMethod),
    AccountSysMessenger(AccountSysMessengerMethod),
    AccountUnit(AccountUnitMethod),
    AccountUnitClient(AccountUnitClientMethod),
    AccountUnitRemote(AccountUnitRemoteMethod),
    AccountUnitAssembler(AccountUnitAssemblerMethod),
    AccountUnitBrowser(AccountUnitBrowserMethod),
    AccountDebugger(AccountDebuggerMethod),
    QuestProcessor(QuestProcessorMethod),
    AvatarCreator(AvatarCreatorMethod),
    AccountVersion(AccountVersionMethod),
    PlayLimits(PlayLimitsMethod),
    ServerSideReplays(ServerSideReplaysMethod),
    EventTokensController(EventTokensControllerMethod),
    makeDenunciation(DB_ID, i32, i8), // idx(24)
    banUnbanUser(DB_ID, u8, u32, String, i8), // idx(25)
    requestToken(u16, u8), // idx(26)
    logStreamCorruption(i16, i32, i32, i32, i32), // idx(29)
    setKickAtTime(i64, String, String), // idx(30)
}

// 0: accountUnitBrowser_unsubscribe [Fixed(0)] @ ["AccountUnitBrowser"]
// 1: requestToken [Fixed(3)] @ []
// 2: onStreamComplete [Fixed(3)] @ ["Chat"]
// 3: requestToken [Fixed(3)] @ ["AccountAuthTokenProvider"]
// 4: accountUnitBrowser_subscribe [Fixed(3)] @ ["AccountUnitBrowser"]
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
// 17: banUnbanUser [Variable(Variable8)] @ []
// 18: setKickAtTime [Variable(Variable8)] @ []
// 19: chatCommandFromClient [Variable(Variable8)] @ ["Chat"]
// 20: inviteCommand [Variable(Variable8)] @ ["Chat"]
// 21: messenger_onActionByClient_chat2 [Variable(Variable8)] @ ["PlayerMessenger_chat2"]
// 22: doCmdStr [Variable(Variable8)] @ ["ClientCommandsPort"]
// 23: doCmdInt2Str [Variable(Variable8)] @ ["ClientCommandsPort"]
// 24: doCmdInt3Str [Variable(Variable8)] @ ["ClientCommandsPort"]
// 25: doCmdIntArr [Variable(Variable8)] @ ["ClientCommandsPort"]
// 26: doCmdIntStr [Variable(Variable8)] @ ["ClientCommandsPort"]
// 27: doCmdIntStrArr [Variable(Variable8)] @ ["ClientCommandsPort"]
// 28: doCmdIntArrStrArr [Variable(Variable8)] @ ["ClientCommandsPort"]
// 29: doCmdStrArr [Variable(Variable8)] @ ["ClientCommandsPort"]
// 30: accountAvatar_sendAccountStats [Variable(Variable8)] @ ["AccountAvatar"]
// 31: accountPrebattle_createTraining [Variable(Variable8)] @ ["AccountPrebattle"]
// 32: accountPrebattle_createDevPrebattle [Variable(Variable8)] @ ["AccountPrebattle"]
// 33: accountPrebattle_sendPrebattleInvites [Variable(Variable8)] @ ["AccountPrebattle"]
// 34: accountGlobalMapConnector_callGlobalMapMethod [Variable(Variable8)] @ ["AccountGlobalMapConnector"]
// 35: accountUnitClient_create [Variable(Variable8)] @ ["AccountUnitClient"]
// 36: accountUnitClient_doCmd [Variable(Variable8)] @ ["AccountUnitClient"]
// 37: accountUnitClient_sendInvites [Variable(Variable8)] @ ["AccountUnitClient"]
// 38: accountUnitClient_setRosterSlots [Variable(Variable8)] @ ["AccountUnitClient"]
// 39: accountDebugger_sendDebugTaskResultChunk [Variable(Variable8)] @ ["AccountDebugger"]

/// Methods for Avatar on base component
pub enum AvatarMethod { 
    Chat(ChatMethod),
    PlayerMessenger_chat2(PlayerMessenger_chat2Method),
    ClientCommandsPort(ClientCommandsPortMethod),
    InvitationsClient(InvitationsClientMethod),
    AccountAuthTokenProviderClient(AccountAuthTokenProviderClientMethod),
    AvatarObserver(AvatarObserverMethod),
    TeamHealthBar_Avatar(TeamHealthBar_AvatarMethod),
    ProtectionZoneController_Avatar(ProtectionZoneController_AvatarMethod),
    RecoveryMechanic_Avatar(RecoveryMechanic_AvatarMethod),
    DestructibleEntity_Avatar(DestructibleEntity_AvatarMethod),
    RespawnController_Avatar(RespawnController_AvatarMethod),
    VehiclesSpawnListStorage_Avatar(VehiclesSpawnListStorage_AvatarMethod),
    VehicleRemovalController_Avatar(VehicleRemovalController_AvatarMethod),
    VehicleHealthBroadcastListenerComponent_Avatar(VehicleHealthBroadcastListenerComponent_AvatarMethod),
    TriggersController_Avatar(TriggersController_AvatarMethod),
    AvatarEpic(AvatarEpicMethod),
    logLag(), // idx(0)
    setClientReady(), // idx(2)
    leaveArena(), // idx(3)
    onLoginToCellFailed(), // idx(4)
    confirmBattleResultsReceiving(), // idx(6)
    makeDenunciation(OBJECT_ID, i32, i8), // idx(7)
    banUnbanUser(DB_ID, u8, u32, String, i8), // idx(8)
    requestToken(u16, u8), // idx(9)
    sendAccountStats(u32, Vec<String>), // idx(10)
    setClientCtx(String), // idx(11)
    vehicle_teleport(Vec3, f32), // idx(14)
    vehicle_replenishAmmo(), // idx(15)
    setDevelopmentFeature(OBJECT_ID, String, i32, String), // idx(19)
    addBotToArena(String, u8, String, Vec3, u8), // idx(20)
    receiveFakeShot(i32, f32, Vec3, Vec3, u8), // idx(21)
    logStreamCorruption(i16, i32, i32, i32, i32), // idx(22)
}

// 0: logLag [Fixed(0)] @ []
// 1: setClientReady [Fixed(0)] @ []
// 2: leaveArena [Fixed(0)] @ []
// 3: onLoginToCellFailed [Fixed(0)] @ []
// 4: confirmBattleResultsReceiving [Fixed(0)] @ []
// 5: vehicle_replenishAmmo [Fixed(0)] @ []
// 6: respawnController_performRespawn [Fixed(0)] @ ["RespawnController_Avatar"]
// 7: respawnController_requestRespawnGroupChange [Fixed(1)] @ ["RespawnController_Avatar"]
// 8: enableFrontLineDevInfo [Fixed(1)] @ ["AvatarEpic"]
// 9: respawnController_chooseVehicleForRespawn [Fixed(2)] @ ["RespawnController_Avatar"]
// 10: requestToken [Fixed(3)] @ []
// 11: onStreamComplete [Fixed(3)] @ ["Chat"]
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
// 22: receiveFakeShot [Fixed(33)] @ []
// 23: ackCommand [Fixed(33)] @ ["Chat"]
// 24: banUnbanUser [Variable(Variable8)] @ []
// 25: sendAccountStats [Variable(Variable8)] @ []
// 26: setClientCtx [Variable(Variable8)] @ []
// 27: setDevelopmentFeature [Variable(Variable8)] @ []
// 28: addBotToArena [Variable(Variable8)] @ []
// 29: chatCommandFromClient [Variable(Variable8)] @ ["Chat"]
// 30: inviteCommand [Variable(Variable8)] @ ["Chat"]
// 31: messenger_onActionByClient_chat2 [Variable(Variable8)] @ ["PlayerMessenger_chat2"]
// 32: doCmdStr [Variable(Variable8)] @ ["ClientCommandsPort"]
// 33: doCmdInt2Str [Variable(Variable8)] @ ["ClientCommandsPort"]
// 34: doCmdInt3Str [Variable(Variable8)] @ ["ClientCommandsPort"]
// 35: doCmdIntArr [Variable(Variable8)] @ ["ClientCommandsPort"]
// 36: doCmdIntStr [Variable(Variable8)] @ ["ClientCommandsPort"]
// 37: doCmdIntStrArr [Variable(Variable8)] @ ["ClientCommandsPort"]
// 38: doCmdIntArrStrArr [Variable(Variable8)] @ ["ClientCommandsPort"]
// 39: doCmdStrArr [Variable(Variable8)] @ ["ClientCommandsPort"]

/// Methods for ArenaInfo on base component
pub enum ArenaInfoMethod { 
    PlaneTrajectoryArenaInfo(PlaneTrajectoryArenaInfoMethod),
}


/// Methods for ClientSelectableObject on base component
pub enum ClientSelectableObjectMethod { 
}


/// Methods for HangarVehicle on base component
pub enum HangarVehicleMethod { 
}


/// Methods for Vehicle on base component
pub enum VehicleMethod { 
    VehicleAIProxy(VehicleAIProxyMethod),
    TeamBase_Vehicle(TeamBase_VehicleMethod),
    SectorBase_Vehicle(SectorBase_VehicleMethod),
    RepairBase_Vehicle(RepairBase_VehicleMethod),
    VehicleObserver(VehicleObserverMethod),
    BattleFeedback(BattleFeedbackMethod),
    Harm(HarmMethod),
    Sector_Vehicle(Sector_VehicleMethod),
    ProtectionZone_Vehicle(ProtectionZone_VehicleMethod),
    StepRepairPoint_Vehicle(StepRepairPoint_VehicleMethod),
    DestructibleEntity_Vehicle(DestructibleEntity_VehicleMethod),
    DefenderBonusController_Vehicle(DefenderBonusController_VehicleMethod),
    RecoveryMechanic_Vehicle(RecoveryMechanic_VehicleMethod),
    RespawnController_Vehicle(RespawnController_VehicleMethod),
    SmokeController_Vehicle(SmokeController_VehicleMethod),
    Wheels(WheelsMethod),
    Perks_Vehicle(Perks_VehicleMethod),
}


/// Methods for AreaDestructibles on base component
pub enum AreaDestructiblesMethod { 
}


/// Methods for OfflineEntity on base component
pub enum OfflineEntityMethod { 
}


/// Methods for Flock on base component
pub enum FlockMethod { 
}


/// Methods for FlockExotic on base component
pub enum FlockExoticMethod { 
}


/// Methods for Login on base component
pub enum LoginMethod { 
}


/// Methods for DetachedTurret on base component
pub enum DetachedTurretMethod { 
}


/// Methods for DebugDrawEntity on base component
pub enum DebugDrawEntityMethod { 
}


/// Methods for ClientSelectableCameraObject on base component
pub enum ClientSelectableCameraObjectMethod { 
}


/// Methods for ClientSelectableCameraVehicle on base component
pub enum ClientSelectableCameraVehicleMethod { 
}


/// Methods for ClientSelectableWebLinksOpener on base component
pub enum ClientSelectableWebLinksOpenerMethod { 
}


/// Methods for ClientSelectableEasterEgg on base component
pub enum ClientSelectableEasterEggMethod { 
}


/// Methods for EmptyEntity on base component
pub enum EmptyEntityMethod { 
}


/// Methods for LimitedVisibilityEntity on base component
pub enum LimitedVisibilityEntityMethod { 
}


/// Methods for HeroTank on base component
pub enum HeroTankMethod { 
}


/// Methods for PlatoonTank on base component
pub enum PlatoonTankMethod { 
}


/// Methods for PlatoonLighting on base component
pub enum PlatoonLightingMethod { 
}


/// Methods for SectorBase on base component
pub enum SectorBaseMethod { 
    EntityTrap(EntityTrapMethod),
}


/// Methods for Sector on base component
pub enum SectorMethod { 
}


/// Methods for DestructibleEntity on base component
pub enum DestructibleEntityMethod { 
    Destructible(DestructibleMethod),
}


/// Methods for StepRepairPoint on base component
pub enum StepRepairPointMethod { 
}


/// Methods for ProtectionZone on base component
pub enum ProtectionZoneMethod { 
}


/// Methods for HangarPoster on base component
pub enum HangarPosterMethod { 
}


/// Methods for TeamInfo on base component
pub enum TeamInfoMethod { 
    ThrottledMethods(ThrottledMethodsMethod),
}


/// Methods for AvatarInfo on base component
pub enum AvatarInfoMethod { 
}


/// Methods for ArenaObserverInfo on base component
pub enum ArenaObserverInfoMethod { 
}


/// Methods for AreaOfEffect on base component
pub enum AreaOfEffectMethod { 
}


/// Methods for AttackBomber on base component
pub enum AttackBomberMethod { 
}


/// Methods for AttackArtilleryFort on base component
pub enum AttackArtilleryFortMethod { 
}


/// Methods for PersonalDeathZone on base component
pub enum PersonalDeathZoneMethod { 
}


/// Methods for ClientSelectableRankedObject on base component
pub enum ClientSelectableRankedObjectMethod { 
}


/// Methods for SimulatedVehicle on base component
pub enum SimulatedVehicleMethod { 
}


/// Methods for ClientSelectableHangarsSwitcher on base component
pub enum ClientSelectableHangarsSwitcherMethod { 
}


/// Methods for StaticDeathZone on base component
pub enum StaticDeathZoneMethod { 
}


/// Methods for BasicMine on base component
pub enum BasicMineMethod { 
}


/// Methods for ApplicationPoint on base component
pub enum ApplicationPointMethod { 
}


/// Methods for NetworkEntity on base component
pub enum NetworkEntityMethod { 
}


/// Methods for Comp7Lighting on base component
pub enum Comp7LightingMethod { 
}


/// Methods for EventVehicle on base component
pub enum EventVehicleMethod { 
}


/// Methods for EventShowcaseVehicle on base component
pub enum EventShowcaseVehicleMethod { 
}


/// Methods for EventPortal on base component
pub enum EventPortalMethod { 
}


