use super::alias::*;
use super::{base, cell, client};

/// Interface Wheels
#[derive(Debug)]
pub struct Wheels {
    pub steeringAngles: Vec<u8>,
    pub wheelsScroll: Vec<u8>,
    pub wheelsState: u64,
    pub burnoutLevel: u8,
}

/// Interface VehiclesSpawnListStorage_Avatar
#[derive(Debug)]
pub struct VehiclesSpawnListStorage_Avatar {
}

/// Interface VehicleRemovalController_Avatar
#[derive(Debug)]
pub struct VehicleRemovalController_Avatar {
}

/// Interface VehicleObserver
#[derive(Debug)]
pub struct VehicleObserver {
    pub remoteCamera: REMOTE_CAMERA_DATA,
}

/// Interface VehicleHealthBroadcastListenerComponent_Avatar
#[derive(Debug)]
pub struct VehicleHealthBroadcastListenerComponent_Avatar {
}

/// Interface VehicleAIProxy
#[derive(Debug)]
pub struct VehicleAIProxy {
}

/// Interface TriggersController_Avatar
#[derive(Debug)]
pub struct TriggersController_Avatar {
}

/// Interface TransactionUser
#[derive(Debug)]
pub struct TransactionUser {
}

/// Interface ThrottledMethods
#[derive(Debug)]
pub struct ThrottledMethods {
}

/// Interface TeamHealthBar_Avatar
#[derive(Debug)]
pub struct TeamHealthBar_Avatar {
}

/// Interface TeamBase_Vehicle
#[derive(Debug)]
pub struct TeamBase_Vehicle {
}

/// Interface StepRepairPoint_Vehicle
#[derive(Debug)]
pub struct StepRepairPoint_Vehicle {
}

/// Interface SmokeController_Vehicle
#[derive(Debug)]
pub struct SmokeController_Vehicle {
}

/// Interface SessionTracker
#[derive(Debug)]
pub struct SessionTracker {
}

/// Interface ServerSideReplays
#[derive(Debug)]
pub struct ServerSideReplays {
}

/// Interface Sector_Vehicle
#[derive(Debug)]
pub struct Sector_Vehicle {
}

/// Interface SectorBase_Vehicle
#[derive(Debug)]
pub struct SectorBase_Vehicle {
}

/// Interface RespawnController_Vehicle
#[derive(Debug)]
pub struct RespawnController_Vehicle {
}

/// Interface RespawnController_Avatar
#[derive(Debug)]
pub struct RespawnController_Avatar {
}

/// Interface RepairBase_Vehicle
#[derive(Debug)]
pub struct RepairBase_Vehicle {
}

/// Interface RecoveryMechanic_Vehicle
#[derive(Debug)]
pub struct RecoveryMechanic_Vehicle {
}

/// Interface RecoveryMechanic_Avatar
#[derive(Debug)]
pub struct RecoveryMechanic_Avatar {
}

/// Interface QuestProcessor
#[derive(Debug)]
pub struct QuestProcessor {
}

/// Interface ProtectionZone_Vehicle
#[derive(Debug)]
pub struct ProtectionZone_Vehicle {
}

/// Interface ProtectionZoneController_Avatar
#[derive(Debug)]
pub struct ProtectionZoneController_Avatar {
}

/// Interface PlayerMessenger_chat2
#[derive(Debug)]
pub struct PlayerMessenger_chat2 {
}

/// Interface PlayLimits
#[derive(Debug)]
pub struct PlayLimits {
}

/// Interface PlaneTrajectoryArenaInfo
#[derive(Debug)]
pub struct PlaneTrajectoryArenaInfo {
    pub planeTrajectory: PLANE_TRAJECTORY,
}

/// Interface Perks_Vehicle
#[derive(Debug)]
pub struct Perks_Vehicle {
    pub perkEffects: ANON79,
    pub perks: Vec<PERK_INFO_HUD>,
    pub perksRibbonNotify: Vec<PERK_INFO_RIBBON>,
}

