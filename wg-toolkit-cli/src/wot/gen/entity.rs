use wgtk::net::app::common::element::Entity;

use super::alias::*;
use super::interface::*;
use super::{base, cell, client};

/// Interface Account
#[derive(Debug, Default)]
pub struct Account {
    pub i_Chat: Chat,
    pub i_PlayerMessenger_chat2: PlayerMessenger_chat2,
    pub i_AccountEditor: AccountEditor,
    pub i_TransactionUser: TransactionUser,
    pub i_InterclusterSender: InterclusterSender,
    pub i_ClientCommandsPort: ClientCommandsPort,
    pub i_AccountAdmin: AccountAdmin,
    pub i_AccountAvatar: AccountAvatar,
    pub i_AccountClan: AccountClan,
    pub i_AccountAuthTokenProvider: AccountAuthTokenProvider,
    pub i_AccountAuthTokenProviderClient: AccountAuthTokenProviderClient,
    pub i_BattleResultProcessor: BattleResultProcessor,
    pub i_Invitations: Invitations,
    pub i_InvitationsClient: InvitationsClient,
    pub i_Invoicing: Invoicing,
    pub i_AccountPrebattle: AccountPrebattle,
    pub i_AccountSpaProcessor: AccountSpaProcessor,
    pub i_AccountIGRProcessing: AccountIGRProcessing,
    pub i_SessionTracker: SessionTracker,
    pub i_AccountGlobalMapConnector: AccountGlobalMapConnector,
    pub i_AccountSysMessenger: AccountSysMessenger,
    pub i_AccountUnit: AccountUnit,
    pub i_AccountUnitClient: AccountUnitClient,
    pub i_AccountUnitRemote: AccountUnitRemote,
    pub i_AccountUnitAssembler: AccountUnitAssembler,
    pub i_AccountUnitBrowser: AccountUnitBrowser,
    pub i_AccountDebugger: AccountDebugger,
    pub i_QuestProcessor: QuestProcessor,
    pub i_AvatarCreator: AvatarCreator,
    pub i_AccountVersion: AccountVersion,
    pub i_PlayLimits: PlayLimits,
    pub i_ServerSideReplays: ServerSideReplays,
    pub i_EventTokensController: EventTokensController,
}

/// Interface Avatar
#[derive(Debug, Default)]
pub struct Avatar {
    pub i_Chat: Chat,
    pub i_PlayerMessenger_chat2: PlayerMessenger_chat2,
    pub i_ClientCommandsPort: ClientCommandsPort,
    pub i_InvitationsClient: InvitationsClient,
    pub i_AccountAuthTokenProviderClient: AccountAuthTokenProviderClient,
    pub i_AvatarObserver: AvatarObserver,
    pub i_TeamHealthBar_Avatar: TeamHealthBar_Avatar,
    pub i_ProtectionZoneController_Avatar: ProtectionZoneController_Avatar,
    pub i_RecoveryMechanic_Avatar: RecoveryMechanic_Avatar,
    pub i_DestructibleEntity_Avatar: DestructibleEntity_Avatar,
    pub i_RespawnController_Avatar: RespawnController_Avatar,
    pub i_VehiclesSpawnListStorage_Avatar: VehiclesSpawnListStorage_Avatar,
    pub i_VehicleRemovalController_Avatar: VehicleRemovalController_Avatar,
    pub i_VehicleHealthBroadcastListenerComponent_Avatar: VehicleHealthBroadcastListenerComponent_Avatar,
    pub i_TriggersController_Avatar: TriggersController_Avatar,
    pub i_AvatarEpic: AvatarEpic,
}

/// Interface ArenaInfo
#[derive(Debug, Default)]
pub struct ArenaInfo {
    pub i_PlaneTrajectoryArenaInfo: PlaneTrajectoryArenaInfo,
}

/// Interface ClientSelectableObject
#[derive(Debug, Default)]
pub struct ClientSelectableObject {
}

/// Interface HangarVehicle
#[derive(Debug, Default)]
pub struct HangarVehicle {
}

/// Interface Vehicle
#[derive(Debug, Default)]
pub struct Vehicle {
    pub i_VehicleAIProxy: VehicleAIProxy,
    pub i_TeamBase_Vehicle: TeamBase_Vehicle,
    pub i_SectorBase_Vehicle: SectorBase_Vehicle,
    pub i_RepairBase_Vehicle: RepairBase_Vehicle,
    pub i_VehicleObserver: VehicleObserver,
    pub i_BattleFeedback: BattleFeedback,
    pub i_Harm: Harm,
    pub i_Sector_Vehicle: Sector_Vehicle,
    pub i_ProtectionZone_Vehicle: ProtectionZone_Vehicle,
    pub i_StepRepairPoint_Vehicle: StepRepairPoint_Vehicle,
    pub i_DestructibleEntity_Vehicle: DestructibleEntity_Vehicle,
    pub i_DefenderBonusController_Vehicle: DefenderBonusController_Vehicle,
    pub i_RecoveryMechanic_Vehicle: RecoveryMechanic_Vehicle,
    pub i_RespawnController_Vehicle: RespawnController_Vehicle,
    pub i_SmokeController_Vehicle: SmokeController_Vehicle,
    pub i_Wheels: Wheels,
    pub i_Perks_Vehicle: Perks_Vehicle,
}

