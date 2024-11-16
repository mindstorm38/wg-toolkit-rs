use super::alias::*;

// Interface Wheels
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct Wheels {
        pub steeringAngles: Vec<u8>,
        pub wheelsScroll: Vec<u8>,
        pub wheelsState: u64,
        pub burnoutLevel: u8,
    }
}

// Method for Wheels on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for Wheels on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for Wheels on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface VehiclesSpawnListStorage_Avatar
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct VehiclesSpawnListStorage_Avatar {
    }
}

// Method for VehiclesSpawnListStorage_Avatar on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct VehiclesSpawnListStorage_Avatar_updateSpawnList {
        pub a0: RelaxString,
    }

}

// Method for VehiclesSpawnListStorage_Avatar on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for VehiclesSpawnListStorage_Avatar on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface VehicleRemovalController_Avatar
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct VehicleRemovalController_Avatar {
    }
}

// Method for VehicleRemovalController_Avatar on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct VehicleRemovalController_Avatar_removeVehicle {
        pub a0: OBJECT_ID,
    }

}

// Method for VehicleRemovalController_Avatar on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for VehicleRemovalController_Avatar on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface VehicleObserver
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct VehicleObserver {
        pub remoteCamera: REMOTE_CAMERA_DATA,
    }
}

// Method for VehicleObserver on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for VehicleObserver on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for VehicleObserver on cell
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct VehicleObserver_setRemoteCamera {
        pub a0: REMOTE_CAMERA_DATA,
    }

}

// Interface VehicleHealthBroadcastListenerComponent_Avatar
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct VehicleHealthBroadcastListenerComponent_Avatar {
    }
}

// Method for VehicleHealthBroadcastListenerComponent_Avatar on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct VehicleHealthBroadcastListenerComponent_Avatar_onVehicleHealthChanged {
        pub a0: OBJECT_ID,
        pub a1: i16,
        pub a2: OBJECT_ID,
        pub a3: u8,
        pub a4: i8,
    }

}

// Method for VehicleHealthBroadcastListenerComponent_Avatar on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for VehicleHealthBroadcastListenerComponent_Avatar on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface VehicleAIProxy
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct VehicleAIProxy {
    }
}

// Method for VehicleAIProxy on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for VehicleAIProxy on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for VehicleAIProxy on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface TriggersController_Avatar
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct TriggersController_Avatar {
    }
}

// Method for TriggersController_Avatar on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct TriggersController_Avatar_externalTrigger {
        pub a0: RelaxString,
        pub a1: Python,
    }

}

// Method for TriggersController_Avatar on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for TriggersController_Avatar on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface TransactionUser
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct TransactionUser {
    }
}

// Method for TransactionUser on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for TransactionUser on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for TransactionUser on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface ThrottledMethods
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct ThrottledMethods {
    }
}

// Method for ThrottledMethods on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for ThrottledMethods on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for ThrottledMethods on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface TeamHealthBar_Avatar
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct TeamHealthBar_Avatar {
    }
}

// Method for TeamHealthBar_Avatar on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct TeamHealthBar_Avatar_updateTeamsHealthPercentage {
        pub a0: Vec<u8>,
    }

}

// Method for TeamHealthBar_Avatar on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for TeamHealthBar_Avatar on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface TeamBase_Vehicle
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct TeamBase_Vehicle {
    }
}

// Method for TeamBase_Vehicle on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for TeamBase_Vehicle on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for TeamBase_Vehicle on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface StepRepairPoint_Vehicle
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct StepRepairPoint_Vehicle {
    }
}

// Method for StepRepairPoint_Vehicle on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for StepRepairPoint_Vehicle on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for StepRepairPoint_Vehicle on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface SmokeController_Vehicle
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct SmokeController_Vehicle {
    }
}

// Method for SmokeController_Vehicle on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for SmokeController_Vehicle on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for SmokeController_Vehicle on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface SessionTracker
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct SessionTracker {
    }
}

// Method for SessionTracker on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for SessionTracker on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for SessionTracker on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface ServerSideReplays
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct ServerSideReplays {
    }
}

// Method for ServerSideReplays on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for ServerSideReplays on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for ServerSideReplays on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface Sector_Vehicle
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct Sector_Vehicle {
    }
}

// Method for Sector_Vehicle on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for Sector_Vehicle on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for Sector_Vehicle on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface SectorBase_Vehicle
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct SectorBase_Vehicle {
    }
}

