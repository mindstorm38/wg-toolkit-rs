use super::alias::*;
use super::{base, cell, client};

/// Interface Wheels
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct Wheels {
        pub steeringAngles: Vec<u8>,
        pub wheelsScroll: Vec<u8>,
        pub wheelsState: u64,
        pub burnoutLevel: u8,
    }
}

/// Interface VehiclesSpawnListStorage_Avatar
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct VehiclesSpawnListStorage_Avatar {
    }
}

/// Interface VehicleRemovalController_Avatar
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct VehicleRemovalController_Avatar {
    }
}

/// Interface VehicleObserver
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct VehicleObserver {
        pub remoteCamera: REMOTE_CAMERA_DATA,
    }
}

/// Interface VehicleHealthBroadcastListenerComponent_Avatar
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct VehicleHealthBroadcastListenerComponent_Avatar {
    }
}

/// Interface VehicleAIProxy
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct VehicleAIProxy {
    }
}

/// Interface TriggersController_Avatar
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct TriggersController_Avatar {
    }
}

/// Interface TransactionUser
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct TransactionUser {
    }
}

/// Interface ThrottledMethods
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct ThrottledMethods {
    }
}

/// Interface TeamHealthBar_Avatar
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct TeamHealthBar_Avatar {
    }
}

/// Interface TeamBase_Vehicle
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct TeamBase_Vehicle {
    }
}

/// Interface StepRepairPoint_Vehicle
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct StepRepairPoint_Vehicle {
    }
}

/// Interface SmokeController_Vehicle
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct SmokeController_Vehicle {
    }
}

/// Interface SessionTracker
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct SessionTracker {
    }
}

/// Interface ServerSideReplays
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct ServerSideReplays {
    }
}

/// Interface Sector_Vehicle
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct Sector_Vehicle {
    }
}

/// Interface SectorBase_Vehicle
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct SectorBase_Vehicle {
    }
}

/// Interface RespawnController_Vehicle
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct RespawnController_Vehicle {
    }
}

/// Interface RespawnController_Avatar
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct RespawnController_Avatar {
    }
}

/// Interface RepairBase_Vehicle
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct RepairBase_Vehicle {
    }
}

/// Interface RecoveryMechanic_Vehicle
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct RecoveryMechanic_Vehicle {
    }
}

/// Interface RecoveryMechanic_Avatar
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct RecoveryMechanic_Avatar {
    }
}

/// Interface QuestProcessor
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct QuestProcessor {
    }
}

/// Interface ProtectionZone_Vehicle
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct ProtectionZone_Vehicle {
    }
}

/// Interface ProtectionZoneController_Avatar
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct ProtectionZoneController_Avatar {
    }
}

/// Interface PlayerMessenger_chat2
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct PlayerMessenger_chat2 {
    }
}

/// Interface PlayLimits
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct PlayLimits {
    }
}

/// Interface PlaneTrajectoryArenaInfo
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct PlaneTrajectoryArenaInfo {
        pub planeTrajectory: PLANE_TRAJECTORY,
    }
}

/// Interface Perks_Vehicle
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct Perks_Vehicle {
        pub perkEffects: ANON79,
        pub perks: Vec<PERK_INFO_HUD>,
        pub perksRibbonNotify: Vec<PERK_INFO_RIBBON>,
    }
}

/// Interface Invoicing
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct Invoicing {
    }
}

/// Interface InvitationsClient
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct InvitationsClient {
    }
}

/// Interface Invitations
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct Invitations {
    }
}

/// Interface InterclusterSender
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct InterclusterSender {
    }
}

/// Interface Harm
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct Harm {
    }
}

/// Interface EntityTrap
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct EntityTrap {
    }
}

/// Interface DestructibleEntity_Vehicle
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct DestructibleEntity_Vehicle {
    }
}

/// Interface DestructibleEntity_Avatar
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct DestructibleEntity_Avatar {
    }
}

/// Interface Destructible
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct Destructible {
    }
}

/// Interface DefenderBonusController_Vehicle
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct DefenderBonusController_Vehicle {
    }
}

/// Interface ControlPoint
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct ControlPoint {
    }
}

/// Interface ClientCommandsPort
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct ClientCommandsPort {
    }
}

/// Interface Chat
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct Chat {
    }
}

/// Interface BattleResultProcessor
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct BattleResultProcessor {
    }
}

/// Interface BattleFeedback
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct BattleFeedback {
    }
}

/// Interface AvatarObserver
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct AvatarObserver {
        pub remoteCamera: REMOTE_CAMERA_DATA,
        pub isObserverFPV: BOOL,
        pub numOfObservers: u8,
    }
}

/// Interface AvatarEpic
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct AvatarEpic {
    }
}

/// Interface AvatarCreator
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct AvatarCreator {
    }
}

/// Interface AccountVersion
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct AccountVersion {
        pub requiredVersion_12610: String,
    }
}

/// Interface AccountUnitRemote
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct AccountUnitRemote {
    }
}

/// Interface AccountUnitClient
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct AccountUnitClient {
    }
}

/// Interface AccountUnitBrowser
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct AccountUnitBrowser {
    }
}

/// Interface AccountUnitAssembler
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct AccountUnitAssembler {
    }
}

/// Interface AccountUnit
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct AccountUnit {
    }
}

/// Interface AccountSysMessenger
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct AccountSysMessenger {
    }
}

/// Interface AccountSpaProcessor
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct AccountSpaProcessor {
    }
}

/// Interface AccountPrebattle
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct AccountPrebattle {
    }
}

/// Interface AccountIGRProcessing
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct AccountIGRProcessing {
    }
}

/// Interface AccountGlobalMapConnector
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct AccountGlobalMapConnector {
    }
}

/// Interface AccountEditor
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct AccountEditor {
    }
}

/// Interface AccountDebugger
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct AccountDebugger {
    }
}

/// Interface AccountClan
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct AccountClan {
    }
}

/// Interface AccountAvatar
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct AccountAvatar {
    }
}

/// Interface AccountAuthTokenProviderClient
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct AccountAuthTokenProviderClient {
    }
}

/// Interface AccountAuthTokenProvider
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct AccountAuthTokenProvider {
    }
}

/// Interface AccountAdmin
wgtk::struct_data_type! {
    #[derive(Debug)]
    pub struct AccountAdmin {
    }
}