/// Interface Invoicing
#[derive(Debug)]
pub struct Invoicing {
}

/// Interface InvitationsClient
#[derive(Debug)]
pub struct InvitationsClient {
}

/// Interface Invitations
#[derive(Debug)]
pub struct Invitations {
}

/// Interface InterclusterSender
#[derive(Debug)]
pub struct InterclusterSender {
}

/// Interface Harm
#[derive(Debug)]
pub struct Harm {
}

/// Interface EntityTrap
#[derive(Debug)]
pub struct EntityTrap {
}

/// Interface DestructibleEntity_Vehicle
#[derive(Debug)]
pub struct DestructibleEntity_Vehicle {
}

/// Interface DestructibleEntity_Avatar
#[derive(Debug)]
pub struct DestructibleEntity_Avatar {
}

/// Interface Destructible
#[derive(Debug)]
pub struct Destructible {
}

/// Interface DefenderBonusController_Vehicle
#[derive(Debug)]
pub struct DefenderBonusController_Vehicle {
}

/// Interface ControlPoint
#[derive(Debug)]
pub struct ControlPoint {
}

/// Interface ClientCommandsPort
#[derive(Debug)]
pub struct ClientCommandsPort {
}

/// Interface Chat
#[derive(Debug)]
pub struct Chat {
}

/// Interface BattleResultProcessor
#[derive(Debug)]
pub struct BattleResultProcessor {
}

/// Interface BattleFeedback
#[derive(Debug)]
pub struct BattleFeedback {
}

/// Interface AvatarObserver
#[derive(Debug)]
pub struct AvatarObserver {
    pub remoteCamera: REMOTE_CAMERA_DATA,
    pub isObserverFPV: BOOL,
    pub numOfObservers: u8,
}

/// Interface AvatarEpic
#[derive(Debug)]
pub struct AvatarEpic {
}

/// Interface AvatarCreator
#[derive(Debug)]
pub struct AvatarCreator {
}

/// Interface AccountVersion
#[derive(Debug)]
pub struct AccountVersion {
    pub requiredVersion_12610: String,
}

/// Interface AccountUnitRemote
#[derive(Debug)]
pub struct AccountUnitRemote {
}

/// Interface AccountUnitClient
#[derive(Debug)]
pub struct AccountUnitClient {
}

/// Interface AccountUnitBrowser
#[derive(Debug)]
pub struct AccountUnitBrowser {
}

/// Interface AccountUnitAssembler
#[derive(Debug)]
pub struct AccountUnitAssembler {
}

/// Interface AccountUnit
#[derive(Debug)]
pub struct AccountUnit {
}

/// Interface AccountSysMessenger
#[derive(Debug)]
pub struct AccountSysMessenger {
}

/// Interface AccountSpaProcessor
#[derive(Debug)]
pub struct AccountSpaProcessor {
}

/// Interface AccountPrebattle
#[derive(Debug)]
pub struct AccountPrebattle {
}

/// Interface AccountIGRProcessing
#[derive(Debug)]
pub struct AccountIGRProcessing {
}

/// Interface AccountGlobalMapConnector
#[derive(Debug)]
pub struct AccountGlobalMapConnector {
}

/// Interface AccountEditor
#[derive(Debug)]
pub struct AccountEditor {
}

/// Interface AccountDebugger
#[derive(Debug)]
pub struct AccountDebugger {
}

/// Interface AccountClan
#[derive(Debug)]
pub struct AccountClan {
}

/// Interface AccountAvatar
#[derive(Debug)]
pub struct AccountAvatar {
}

/// Interface AccountAuthTokenProviderClient
#[derive(Debug)]
pub struct AccountAuthTokenProviderClient {
}

/// Interface AccountAuthTokenProvider
#[derive(Debug)]
pub struct AccountAuthTokenProvider {
}

/// Interface AccountAdmin
#[derive(Debug)]
pub struct AccountAdmin {
}

