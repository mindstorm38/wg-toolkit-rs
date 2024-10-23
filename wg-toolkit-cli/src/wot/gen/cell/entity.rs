use wgtk::net::app::common::element::Method;

use super::super::alias::*;
use super::interface::*;

/// Methods for Account on cell component
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
}


/// Methods for Avatar on cell component
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
    autoAim(OBJECT_ID, BOOL), // idx(0)
    moveTo(Vec3), // idx(1)
    bindToVehicle(OBJECT_ID), // idx(2)
    monitorVehicleDamagedDevices(OBJECT_ID), // idx(3)
    activateEquipment(u16, i16), // idx(14)
    activateVehicleEquipment(u16, String), // idx(15)
    setEquipmentApplicationPoint(u16, Vec3, Vec2), // idx(16)
    switchViewPointOrBindToVehicle(BOOL, OBJECT_ID), // idx(17)
    setDualGunCharger(BOOL), // idx(22)
    reportClientStats(CLIENT_STATUS_STATISTICS), // idx(23)
    vehicle_moveWith(u8), // idx(25)
    vehicle_shoot(), // idx(26)
    vehicle_trackWorldPointWithGun(Vec3), // idx(27)
    vehicle_trackRelativePointWithGun(Vec3), // idx(28)
    vehicle_stopTrackingWithGun(f32, f32), // idx(29)
    setupAmmo(i64), // idx(30)
    vehicle_changeSetting(u8, i32), // idx(31)
    setServerMarker(BOOL), // idx(32)
    setSendKillCamSimulationData(BOOL), // idx(33)
}

// 0: vehicle_shoot [Fixed(0)] @ []
// 1: setDualGunCharger [Fixed(1)] @ []
// 2: vehicle_moveWith [Fixed(1)] @ []
// 3: setServerMarker [Fixed(1)] @ []
// 4: setSendKillCamSimulationData [Fixed(1)] @ []
// 5: switchObserverFPV [Fixed(1)] @ ["AvatarObserver"]
// 6: bindToVehicle [Fixed(4)] @ []
// 7: monitorVehicleDamagedDevices [Fixed(4)] @ []
// 8: activateEquipment [Fixed(4)] @ []
// 9: autoAim [Fixed(5)] @ []
// 10: switchViewPointOrBindToVehicle [Fixed(5)] @ []
// 11: vehicle_changeSetting [Fixed(5)] @ []
// 12: vehicle_stopTrackingWithGun [Fixed(8)] @ []
// 13: setupAmmo [Fixed(8)] @ []
// 14: moveTo [Fixed(12)] @ []
// 15: vehicle_trackWorldPointWithGun [Fixed(12)] @ []
// 16: vehicle_trackRelativePointWithGun [Fixed(12)] @ []
// 17: setEquipmentApplicationPoint [Fixed(22)] @ []
// 18: reportClientStats [Fixed(24)] @ []
// 19: activateVehicleEquipment [Variable(Variable8)] @ []

/// Methods for ArenaInfo on cell component
pub enum ArenaInfoMethod { 
    PlaneTrajectoryArenaInfo(PlaneTrajectoryArenaInfoMethod),
}


/// Methods for ClientSelectableObject on cell component
pub enum ClientSelectableObjectMethod { 
}


/// Methods for HangarVehicle on cell component
pub enum HangarVehicleMethod { 
}


/// Methods for Vehicle on cell component
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
    moveWith(u8), // idx(1)
    trackWorldPointWithGun(Vec3), // idx(2)
    trackRelativePointWithGun(Vec3), // idx(3)
    stopTrackingWithGun(f32, f32), // idx(4)
    changeSetting(u8, i32), // idx(6)
    sendVisibilityDevelopmentInfo(OBJECT_ID, Vec3), // idx(7)
    sendStateToOwnClient(), // idx(16)
    switchSetup(u8, u8), // idx(40)
}

// 0: sendStateToOwnClient [Fixed(0)] @ []
// 1: recoveryMechanic_startRecovering [Fixed(0)] @ ["RecoveryMechanic_Vehicle"]
// 2: recoveryMechanic_stopRecovering [Fixed(0)] @ ["RecoveryMechanic_Vehicle"]
// 3: moveWith [Fixed(1)] @ []
// 4: switchSetup [Fixed(2)] @ []
// 5: changeSetting [Fixed(5)] @ []
// 6: stopTrackingWithGun [Fixed(8)] @ []
// 7: trackWorldPointWithGun [Fixed(12)] @ []
// 8: trackRelativePointWithGun [Fixed(12)] @ []
// 9: sendVisibilityDevelopmentInfo [Fixed(16)] @ []
// 10: setRemoteCamera [Fixed(22)] @ ["VehicleObserver"]