// Method for SectorBase_Vehicle on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for SectorBase_Vehicle on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for SectorBase_Vehicle on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface RespawnController_Vehicle
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct RespawnController_Vehicle {
    }
}

// Method for RespawnController_Vehicle on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for RespawnController_Vehicle on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for RespawnController_Vehicle on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface RespawnController_Avatar
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct RespawnController_Avatar {
    }
}

// Method for RespawnController_Avatar on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct RespawnController_Avatar_redrawVehicleOnRespawn {
        pub a0: OBJECT_ID,
        pub a1: RelaxString,
        pub a2: RelaxString,
    }

    #[derive(Debug)]
    pub struct RespawnController_Avatar_explodeVehicleBeforeRespawn {
        pub a0: OBJECT_ID,
    }

    #[derive(Debug)]
    pub struct RespawnController_Avatar_updateRespawnVehicles {
        pub a0: Vec<RESPAWN_AVAILABLE_VEHICLE>,
    }

    #[derive(Debug)]
    pub struct RespawnController_Avatar_updateRespawnCooldowns {
        pub a0: Vec<RESPAWN_COOLDOWN_ITEM>,
    }

    #[derive(Debug)]
    pub struct RespawnController_Avatar_updateRespawnInfo {
        pub a0: RESPAWN_INFO,
    }

    #[derive(Debug)]
    pub struct RespawnController_Avatar_updateVehicleLimits {
        pub a0: Vec<RESPAWN_LIMITED_VEHICLES>,
    }

    #[derive(Debug)]
    pub struct RespawnController_Avatar_updatePlayerLives {
        pub a0: u8,
    }

    #[derive(Debug)]
    pub struct RespawnController_Avatar_onTeamLivesRestored {
        pub a0: Vec<u8>,
    }

}

// Method for RespawnController_Avatar on base
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct RespawnController_Avatar_respawnController_requestRespawnGroupChange {
        pub a0: u8,
    }

    #[derive(Debug)]
    pub struct RespawnController_Avatar_respawnController_chooseVehicleForRespawn {
        pub a0: u16,
    }

    #[derive(Debug)]
    pub struct RespawnController_Avatar_respawnController_performRespawn {
    }

    #[derive(Debug)]
    pub struct RespawnController_Avatar_respawnController_chooseRespawnZone {
        pub a0: Vec3,
    }

    #[derive(Debug)]
    pub struct RespawnController_Avatar_respawnController_switchSetup {
        pub a0: u16,
        pub a1: u8,
        pub a2: u8,
    }

}

// Method for RespawnController_Avatar on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface RepairBase_Vehicle
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct RepairBase_Vehicle {
    }
}

// Method for RepairBase_Vehicle on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for RepairBase_Vehicle on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for RepairBase_Vehicle on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface RecoveryMechanic_Vehicle
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct RecoveryMechanic_Vehicle {
    }
}

// Method for RecoveryMechanic_Vehicle on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for RecoveryMechanic_Vehicle on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for RecoveryMechanic_Vehicle on cell
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct RecoveryMechanic_Vehicle_recoveryMechanic_startRecovering {
    }

    #[derive(Debug)]
    pub struct RecoveryMechanic_Vehicle_recoveryMechanic_stopRecovering {
    }

}

// Interface RecoveryMechanic_Avatar
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct RecoveryMechanic_Avatar {
    }
}

// Method for RecoveryMechanic_Avatar on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct RecoveryMechanic_Avatar_notifyCannotStartRecovering {
    }

    #[derive(Debug)]
    pub struct RecoveryMechanic_Avatar_notifyCancelled {
    }

    #[derive(Debug)]
    pub struct RecoveryMechanic_Avatar_updateState {
        pub a0: BOOL,
        pub a1: i32,
        pub a2: i32,
        pub a3: f32,
    }

}

// Method for RecoveryMechanic_Avatar on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for RecoveryMechanic_Avatar on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface QuestProcessor
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct QuestProcessor {
    }
}

// Method for QuestProcessor on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for QuestProcessor on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for QuestProcessor on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface ProtectionZone_Vehicle
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct ProtectionZone_Vehicle {
    }
}

// Method for ProtectionZone_Vehicle on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for ProtectionZone_Vehicle on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for ProtectionZone_Vehicle on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface ProtectionZoneController_Avatar
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct ProtectionZoneController_Avatar {
    }
}