/// Interface AreaDestructibles
#[derive(Debug, Default)]
pub struct AreaDestructibles {
}

/// Interface OfflineEntity
#[derive(Debug, Default)]
pub struct OfflineEntity {
}

/// Interface Flock
#[derive(Debug, Default)]
pub struct Flock {
}

/// Interface FlockExotic
#[derive(Debug, Default)]
pub struct FlockExotic {
}

/// Interface Login
#[derive(Debug, Default)]
pub struct Login {
}

/// Interface DetachedTurret
#[derive(Debug, Default)]
pub struct DetachedTurret {
}

/// Interface DebugDrawEntity
#[derive(Debug, Default)]
pub struct DebugDrawEntity {
}

/// Interface ClientSelectableCameraObject
#[derive(Debug, Default)]
pub struct ClientSelectableCameraObject {
}

/// Interface ClientSelectableCameraVehicle
#[derive(Debug, Default)]
pub struct ClientSelectableCameraVehicle {
}

/// Interface ClientSelectableWebLinksOpener
#[derive(Debug, Default)]
pub struct ClientSelectableWebLinksOpener {
}

/// Interface ClientSelectableEasterEgg
#[derive(Debug, Default)]
pub struct ClientSelectableEasterEgg {
}

/// Interface EmptyEntity
#[derive(Debug, Default)]
pub struct EmptyEntity {
}

/// Interface LimitedVisibilityEntity
#[derive(Debug, Default)]
pub struct LimitedVisibilityEntity {
}

/// Interface HeroTank
#[derive(Debug, Default)]
pub struct HeroTank {
}

/// Interface PlatoonTank
#[derive(Debug, Default)]
pub struct PlatoonTank {
}

/// Interface PlatoonLighting
#[derive(Debug, Default)]
pub struct PlatoonLighting {
}

/// Interface SectorBase
#[derive(Debug, Default)]
pub struct SectorBase {
    pub i_EntityTrap: EntityTrap,
}

/// Interface Sector
#[derive(Debug, Default)]
pub struct Sector {
}

/// Interface DestructibleEntity
#[derive(Debug, Default)]
pub struct DestructibleEntity {
    pub i_Destructible: Destructible,
}

/// Interface StepRepairPoint
#[derive(Debug, Default)]
pub struct StepRepairPoint {
}

/// Interface ProtectionZone
#[derive(Debug, Default)]
pub struct ProtectionZone {
}

/// Interface HangarPoster
#[derive(Debug, Default)]
pub struct HangarPoster {
}

/// Interface TeamInfo
#[derive(Debug, Default)]
pub struct TeamInfo {
    pub i_ThrottledMethods: ThrottledMethods,
}

/// Interface AvatarInfo
#[derive(Debug, Default)]
pub struct AvatarInfo {
}

/// Interface ArenaObserverInfo
#[derive(Debug, Default)]
pub struct ArenaObserverInfo {
}

/// Interface AreaOfEffect
#[derive(Debug, Default)]
pub struct AreaOfEffect {
}

/// Interface AttackBomber
#[derive(Debug, Default)]
pub struct AttackBomber {
}

/// Interface AttackArtilleryFort
#[derive(Debug, Default)]
pub struct AttackArtilleryFort {
}

/// Interface PersonalDeathZone
#[derive(Debug, Default)]
pub struct PersonalDeathZone {
}

/// Interface ClientSelectableRankedObject
#[derive(Debug, Default)]
pub struct ClientSelectableRankedObject {
}

/// Interface SimulatedVehicle
#[derive(Debug, Default)]
pub struct SimulatedVehicle {
}

/// Interface ClientSelectableHangarsSwitcher
#[derive(Debug, Default)]
pub struct ClientSelectableHangarsSwitcher {
}

/// Interface StaticDeathZone
#[derive(Debug, Default)]
pub struct StaticDeathZone {
}

/// Interface BasicMine
#[derive(Debug, Default)]
pub struct BasicMine {
}

/// Interface ApplicationPoint
#[derive(Debug, Default)]
pub struct ApplicationPoint {
}

/// Interface NetworkEntity
#[derive(Debug, Default)]
pub struct NetworkEntity {
}

/// Interface Comp7Lighting
#[derive(Debug, Default)]
pub struct Comp7Lighting {
}

/// Interface EventVehicle
#[derive(Debug, Default)]
pub struct EventVehicle {
}

/// Interface EventShowcaseVehicle
#[derive(Debug, Default)]
pub struct EventShowcaseVehicle {
}

/// Interface EventPortal
#[derive(Debug, Default)]
pub struct EventPortal {
}