/// Methods for AreaDestructibles on cell component
pub enum AreaDestructiblesMethod { 
}


/// Methods for OfflineEntity on cell component
pub enum OfflineEntityMethod { 
}


/// Methods for Flock on cell component
pub enum FlockMethod { 
}


/// Methods for FlockExotic on cell component
pub enum FlockExoticMethod { 
}


/// Methods for Login on cell component
pub enum LoginMethod { 
}


/// Methods for DetachedTurret on cell component
pub enum DetachedTurretMethod { 
}


/// Methods for DebugDrawEntity on cell component
pub enum DebugDrawEntityMethod { 
}


/// Methods for ClientSelectableCameraObject on cell component
pub enum ClientSelectableCameraObjectMethod { 
}


/// Methods for ClientSelectableCameraVehicle on cell component
pub enum ClientSelectableCameraVehicleMethod { 
}


/// Methods for ClientSelectableWebLinksOpener on cell component
pub enum ClientSelectableWebLinksOpenerMethod { 
}


/// Methods for ClientSelectableEasterEgg on cell component
pub enum ClientSelectableEasterEggMethod { 
}


/// Methods for EmptyEntity on cell component
pub enum EmptyEntityMethod { 
}


/// Methods for LimitedVisibilityEntity on cell component
pub enum LimitedVisibilityEntityMethod { 
}


/// Methods for HeroTank on cell component
pub enum HeroTankMethod { 
}


/// Methods for PlatoonTank on cell component
pub enum PlatoonTankMethod { 
}


/// Methods for PlatoonLighting on cell component
pub enum PlatoonLightingMethod { 
}


/// Methods for SectorBase on cell component
pub enum SectorBaseMethod { 
    EntityTrap(EntityTrapMethod),
}


/// Methods for Sector on cell component
pub enum SectorMethod { 
}


/// Methods for DestructibleEntity on cell component
pub enum DestructibleEntityMethod { 
    Destructible(DestructibleMethod),
}


/// Methods for StepRepairPoint on cell component
pub enum StepRepairPointMethod { 
}


/// Methods for ProtectionZone on cell component
pub enum ProtectionZoneMethod { 
}


/// Methods for HangarPoster on cell component
pub enum HangarPosterMethod { 
}


/// Methods for TeamInfo on cell component
pub enum TeamInfoMethod { 
    ThrottledMethods(ThrottledMethodsMethod),
}


/// Methods for AvatarInfo on cell component
pub enum AvatarInfoMethod { 
}


/// Methods for ArenaObserverInfo on cell component
pub enum ArenaObserverInfoMethod { 
}


/// Methods for AreaOfEffect on cell component
pub enum AreaOfEffectMethod { 
}


/// Methods for AttackBomber on cell component
pub enum AttackBomberMethod { 
}


/// Methods for AttackArtilleryFort on cell component
pub enum AttackArtilleryFortMethod { 
}


/// Methods for PersonalDeathZone on cell component
pub enum PersonalDeathZoneMethod { 
}


/// Methods for ClientSelectableRankedObject on cell component
pub enum ClientSelectableRankedObjectMethod { 
}


/// Methods for SimulatedVehicle on cell component
pub enum SimulatedVehicleMethod { 
}


/// Methods for ClientSelectableHangarsSwitcher on cell component
pub enum ClientSelectableHangarsSwitcherMethod { 
}


/// Methods for StaticDeathZone on cell component
pub enum StaticDeathZoneMethod { 
}


/// Methods for BasicMine on cell component
pub enum BasicMineMethod { 
}


/// Methods for ApplicationPoint on cell component
pub enum ApplicationPointMethod { 
}


/// Methods for NetworkEntity on cell component
pub enum NetworkEntityMethod { 
}


/// Methods for Comp7Lighting on cell component
pub enum Comp7LightingMethod { 
}


/// Methods for EventVehicle on cell component
pub enum EventVehicleMethod { 
}


/// Methods for EventShowcaseVehicle on cell component
pub enum EventShowcaseVehicleMethod { 
}


/// Methods for EventPortal on cell component
pub enum EventPortalMethod { 
}