// Method for ProtectionZoneController_Avatar on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for ProtectionZoneController_Avatar on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for ProtectionZoneController_Avatar on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface PlayerMessenger_chat2
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct PlayerMessenger_chat2 {
    }
}

// Method for PlayerMessenger_chat2 on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct PlayerMessenger_chat2_messenger_onActionByServer_chat2 {
        pub a0: i16,
        pub a1: u16,
        pub a2: GENERIC_MESSENGER_ARGS_chat2,
    }

}

// Method for PlayerMessenger_chat2 on base
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct PlayerMessenger_chat2_messenger_onActionByClient_chat2 {
        pub a0: i16,
        pub a1: u16,
        pub a2: GENERIC_MESSENGER_ARGS_chat2,
    }

}

// Method for PlayerMessenger_chat2 on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface PlayLimits
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct PlayLimits {
    }
}

// Method for PlayLimits on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for PlayLimits on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for PlayLimits on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface PlaneTrajectoryArenaInfo
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct PlaneTrajectoryArenaInfo {
        pub planeTrajectory: PLANE_TRAJECTORY,
    }
}

// Method for PlaneTrajectoryArenaInfo on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for PlaneTrajectoryArenaInfo on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for PlaneTrajectoryArenaInfo on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface Perks_Vehicle
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct Perks_Vehicle {
        pub perkEffects: ANON79,
        pub perks: Vec<PERK_INFO_HUD>,
        pub perksRibbonNotify: Vec<PERK_INFO_RIBBON>,
    }
}

// Method for Perks_Vehicle on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for Perks_Vehicle on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for Perks_Vehicle on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface Invoicing
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct Invoicing {
    }
}

// Method for Invoicing on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for Invoicing on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for Invoicing on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface InvitationsClient
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct InvitationsClient {
    }
}

// Method for InvitationsClient on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct InvitationsClient_processInvitations {
        pub a0: Python,
    }

}

// Method for InvitationsClient on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for InvitationsClient on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface Invitations
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct Invitations {
    }
}

// Method for Invitations on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for Invitations on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for Invitations on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface InterclusterSender
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct InterclusterSender {
    }
}

// Method for InterclusterSender on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for InterclusterSender on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for InterclusterSender on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface Harm
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct Harm {
    }
}

// Method for Harm on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for Harm on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for Harm on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface EntityTrap
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct EntityTrap {
    }
}

// Method for EntityTrap on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for EntityTrap on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for EntityTrap on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface DestructibleEntity_Vehicle
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct DestructibleEntity_Vehicle {
    }
}

// Method for DestructibleEntity_Vehicle on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for DestructibleEntity_Vehicle on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for DestructibleEntity_Vehicle on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface DestructibleEntity_Avatar
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct DestructibleEntity_Avatar {
    }
}

// Method for DestructibleEntity_Avatar on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for DestructibleEntity_Avatar on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for DestructibleEntity_Avatar on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface Destructible
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct Destructible {
    }
}

// Method for Destructible on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for Destructible on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for Destructible on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface DefenderBonusController_Vehicle
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct DefenderBonusController_Vehicle {
    }
}

// Method for DefenderBonusController_Vehicle on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for DefenderBonusController_Vehicle on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for DefenderBonusController_Vehicle on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface ControlPoint
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct ControlPoint {
    }
}

// Method for ControlPoint on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for ControlPoint on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for ControlPoint on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface ClientCommandsPort
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct ClientCommandsPort {
    }
}

// Method for ClientCommandsPort on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct ClientCommandsPort_onCmdResponse {
        pub request_id: i16,
        pub result_id: i16,
        pub error: RelaxString,
    }

    #[derive(Debug)]
    pub struct ClientCommandsPort_onCmdResponseExt {
        pub request_id: i16,
        pub result_id: i16,
        pub error: RelaxString,
        pub ext: RelaxString,
    }

}

