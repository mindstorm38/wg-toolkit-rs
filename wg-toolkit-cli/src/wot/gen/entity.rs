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

impl Entity for Account {
    const ID: u16 = 1;
    type ClientMethod = client::entity::AccountMethod;
    type BaseMethod = base::entity::AccountMethod;
    type CellMethod = cell::entity::AccountMethod;
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

impl Entity for Avatar {
    const ID: u16 = 2;
    type ClientMethod = client::entity::AvatarMethod;
    type BaseMethod = base::entity::AvatarMethod;
    type CellMethod = cell::entity::AvatarMethod;
}

/// Interface ArenaInfo
#[derive(Debug, Default)]
pub struct ArenaInfo {
    pub i_PlaneTrajectoryArenaInfo: PlaneTrajectoryArenaInfo,
}

impl Entity for ArenaInfo {
    const ID: u16 = 3;
    type ClientMethod = client::entity::ArenaInfoMethod;
    type BaseMethod = base::entity::ArenaInfoMethod;
    type CellMethod = cell::entity::ArenaInfoMethod;
}

/// Interface ClientSelectableObject
#[derive(Debug, Default)]
pub struct ClientSelectableObject {
}

impl Entity for ClientSelectableObject {
    const ID: u16 = 4;
    type ClientMethod = client::entity::ClientSelectableObjectMethod;
    type BaseMethod = base::entity::ClientSelectableObjectMethod;
    type CellMethod = cell::entity::ClientSelectableObjectMethod;
}

/// Interface HangarVehicle
#[derive(Debug, Default)]
pub struct HangarVehicle {
}

impl Entity for HangarVehicle {
    const ID: u16 = 5;
    type ClientMethod = client::entity::HangarVehicleMethod;
    type BaseMethod = base::entity::HangarVehicleMethod;
    type CellMethod = cell::entity::HangarVehicleMethod;
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

impl Entity for Vehicle {
    const ID: u16 = 6;
    type ClientMethod = client::entity::VehicleMethod;
    type BaseMethod = base::entity::VehicleMethod;
    type CellMethod = cell::entity::VehicleMethod;
}

/// Interface AreaDestructibles
#[derive(Debug, Default)]
pub struct AreaDestructibles {
}

impl Entity for AreaDestructibles {
    const ID: u16 = 7;
    type ClientMethod = client::entity::AreaDestructiblesMethod;
    type BaseMethod = base::entity::AreaDestructiblesMethod;
    type CellMethod = cell::entity::AreaDestructiblesMethod;
}

/// Interface OfflineEntity
#[derive(Debug, Default)]
pub struct OfflineEntity {
}

impl Entity for OfflineEntity {
    const ID: u16 = 8;
    type ClientMethod = client::entity::OfflineEntityMethod;
    type BaseMethod = base::entity::OfflineEntityMethod;
    type CellMethod = cell::entity::OfflineEntityMethod;
}

/// Interface Flock
#[derive(Debug, Default)]
pub struct Flock {
}

impl Entity for Flock {
    const ID: u16 = 9;
    type ClientMethod = client::entity::FlockMethod;
    type BaseMethod = base::entity::FlockMethod;
    type CellMethod = cell::entity::FlockMethod;
}

/// Interface FlockExotic
#[derive(Debug, Default)]
pub struct FlockExotic {
}

impl Entity for FlockExotic {
    const ID: u16 = 10;
    type ClientMethod = client::entity::FlockExoticMethod;
    type BaseMethod = base::entity::FlockExoticMethod;
    type CellMethod = cell::entity::FlockExoticMethod;
}

/// Interface Login
#[derive(Debug, Default)]
pub struct Login {
}

impl Entity for Login {
    const ID: u16 = 11;
    type ClientMethod = client::entity::LoginMethod;
    type BaseMethod = base::entity::LoginMethod;
    type CellMethod = cell::entity::LoginMethod;
}

/// Interface DetachedTurret
#[derive(Debug, Default)]
pub struct DetachedTurret {
}

impl Entity for DetachedTurret {
    const ID: u16 = 12;
    type ClientMethod = client::entity::DetachedTurretMethod;
    type BaseMethod = base::entity::DetachedTurretMethod;
    type CellMethod = cell::entity::DetachedTurretMethod;
}

/// Interface DebugDrawEntity
#[derive(Debug, Default)]
pub struct DebugDrawEntity {
}

impl Entity for DebugDrawEntity {
    const ID: u16 = 13;
    type ClientMethod = client::entity::DebugDrawEntityMethod;
    type BaseMethod = base::entity::DebugDrawEntityMethod;
    type CellMethod = cell::entity::DebugDrawEntityMethod;
}

/// Interface ClientSelectableCameraObject
#[derive(Debug, Default)]
pub struct ClientSelectableCameraObject {
}

impl Entity for ClientSelectableCameraObject {
    const ID: u16 = 14;
    type ClientMethod = client::entity::ClientSelectableCameraObjectMethod;
    type BaseMethod = base::entity::ClientSelectableCameraObjectMethod;
    type CellMethod = cell::entity::ClientSelectableCameraObjectMethod;
}

/// Interface ClientSelectableCameraVehicle
#[derive(Debug, Default)]
pub struct ClientSelectableCameraVehicle {
}

impl Entity for ClientSelectableCameraVehicle {
    const ID: u16 = 15;
    type ClientMethod = client::entity::ClientSelectableCameraVehicleMethod;
    type BaseMethod = base::entity::ClientSelectableCameraVehicleMethod;
    type CellMethod = cell::entity::ClientSelectableCameraVehicleMethod;
}

/// Interface ClientSelectableWebLinksOpener
#[derive(Debug, Default)]
pub struct ClientSelectableWebLinksOpener {
}

impl Entity for ClientSelectableWebLinksOpener {
    const ID: u16 = 16;
    type ClientMethod = client::entity::ClientSelectableWebLinksOpenerMethod;
    type BaseMethod = base::entity::ClientSelectableWebLinksOpenerMethod;
    type CellMethod = cell::entity::ClientSelectableWebLinksOpenerMethod;
}

/// Interface ClientSelectableEasterEgg
#[derive(Debug, Default)]
pub struct ClientSelectableEasterEgg {
}

impl Entity for ClientSelectableEasterEgg {
    const ID: u16 = 17;
    type ClientMethod = client::entity::ClientSelectableEasterEggMethod;
    type BaseMethod = base::entity::ClientSelectableEasterEggMethod;
    type CellMethod = cell::entity::ClientSelectableEasterEggMethod;
}

/// Interface EmptyEntity
#[derive(Debug, Default)]
pub struct EmptyEntity {
}

impl Entity for EmptyEntity {
    const ID: u16 = 18;
    type ClientMethod = client::entity::EmptyEntityMethod;
    type BaseMethod = base::entity::EmptyEntityMethod;
    type CellMethod = cell::entity::EmptyEntityMethod;
}

/// Interface LimitedVisibilityEntity
#[derive(Debug, Default)]
pub struct LimitedVisibilityEntity {
}

impl Entity for LimitedVisibilityEntity {
    const ID: u16 = 19;
    type ClientMethod = client::entity::LimitedVisibilityEntityMethod;
    type BaseMethod = base::entity::LimitedVisibilityEntityMethod;
    type CellMethod = cell::entity::LimitedVisibilityEntityMethod;
}

/// Interface HeroTank
#[derive(Debug, Default)]
pub struct HeroTank {
}

impl Entity for HeroTank {
    const ID: u16 = 20;
    type ClientMethod = client::entity::HeroTankMethod;
    type BaseMethod = base::entity::HeroTankMethod;
    type CellMethod = cell::entity::HeroTankMethod;
}

/// Interface PlatoonTank
#[derive(Debug, Default)]
pub struct PlatoonTank {
}

impl Entity for PlatoonTank {
    const ID: u16 = 21;
    type ClientMethod = client::entity::PlatoonTankMethod;
    type BaseMethod = base::entity::PlatoonTankMethod;
    type CellMethod = cell::entity::PlatoonTankMethod;
}

/// Interface PlatoonLighting
#[derive(Debug, Default)]
pub struct PlatoonLighting {
}

impl Entity for PlatoonLighting {
    const ID: u16 = 22;
    type ClientMethod = client::entity::PlatoonLightingMethod;
    type BaseMethod = base::entity::PlatoonLightingMethod;
    type CellMethod = cell::entity::PlatoonLightingMethod;
}

/// Interface SectorBase
#[derive(Debug, Default)]
pub struct SectorBase {
    pub i_EntityTrap: EntityTrap,
}

impl Entity for SectorBase {
    const ID: u16 = 23;
    type ClientMethod = client::entity::SectorBaseMethod;
    type BaseMethod = base::entity::SectorBaseMethod;
    type CellMethod = cell::entity::SectorBaseMethod;
}

/// Interface Sector
#[derive(Debug, Default)]
pub struct Sector {
}

impl Entity for Sector {
    const ID: u16 = 24;
    type ClientMethod = client::entity::SectorMethod;
    type BaseMethod = base::entity::SectorMethod;
    type CellMethod = cell::entity::SectorMethod;
}

/// Interface DestructibleEntity
#[derive(Debug, Default)]
pub struct DestructibleEntity {
    pub i_Destructible: Destructible,
}

impl Entity for DestructibleEntity {
    const ID: u16 = 25;
    type ClientMethod = client::entity::DestructibleEntityMethod;
    type BaseMethod = base::entity::DestructibleEntityMethod;
    type CellMethod = cell::entity::DestructibleEntityMethod;
}

/// Interface StepRepairPoint
#[derive(Debug, Default)]
pub struct StepRepairPoint {
}

impl Entity for StepRepairPoint {
    const ID: u16 = 26;
    type ClientMethod = client::entity::StepRepairPointMethod;
    type BaseMethod = base::entity::StepRepairPointMethod;
    type CellMethod = cell::entity::StepRepairPointMethod;
}

/// Interface ProtectionZone
#[derive(Debug, Default)]
pub struct ProtectionZone {
}

impl Entity for ProtectionZone {
    const ID: u16 = 27;
    type ClientMethod = client::entity::ProtectionZoneMethod;
    type BaseMethod = base::entity::ProtectionZoneMethod;
    type CellMethod = cell::entity::ProtectionZoneMethod;
}

/// Interface HangarPoster
#[derive(Debug, Default)]
pub struct HangarPoster {
}

impl Entity for HangarPoster {
    const ID: u16 = 28;
    type ClientMethod = client::entity::HangarPosterMethod;
    type BaseMethod = base::entity::HangarPosterMethod;
    type CellMethod = cell::entity::HangarPosterMethod;
}

/// Interface TeamInfo
#[derive(Debug, Default)]
pub struct TeamInfo {
    pub i_ThrottledMethods: ThrottledMethods,
}

impl Entity for TeamInfo {
    const ID: u16 = 29;
    type ClientMethod = client::entity::TeamInfoMethod;
    type BaseMethod = base::entity::TeamInfoMethod;
    type CellMethod = cell::entity::TeamInfoMethod;
}

/// Interface AvatarInfo
#[derive(Debug, Default)]
pub struct AvatarInfo {
}

impl Entity for AvatarInfo {
    const ID: u16 = 30;
    type ClientMethod = client::entity::AvatarInfoMethod;
    type BaseMethod = base::entity::AvatarInfoMethod;
    type CellMethod = cell::entity::AvatarInfoMethod;
}

/// Interface ArenaObserverInfo
#[derive(Debug, Default)]
pub struct ArenaObserverInfo {
}

impl Entity for ArenaObserverInfo {
    const ID: u16 = 31;
    type ClientMethod = client::entity::ArenaObserverInfoMethod;
    type BaseMethod = base::entity::ArenaObserverInfoMethod;
    type CellMethod = cell::entity::ArenaObserverInfoMethod;
}

/// Interface AreaOfEffect
#[derive(Debug, Default)]
pub struct AreaOfEffect {
}

impl Entity for AreaOfEffect {
    const ID: u16 = 32;
    type ClientMethod = client::entity::AreaOfEffectMethod;
    type BaseMethod = base::entity::AreaOfEffectMethod;
    type CellMethod = cell::entity::AreaOfEffectMethod;
}

/// Interface AttackBomber
#[derive(Debug, Default)]
pub struct AttackBomber {
}

impl Entity for AttackBomber {
    const ID: u16 = 33;
    type ClientMethod = client::entity::AttackBomberMethod;
    type BaseMethod = base::entity::AttackBomberMethod;
    type CellMethod = cell::entity::AttackBomberMethod;
}

/// Interface AttackArtilleryFort
#[derive(Debug, Default)]
pub struct AttackArtilleryFort {
}

impl Entity for AttackArtilleryFort {
    const ID: u16 = 34;
    type ClientMethod = client::entity::AttackArtilleryFortMethod;
    type BaseMethod = base::entity::AttackArtilleryFortMethod;
    type CellMethod = cell::entity::AttackArtilleryFortMethod;
}

/// Interface PersonalDeathZone
#[derive(Debug, Default)]
pub struct PersonalDeathZone {
}

impl Entity for PersonalDeathZone {
    const ID: u16 = 35;
    type ClientMethod = client::entity::PersonalDeathZoneMethod;
    type BaseMethod = base::entity::PersonalDeathZoneMethod;
    type CellMethod = cell::entity::PersonalDeathZoneMethod;
}

/// Interface ClientSelectableRankedObject
#[derive(Debug, Default)]
pub struct ClientSelectableRankedObject {
}

impl Entity for ClientSelectableRankedObject {
    const ID: u16 = 36;
    type ClientMethod = client::entity::ClientSelectableRankedObjectMethod;
    type BaseMethod = base::entity::ClientSelectableRankedObjectMethod;
    type CellMethod = cell::entity::ClientSelectableRankedObjectMethod;
}

/// Interface SimulatedVehicle
#[derive(Debug, Default)]
pub struct SimulatedVehicle {
}

impl Entity for SimulatedVehicle {
    const ID: u16 = 37;
    type ClientMethod = client::entity::SimulatedVehicleMethod;
    type BaseMethod = base::entity::SimulatedVehicleMethod;
    type CellMethod = cell::entity::SimulatedVehicleMethod;
}

/// Interface ClientSelectableHangarsSwitcher
#[derive(Debug, Default)]
pub struct ClientSelectableHangarsSwitcher {
}

impl Entity for ClientSelectableHangarsSwitcher {
    const ID: u16 = 38;
    type ClientMethod = client::entity::ClientSelectableHangarsSwitcherMethod;
    type BaseMethod = base::entity::ClientSelectableHangarsSwitcherMethod;
    type CellMethod = cell::entity::ClientSelectableHangarsSwitcherMethod;
}

/// Interface StaticDeathZone
#[derive(Debug, Default)]
pub struct StaticDeathZone {
}

impl Entity for StaticDeathZone {
    const ID: u16 = 39;
    type ClientMethod = client::entity::StaticDeathZoneMethod;
    type BaseMethod = base::entity::StaticDeathZoneMethod;
    type CellMethod = cell::entity::StaticDeathZoneMethod;
}

/// Interface BasicMine
#[derive(Debug, Default)]
pub struct BasicMine {
}

impl Entity for BasicMine {
    const ID: u16 = 40;
    type ClientMethod = client::entity::BasicMineMethod;
    type BaseMethod = base::entity::BasicMineMethod;
    type CellMethod = cell::entity::BasicMineMethod;
}

/// Interface ApplicationPoint
#[derive(Debug, Default)]
pub struct ApplicationPoint {
}

impl Entity for ApplicationPoint {
    const ID: u16 = 41;
    type ClientMethod = client::entity::ApplicationPointMethod;
    type BaseMethod = base::entity::ApplicationPointMethod;
    type CellMethod = cell::entity::ApplicationPointMethod;
}

/// Interface NetworkEntity
#[derive(Debug, Default)]
pub struct NetworkEntity {
}

impl Entity for NetworkEntity {
    const ID: u16 = 42;
    type ClientMethod = client::entity::NetworkEntityMethod;
    type BaseMethod = base::entity::NetworkEntityMethod;
    type CellMethod = cell::entity::NetworkEntityMethod;
}

/// Interface Comp7Lighting
#[derive(Debug, Default)]
pub struct Comp7Lighting {
}

impl Entity for Comp7Lighting {
    const ID: u16 = 43;
    type ClientMethod = client::entity::Comp7LightingMethod;
    type BaseMethod = base::entity::Comp7LightingMethod;
    type CellMethod = cell::entity::Comp7LightingMethod;
}

/// Interface EventVehicle
#[derive(Debug, Default)]
pub struct EventVehicle {
}

impl Entity for EventVehicle {
    const ID: u16 = 44;
    type ClientMethod = client::entity::EventVehicleMethod;
    type BaseMethod = base::entity::EventVehicleMethod;
    type CellMethod = cell::entity::EventVehicleMethod;
}

/// Interface EventShowcaseVehicle
#[derive(Debug, Default)]
pub struct EventShowcaseVehicle {
}

impl Entity for EventShowcaseVehicle {
    const ID: u16 = 45;
    type ClientMethod = client::entity::EventShowcaseVehicleMethod;
    type BaseMethod = base::entity::EventShowcaseVehicleMethod;
    type CellMethod = cell::entity::EventShowcaseVehicleMethod;
}

/// Interface EventPortal
#[derive(Debug, Default)]
pub struct EventPortal {
}

impl Entity for EventPortal {
    const ID: u16 = 46;
    type ClientMethod = client::entity::EventPortalMethod;
    type BaseMethod = base::entity::EventPortalMethod;
    type CellMethod = cell::entity::EventPortalMethod;
}