// Method for ClientCommandsPort on base
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct ClientCommandsPort_doCmdNoArgs {
        pub request_id: i16,
        pub cmd_id: i16,
    }

    #[derive(Debug)]
    pub struct ClientCommandsPort_doCmdStr {
        pub request_id: i16,
        pub cmd_id: i16,
        pub arg0: RelaxString,
    }

    #[derive(Debug)]
    pub struct ClientCommandsPort_doCmdInt {
        pub request_id: i16,
        pub cmd_id: i16,
        pub arg0: i64,
    }

    #[derive(Debug)]
    pub struct ClientCommandsPort_doCmdInt2 {
        pub request_id: i16,
        pub cmd_id: i16,
        pub arg0: i64,
        pub arg1: i64,
    }

    #[derive(Debug)]
    pub struct ClientCommandsPort_doCmdInt3 {
        pub request_id: i16,
        pub cmd_id: i16,
        pub arg0: i64,
        pub arg1: i64,
        pub arg2: i64,
    }

    #[derive(Debug)]
    pub struct ClientCommandsPort_doCmdInt4 {
        pub request_id: i16,
        pub cmd_id: i16,
        pub arg0: i64,
        pub arg1: i64,
        pub arg2: i32,
        pub arg3: i32,
    }

    #[derive(Debug)]
    pub struct ClientCommandsPort_doCmdInt2Str {
        pub request_id: i16,
        pub cmd_id: i16,
        pub arg0: i64,
        pub arg1: i64,
        pub arg2: RelaxString,
    }

    #[derive(Debug)]
    pub struct ClientCommandsPort_doCmdInt3Str {
        pub request_id: i16,
        pub cmd_id: i16,
        pub arg0: i64,
        pub arg1: i64,
        pub arg2: i64,
        pub arg3: RelaxString,
    }

    #[derive(Debug)]
    pub struct ClientCommandsPort_doCmdIntArr {
        pub request_id: i16,
        pub cmd_id: i16,
        pub arg0: Vec<i32>,
    }

    #[derive(Debug)]
    pub struct ClientCommandsPort_doCmdIntStr {
        pub request_id: i16,
        pub cmd_id: i16,
        pub arg0: i64,
        pub arg1: RelaxString,
    }

    #[derive(Debug)]
    pub struct ClientCommandsPort_doCmdIntStrArr {
        pub request_id: i16,
        pub cmd_id: i16,
        pub arg0: i64,
        pub arg1: Vec<RelaxString>,
    }

    #[derive(Debug)]
    pub struct ClientCommandsPort_doCmdIntArrStrArr {
        pub request_id: i16,
        pub cmd_id: i16,
        pub arg0: Vec<i64>,
        pub arg1: Vec<RelaxString>,
    }

    #[derive(Debug)]
    pub struct ClientCommandsPort_doCmdStrArr {
        pub request_id: i16,
        pub cmd_id: i16,
        pub arg0: Vec<RelaxString>,
    }

}

// Method for ClientCommandsPort on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface Chat
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct Chat {
    }
}

// Method for Chat on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct Chat_onChatAction {
        pub a0: CHAT_ACTION_DATA,
    }

}

// Method for Chat on base
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct Chat_chatCommandFromClient {
        pub request_id: i64,
        pub command_id: u8,
        pub channel_id: OBJECT_ID,
        pub i64_arg: i64,
        pub i16_arg: i16,
        pub str_arg0: RelaxString,
        pub str_arg1: RelaxString,
    }

    #[derive(Debug)]
    pub struct Chat_inviteCommand {
        pub request_id: i64,
        pub command_id: u8,
        pub invalid_type: i8,
        pub receiver_name: RelaxString,
        pub i64_arg: i64,
        pub i16_arg: i16,
        pub str_arg0: RelaxString,
        pub str_arg1: RelaxString,
    }

    #[derive(Debug)]
    pub struct Chat_ackCommand {
        pub request_id: i64,
        pub command_id: u8,
        pub time: f64,
        pub invite_id: i64,
        pub a4: i64,
    }

    #[derive(Debug)]
    pub struct Chat_onStreamComplete {
        pub a0: i16,
        pub a1: BOOL,
    }

}

// Method for Chat on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface BattleResultProcessor
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct BattleResultProcessor {
    }
}

// Method for BattleResultProcessor on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for BattleResultProcessor on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for BattleResultProcessor on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface BattleFeedback
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct BattleFeedback {
    }
}

// Method for BattleFeedback on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for BattleFeedback on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for BattleFeedback on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface AvatarObserver
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AvatarObserver {
        pub remoteCamera: REMOTE_CAMERA_DATA,
        pub isObserverFPV: BOOL,
        pub numOfObservers: u8,
    }
}

// Method for AvatarObserver on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for AvatarObserver on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for AvatarObserver on cell
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct AvatarObserver_switchObserverFPV {
        pub a0: BOOL,
    }

}

// Interface AvatarEpic
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AvatarEpic {
    }
}

// Method for AvatarEpic on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct AvatarEpic_welcomeToSector {
        pub a0: u8,
        pub a1: u8,
        pub a2: u8,
        pub a3: BOOL,
        pub a4: f32,
        pub a5: f32,
    }

    #[derive(Debug)]
    pub struct AvatarEpic_onStepRepairPointAction {
        pub a0: OBJECT_ID,
        pub a1: u8,
        pub a2: f32,
        pub a3: u16,
    }

    #[derive(Debug)]
    pub struct AvatarEpic_onSectorBaseAction {
        pub a0: u8,
        pub a1: u8,
        pub a2: f32,
    }

    #[derive(Debug)]
    pub struct AvatarEpic_enteringProtectionZone {
        pub a0: u8,
    }

    #[derive(Debug)]
    pub struct AvatarEpic_leavingProtectionZone {
        pub a0: u8,
    }

    #[derive(Debug)]
    pub struct AvatarEpic_protectionZoneShooting {
        pub a0: u8,
    }

    #[derive(Debug)]
    pub struct AvatarEpic_onSectorShooting {
        pub a0: u8,
    }

    #[derive(Debug)]
    pub struct AvatarEpic_onXPUpdated {
        pub a0: i16,
    }

    #[derive(Debug)]
    pub struct AvatarEpic_onCrewRoleFactorAndRankUpdate {
        pub a0: f32,
        pub a1: i64,
        pub a2: u8,
    }

    #[derive(Debug)]
    pub struct AvatarEpic_syncPurchasedAbilities {
        pub a0: Vec<i64>,
    }

    #[derive(Debug)]
    pub struct AvatarEpic_onRandomReserveOffer {
        pub a0: Vec<i32>,
        pub a1: Vec<u8>,
        pub a2: u8,
    }

    #[derive(Debug)]
    pub struct AvatarEpic_onRankUpdate {
        pub a0: u8,
    }

    #[derive(Debug)]
    pub struct AvatarEpic_showDestructibleShotResults {
        pub a0: u8,
        pub a1: Vec<u32>,
    }

    #[derive(Debug)]
    pub struct AvatarEpic_onDestructibleDestroyed {
        pub a0: u8,
        pub a1: OBJECT_ID,
    }

}

// Method for AvatarEpic on base
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct AvatarEpic_enableFrontLineDevInfo {
        pub a0: BOOL,
    }

}

// Method for AvatarEpic on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface AvatarCreator
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AvatarCreator {
    }
}

// Method for AvatarCreator on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for AvatarCreator on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for AvatarCreator on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface AccountVersion
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AccountVersion {
        pub requiredVersion_12610: RelaxString,
    }
}

// Method for AccountVersion on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountVersion on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountVersion on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface AccountUnitRemote
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AccountUnitRemote {
    }
}

// Method for AccountUnitRemote on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountUnitRemote on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountUnitRemote on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface AccountUnitClient
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AccountUnitClient {
    }
}

// Method for AccountUnitClient on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountUnitClient on base
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct AccountUnitClient_accountUnitClient_create {
        pub a0: i32,
        pub a1: i32,
        pub a2: i32,
        pub a3: RelaxString,
        pub a4: RelaxString,
    }

    #[derive(Debug)]
    pub struct AccountUnitClient_accountUnitClient_join {
        pub a0: i32,
        pub a1: u64,
        pub a2: i32,
    }

    #[derive(Debug)]
    pub struct AccountUnitClient_accountUnitClient_doCmd {
        pub a0: i32,
        pub a1: OBJECT_ID,
        pub a2: i32,
        pub a3: u64,
        pub a4: i32,
        pub a5: RelaxString,
    }

    #[derive(Debug)]
    pub struct AccountUnitClient_accountUnitClient_sendInvites {
        pub a0: i32,
        pub a1: u64,
        pub a2: Vec<DB_ID>,
        pub a3: RelaxString,
    }

    #[derive(Debug)]
    pub struct AccountUnitClient_accountUnitClient_setRosterSlots {
        pub a0: i32,
        pub a1: u64,
        pub a2: Vec<i32>,
        pub a3: Vec<RelaxString>,
    }

}

// Method for AccountUnitClient on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface AccountUnitBrowser
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AccountUnitBrowser {
    }
}

// Method for AccountUnitBrowser on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountUnitBrowser on base
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct AccountUnitBrowser_accountUnitBrowser_subscribe {
        pub unit_type_flags: i16,
        pub show_other_locations: BOOL,
    }

    #[derive(Debug)]
    pub struct AccountUnitBrowser_accountUnitBrowser_unsubscribe {
    }

    #[derive(Debug)]
    pub struct AccountUnitBrowser_accountUnitBrowser_recenter {
        pub a0: i32,
        pub a1: i16,
        pub a2: BOOL,
    }

    #[derive(Debug)]
    pub struct AccountUnitBrowser_accountUnitBrowser_doCmd {
        pub a0: i32,
    }

}

// Method for AccountUnitBrowser on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface AccountUnitAssembler
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AccountUnitAssembler {
    }
}

// Method for AccountUnitAssembler on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountUnitAssembler on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountUnitAssembler on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface AccountUnit
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AccountUnit {
    }
}

// Method for AccountUnit on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountUnit on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountUnit on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface AccountSysMessenger
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AccountSysMessenger {
    }
}

// Method for AccountSysMessenger on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountSysMessenger on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountSysMessenger on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface AccountSpaProcessor
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AccountSpaProcessor {
    }
}

// Method for AccountSpaProcessor on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountSpaProcessor on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountSpaProcessor on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface AccountPrebattle
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AccountPrebattle {
    }
}

// Method for AccountPrebattle on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountPrebattle on base
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct AccountPrebattle_accountPrebattle_createTraining {
        pub a0: i32,
        pub a1: i32,
        pub a2: BOOL,
        pub a3: RelaxString,
    }

    #[derive(Debug)]
    pub struct AccountPrebattle_accountPrebattle_createDevPrebattle {
        pub a0: u8,
        pub a1: u8,
        pub a2: i32,
        pub a3: i32,
        pub a4: RelaxString,
    }

    #[derive(Debug)]
    pub struct AccountPrebattle_accountPrebattle_sendPrebattleInvites {
        pub a0: Vec<i64>,
        pub a1: RelaxString,
    }

}

// Method for AccountPrebattle on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface AccountIGRProcessing
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AccountIGRProcessing {
    }
}

// Method for AccountIGRProcessing on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountIGRProcessing on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountIGRProcessing on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface AccountGlobalMapConnector
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AccountGlobalMapConnector {
    }
}

// Method for AccountGlobalMapConnector on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountGlobalMapConnector on base
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct AccountGlobalMapConnector_accountGlobalMapConnector_callGlobalMapMethod {
        pub a0: u64,
        pub a1: i32,
        pub a2: i64,
        pub a3: RelaxString,
    }

}

// Method for AccountGlobalMapConnector on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface AccountEditor
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AccountEditor {
    }
}

// Method for AccountEditor on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountEditor on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountEditor on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface AccountDebugger
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AccountDebugger {
    }
}

// Method for AccountDebugger on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountDebugger on base
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct AccountDebugger_accountDebugger_registerDebugTaskResult {
        pub a0: i64,
        pub a1: i32,
        pub a2: i64,
    }

    #[derive(Debug)]
    pub struct AccountDebugger_accountDebugger_sendDebugTaskResultChunk {
        pub a0: i64,
        pub a1: i64,
        pub a2: RelaxString,
    }

}

// Method for AccountDebugger on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface AccountClan
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AccountClan {
    }
}

// Method for AccountClan on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountClan on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountClan on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface AccountAvatar
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AccountAvatar {
    }
}

// Method for AccountAvatar on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountAvatar on base
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct AccountAvatar_accountAvatar_sendAccountStats {
        pub a0: u32,
        pub a1: Vec<RelaxString>,
    }

}

// Method for AccountAvatar on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface AccountAuthTokenProviderClient
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AccountAuthTokenProviderClient {
    }
}

// Method for AccountAuthTokenProviderClient on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct AccountAuthTokenProviderClient_onTokenReceived {
        pub request_id: u16,
        pub token_type: u8,
        pub data: Python,
    }

}

// Method for AccountAuthTokenProviderClient on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountAuthTokenProviderClient on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface AccountAuthTokenProvider
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AccountAuthTokenProvider {
    }
}

// Method for AccountAuthTokenProvider on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountAuthTokenProvider on base
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct AccountAuthTokenProvider_requestToken {
        pub a0: u16,
        pub a1: u8,
    }

}

// Method for AccountAuthTokenProvider on cell
wgtk::__bootstrap_struct_data_type! {

}

// Interface AccountAdmin
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AccountAdmin {
    }
}

// Method for AccountAdmin on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountAdmin on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for AccountAdmin on cell
wgtk::__bootstrap_struct_data_type! {

}

