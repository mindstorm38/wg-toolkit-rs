use wgtk::net::app::common::entity::{Entity, DataTypeEntity};

use super::alias::*;
use super::interface::*;

// Entity 0x01
// Interface Account
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct Account {
        pub i_AccountVersion: AccountVersion,
        pub name: RelaxString,
        pub incarnationID: u64,
        pub initialServerSettings: Python,
    }
}

// Method for Account on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct Account_onKickedFromServer {
        pub a0: RelaxString,
        pub a1: u8,
        pub a2: u32,
    }

    #[derive(Debug)]
    pub struct Account_onEnqueued {
        pub a0: u8,
    }

    #[derive(Debug)]
    pub struct Account_onEnqueueFailure {
        pub a0: u8,
        pub a1: u8,
        pub a2: RelaxString,
    }

    #[derive(Debug)]
    pub struct Account_onDequeued {
        pub a0: u8,
    }

    #[derive(Debug)]
    pub struct Account_onKickedFromQueue {
        pub a0: u8,
    }

    #[derive(Debug)]
    pub struct Account_onArenaCreated {
    }

    #[derive(Debug)]
    pub struct Account_onIGRTypeChanged {
        pub a0: RelaxString,
    }

    #[derive(Debug)]
    pub struct Account_onArenaJoinFailure {
        pub a0: u8,
        pub a1: RelaxString,
    }

    #[derive(Debug)]
    pub struct Account_onPrebattleJoined {
        pub a0: OBJECT_ID,
    }

    #[derive(Debug)]
    pub struct Account_onPrebattleJoinFailure {
        pub a0: u8,
    }

    #[derive(Debug)]
    pub struct Account_onPrebattleLeft {
    }

    #[derive(Debug)]
    pub struct Account_onKickedFromArena {
        pub a0: u8,
    }

    #[derive(Debug)]
    pub struct Account_onKickedFromPrebattle {
        pub a0: u8,
    }

    #[derive(Debug)]
    pub struct Account_onCenterIsLongDisconnected {
        pub a0: BOOL,
    }

    #[derive(Debug)]
    pub struct Account_showGUI {
        pub a0: RelaxString,
    }

    #[derive(Debug)]
    pub struct Account_receiveActiveArenas {
        pub a0: Vec<PUBLIC_ARENA_INFO>,
    }

    #[derive(Debug)]
    pub struct Account_receiveServerStats {
        pub a0: SERVER_STATISTICS,
    }

    #[derive(Debug)]
    pub struct Account_receiveQueueInfo {
        pub a0: QUEUE_INFO,
    }

    #[derive(Debug)]
    pub struct Account_updatePrebattle {
        pub a0: u8,
        pub a1: RelaxString,
    }

    #[derive(Debug)]
    pub struct Account_update {
        pub a0: RelaxString,
    }

    #[derive(Debug)]
    pub struct Account_resyncDossiers {
        pub a0: BOOL,
    }

    #[derive(Debug)]
    pub struct Account_reloadShop {
    }

    #[derive(Debug)]
    pub struct Account_onUnitUpdate {
        pub a0: u64,
        pub a1: RelaxString,
        pub a2: RelaxString,
    }

    #[derive(Debug)]
    pub struct Account_onUnitCallOk {
        pub a0: i32,
    }

    #[derive(Debug)]
    pub struct Account_onUnitNotify {
        pub a0: u64,
        pub a1: i32,
        pub a2: RelaxString,
        pub a3: Python,
    }

    #[derive(Debug)]
    pub struct Account_onUnitError {
        pub a0: i32,
        pub a1: u64,
        pub a2: i32,
        pub a3: RelaxString,
    }

    #[derive(Debug)]
    pub struct Account_onUnitBrowserError {
        pub a0: i32,
        pub a1: RelaxString,
    }

    #[derive(Debug)]
    pub struct Account_onUnitBrowserResultsSet {
        pub a0: RelaxString,
    }

    #[derive(Debug)]
    pub struct Account_onUnitBrowserResultsUpdate {
        pub a0: RelaxString,
    }

    #[derive(Debug)]
    pub struct Account_onGlobalMapUpdate {
        pub a0: RelaxString,
        pub a1: RelaxString,
    }

    #[derive(Debug)]
    pub struct Account_onGlobalMapReply {
        pub a0: u64,
        pub a1: i32,
        pub a2: RelaxString,
    }

    #[derive(Debug)]
    pub struct Account_onSendPrebattleInvites {
        pub a0: DB_ID,
        pub a1: RelaxString,
        pub a2: DB_ID,
        pub a3: RelaxString,
        pub a4: u64,
        pub a5: u8,
    }

    #[derive(Debug)]
    pub struct Account_onClanInfoReceived {
        pub a0: DB_ID,
        pub a1: RelaxString,
        pub a2: RelaxString,
        pub a3: RelaxString,
        pub a4: RelaxString,
    }

    #[derive(Debug)]
    pub struct Account_receiveNotification {
        pub a0: RelaxString,
    }

}

// Method for Account on base
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct Account_makeDenunciation {
        pub a0: DB_ID,
        pub a1: i32,
        pub a2: i8,
    }

    #[derive(Debug)]
    pub struct Account_banUnbanUser {
        pub a0: DB_ID,
        pub a1: u8,
        pub a2: u32,
        pub a3: RelaxString,
        pub a4: i8,
    }

    #[derive(Debug)]
    pub struct Account_requestToken {
        pub a0: u16,
        pub a1: u8,
    }

    #[derive(Debug)]
    pub struct Account_logStreamCorruption {
        pub a0: i16,
        pub a1: i32,
        pub a2: i32,
        pub a3: i32,
        pub a4: i32,
    }

    #[derive(Debug)]
    pub struct Account_setKickAtTime {
        pub a0: i64,
        pub a1: RelaxString,
        pub a2: RelaxString,
    }

}

// Method for Account on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for Account on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum Account_Client {
        Account_onArenaCreated(0x00, 0),
        Account_onPrebattleLeft(0x01, 0),
        Account_reloadShop(0x02, 0),
        Account_onEnqueued(0x03, 1),
        Account_onDequeued(0x04, 1),
        Account_onKickedFromQueue(0x05, 1),
        Account_onPrebattleJoinFailure(0x06, 1),
        Account_onKickedFromArena(0x07, 1),
        Account_onKickedFromPrebattle(0x08, 1),
        Account_onCenterIsLongDisconnected(0x09, 1),
        Account_resyncDossiers(0x0A, 1),
        Account_onPrebattleJoined(0x0B, 4),
        Account_onUnitCallOk(0x0C, 4),
        Account_receiveServerStats(0x0D, 8),
        Chat_onChatAction(0x0E, var8),
        PlayerMessenger_chat2_messenger_onActionByServer_chat2(0x0F, var8),
        ClientCommandsPort_onCmdResponse(0x10, var8),
        ClientCommandsPort_onCmdResponseExt(0x11, var8),
        AccountAuthTokenProviderClient_onTokenReceived(0x12, var8),
        InvitationsClient_processInvitations(0x13, var8),
        Account_onKickedFromServer(0x14, var8),
        Account_onEnqueueFailure(0x15, var8),
        Account_onIGRTypeChanged(0x16, var8),
        Account_onArenaJoinFailure(0x17, var8),
        Account_receiveActiveArenas(0x18, var8),
        Account_receiveQueueInfo(0x19, var8),
        Account_updatePrebattle(0x1A, var8),
        Account_update(0x1B, var8),
        Account_onUnitUpdate(0x1C, var8),
        Account_onUnitNotify(0x1D, var8),
        Account_onUnitError(0x1E, var8),
        Account_onUnitBrowserError(0x1F, var8),
        Account_onUnitBrowserResultsSet(0x20, var8),
        Account_onUnitBrowserResultsUpdate(0x21, var8),
        Account_onGlobalMapUpdate(0x22, var8),
        Account_onGlobalMapReply(0x23, var8),
        Account_onSendPrebattleInvites(0x24, var8),
        Account_onClanInfoReceived(0x25, var8),
        Account_receiveNotification(0x26, var8),
        Account_showGUI(0x27, var16),
    }
}

// Entity methods for Account on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum Account_Base {
        AccountUnitBrowser_accountUnitBrowser_unsubscribe(0x00, 0),
        Chat_onStreamComplete(0x01, 3),
        AccountAuthTokenProvider_requestToken(0x02, 3),
        AccountUnitBrowser_accountUnitBrowser_subscribe(0x03, 3),
        Account_requestToken(0x04, 3),
        ClientCommandsPort_doCmdNoArgs(0x05, 4),
        AccountUnitBrowser_accountUnitBrowser_doCmd(0x06, 4),
        AccountUnitBrowser_accountUnitBrowser_recenter(0x07, 7),
        ClientCommandsPort_doCmdInt(0x08, 12),
        Account_makeDenunciation(0x09, 13),
        AccountUnitClient_accountUnitClient_join(0x0A, 16),
        Account_logStreamCorruption(0x0B, 18),
        ClientCommandsPort_doCmdInt2(0x0C, 20),
        AccountDebugger_accountDebugger_registerDebugTaskResult(0x0D, 20),
        ClientCommandsPort_doCmdInt3(0x0E, 28),
        ClientCommandsPort_doCmdInt4(0x0F, 28),
        Chat_ackCommand(0x10, 33),
        Chat_chatCommandFromClient(0x11, var8),
        Chat_inviteCommand(0x12, var8),
        PlayerMessenger_chat2_messenger_onActionByClient_chat2(0x13, var8),
        ClientCommandsPort_doCmdStr(0x14, var8),
        ClientCommandsPort_doCmdInt2Str(0x15, var8),
        ClientCommandsPort_doCmdInt3Str(0x16, var8),
        ClientCommandsPort_doCmdIntArr(0x17, var8),
        ClientCommandsPort_doCmdIntStr(0x18, var8),
        ClientCommandsPort_doCmdIntStrArr(0x19, var8),
        ClientCommandsPort_doCmdIntArrStrArr(0x1A, var8),
        ClientCommandsPort_doCmdStrArr(0x1B, var8),
        AccountAvatar_accountAvatar_sendAccountStats(0x1C, var8),
        AccountPrebattle_accountPrebattle_createTraining(0x1D, var8),
        AccountPrebattle_accountPrebattle_createDevPrebattle(0x1E, var8),
        AccountPrebattle_accountPrebattle_sendPrebattleInvites(0x1F, var8),
        AccountGlobalMapConnector_accountGlobalMapConnector_callGlobalMapMethod(0x20, var8),
        AccountUnitClient_accountUnitClient_create(0x21, var8),
        AccountUnitClient_accountUnitClient_doCmd(0x22, var8),
        AccountUnitClient_accountUnitClient_sendInvites(0x23, var8),
        AccountUnitClient_accountUnitClient_setRosterSlots(0x24, var8),
        AccountDebugger_accountDebugger_sendDebugTaskResultChunk(0x25, var8),
        Account_banUnbanUser(0x26, var8),
        Account_setKickAtTime(0x27, var8),
    }
}

// Entity methods for Account on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum Account_Cell {
    }
}

impl Account {
    const TYPE_ID: u16 = 0x01;
}

impl DataTypeEntity for Account {
    type ClientMethod = Account_Client;
    type BaseMethod = Account_Base;
    type CellMethod = Account_Cell;
}

// Entity 0x02
// Interface Avatar
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct Avatar {
        pub i_AvatarObserver: AvatarObserver,
        pub name: RelaxString,
        pub sessionID: RelaxString,
        pub arenaUniqueID: u64,
        pub arenaTypeID: i32,
        pub arenaBonusType: u8,
        pub arenaGuiType: u8,
        pub arenaExtraData: Python,
        pub weatherPresetID: u8,
        pub denunciationsLeft: i16,
        pub clientCtx: RelaxString,
        pub tkillIsSuspected: BOOL,
        pub team: u8,
        pub playerVehicleID: OBJECT_ID,
        pub isObserverBothTeams: BOOL,
        pub observableTeamID: u8,
        pub isGunLocked: BOOL,
        pub ownVehicleGear: u8,
        pub ownVehicleAuxPhysicsData: u64,
        pub ownVehicleHullAimingPitchPacked: u16,
        pub ammoViews: AVATAR_AMMO_VIEWS,
        pub customizationDisplayType: u8,
        pub playLimits: PLAY_LIMITS,
        pub battleChatRestriction: BATTLE_CHAT_RESTRICTION,
        pub goodiesSnapshot: Vec<BATTLE_GOODIE_RECORD>,
        pub shouldSendKillcamSimulationData: BOOL,
    }
}

// Method for Avatar on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct Avatar_update {
        pub a0: RelaxString,
    }

    #[derive(Debug)]
    pub struct Avatar_onKickedFromServer {
        pub a0: RelaxString,
        pub a1: u8,
        pub a2: u32,
    }

    #[derive(Debug)]
    pub struct Avatar_onIGRTypeChanged {
        pub a0: RelaxString,
    }

    #[derive(Debug)]
    pub struct Avatar_onAutoAimVehicleLost {
        pub a0: u8,
    }

    #[derive(Debug)]
    pub struct Avatar_receiveAccountStats {
        pub a0: u32,
        pub a1: RelaxString,
    }

    #[derive(Debug)]
    pub struct Avatar_updateVehicleHealth {
        pub a0: OBJECT_ID,
        pub a1: i16,
        pub a2: i8,
        pub a3: BOOL,
        pub a4: BOOL,
    }

    #[derive(Debug)]
    pub struct Avatar_updateVehicleAmmo {
        pub a0: OBJECT_ID,
        pub a1: i32,
        pub a2: u16,
        pub a3: u8,
        pub a4: u8,
        pub a5: i16,
        pub a6: i16,
        pub a7: i16,
    }

    #[derive(Debug)]
    pub struct Avatar_onSwitchViewpoint {
        pub a0: OBJECT_ID,
        pub a1: Vec3,
    }

    #[derive(Debug)]
    pub struct Avatar_updateVehicleSetting {
        pub a0: OBJECT_ID,
        pub a1: u8,
        pub a2: i32,
    }

    #[derive(Debug)]
    pub struct Avatar_updateTargetingInfo {
        pub a0: f32,
        pub a1: f32,
        pub a2: f32,
        pub a3: f32,
        pub a4: f32,
        pub a5: f32,
        pub a6: f32,
        pub a7: f32,
        pub a8: f32,
    }

    #[derive(Debug)]
    pub struct Avatar_updateTargetVehicleID {
        pub a0: OBJECT_ID,
    }

    #[derive(Debug)]
    pub struct Avatar_showOwnVehicleHitDirection {
        pub a0: f32,
        pub a1: OBJECT_ID,
        pub a2: u16,
        pub a3: u32,
        pub a4: BOOL,
        pub a5: BOOL,
        pub a6: OBJECT_ID,
        pub a7: u8,
    }

    #[derive(Debug)]
    pub struct Avatar_showOtherVehicleDamagedDevices {
        pub a0: OBJECT_ID,
        pub a1: Vec<EXTRA_ID>,
        pub a2: Vec<EXTRA_ID>,
    }

    #[derive(Debug)]
    pub struct Avatar_showShotResults {
        pub a0: Vec<u64>,
    }

    #[derive(Debug)]
    pub struct Avatar_showDevelopmentInfo {
        pub a0: u8,
        pub a1: RelaxString,
    }

    #[derive(Debug)]
    pub struct Avatar_showHittingArea {
        pub a0: u16,
        pub a1: Vec3,
        pub a2: Vec3,
        pub a3: f64,
    }

    #[derive(Debug)]
    pub struct Avatar_showCarpetBombing {
        pub a0: u16,
        pub a1: Vec3,
        pub a2: Vec3,
        pub a3: f64,
    }

    #[derive(Debug)]
    pub struct Avatar_showTracer {
        pub a0: OBJECT_ID,
        pub a1: SHOT_ID,
        pub a2: BOOL,
        pub a3: u8,
        pub a4: Vec3,
        pub a5: Vec3,
        pub a6: f32,
        pub a7: f32,
        pub a8: u8,
    }

    #[derive(Debug)]
    pub struct Avatar_stopTracer {
        pub a0: SHOT_ID,
        pub a1: Vec3,
    }

    #[derive(Debug)]
    pub struct Avatar_explodeProjectile {
        pub a0: SHOT_ID,
        pub a1: u8,
        pub a2: u8,
        pub a3: Vec3,
        pub a4: Vec3,
        pub a5: Vec<u32>,
    }

    #[derive(Debug)]
    pub struct Avatar_onRoundFinished {
        pub a0: i8,
        pub a1: u8,
    }

    #[derive(Debug)]
    pub struct Avatar_onKickedFromArena {
        pub a0: u8,
    }

    #[derive(Debug)]
    pub struct Avatar_onBattleEvents {
        pub a0: Vec<BATTLE_EVENT>,
    }

    #[derive(Debug)]
    pub struct Avatar_battleEventsSummary {
        pub a0: BATTLE_EVENTS_SUMMARY,
    }

    #[derive(Debug)]
    pub struct Avatar_updateArena {
        pub a0: u8,
        pub a1: RelaxString,
    }

    #[derive(Debug)]
    pub struct Avatar_updatePositions {
        pub a0: Vec<u16>,
        pub a1: Vec<i16>,
    }

    #[derive(Debug)]
    pub struct Avatar_receivePhysicsDebugInfo {
        pub a0: RelaxString,
    }

    #[derive(Debug)]
    pub struct Avatar_updateCarriedFlagPositions {
        pub a0: Vec<u8>,
        pub a1: Vec<i16>,
    }

    #[derive(Debug)]
    pub struct Avatar_receiveNotification {
        pub a0: RelaxString,
    }

    #[derive(Debug)]
    pub struct Avatar_onRepairPointAction {
        pub a0: u8,
        pub a1: u8,
        pub a2: f32,
    }

    #[derive(Debug)]
    pub struct Avatar_updateAvatarPrivateStats {
        pub a0: RelaxString,
    }

    #[derive(Debug)]
    pub struct Avatar_updateResourceAmount {
        pub a0: u8,
        pub a1: u32,
    }

    #[derive(Debug)]
    pub struct Avatar_onFrictionWithVehicle {
        pub a0: OBJECT_ID,
        pub a1: Vec3,
        pub a2: u8,
    }

    #[derive(Debug)]
    pub struct Avatar_onCollisionWithVehicle {
        pub a0: Vec3,
        pub a1: f32,
    }

    #[derive(Debug)]
    pub struct Avatar_onSmoke {
        pub a0: SMOKE_INFO,
    }

    #[derive(Debug)]
    pub struct Avatar_onCombatEquipmentShotLaunched {
        pub a0: u16,
        pub a1: Vec3,
    }

    #[derive(Debug)]
    pub struct Avatar_updateQuestProgress {
        pub a0: RelaxString,
        pub a1: Python,
    }

    #[derive(Debug)]
    pub struct Avatar_updateVehicleQuickShellChanger {
        pub a0: OBJECT_ID,
        pub a1: BOOL,
    }

    #[derive(Debug)]
    pub struct Avatar_enemySPGHit {
        pub a0: Vec3,
    }

    #[derive(Debug)]
    pub struct Avatar_enemySPGShotSound {
        pub a0: Vec3,
        pub a1: Vec3,
    }

    #[derive(Debug)]
    pub struct Avatar_handleScriptEventFromServer {
        pub a0: RelaxString,
        pub a1: RelaxString,
        pub a2: RelaxString,
        pub a3: RelaxString,
        pub a4: RelaxString,
    }

    #[derive(Debug)]
    pub struct Avatar_setUpdatedGoodiesSnapshot {
        pub a0: Vec<BATTLE_GOODIE_RECORD>,
    }

    #[derive(Debug)]
    pub struct Avatar_onRandomEvent {
        pub a0: RelaxString,
    }

}

// Method for Avatar on base
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct Avatar_logLag {
    }

    #[derive(Debug)]
    pub struct Avatar_setClientReady {
    }

    #[derive(Debug)]
    pub struct Avatar_leaveArena {
    }

    #[derive(Debug)]
    pub struct Avatar_onLoginToCellFailed {
    }

    #[derive(Debug)]
    pub struct Avatar_confirmBattleResultsReceiving {
    }

    #[derive(Debug)]
    pub struct Avatar_makeDenunciation {
        pub a0: OBJECT_ID,
        pub a1: i32,
        pub a2: i8,
    }

    #[derive(Debug)]
    pub struct Avatar_banUnbanUser {
        pub a0: DB_ID,
        pub a1: u8,
        pub a2: u32,
        pub a3: RelaxString,
        pub a4: i8,
    }

    #[derive(Debug)]
    pub struct Avatar_requestToken {
        pub a0: u16,
        pub a1: u8,
    }

    #[derive(Debug)]
    pub struct Avatar_sendAccountStats {
        pub a0: u32,
        pub a1: Vec<RelaxString>,
    }

    #[derive(Debug)]
    pub struct Avatar_setClientCtx {
        pub a0: RelaxString,
    }

    #[derive(Debug)]
    pub struct Avatar_vehicle_teleport {
        pub a0: Vec3,
        pub a1: f32,
    }

    #[derive(Debug)]
    pub struct Avatar_vehicle_replenishAmmo {
    }

    #[derive(Debug)]
    pub struct Avatar_setDevelopmentFeature {
        pub a0: OBJECT_ID,
        pub a1: RelaxString,
        pub a2: i32,
        pub a3: RelaxString,
    }

    #[derive(Debug)]
    pub struct Avatar_addBotToArena {
        pub a0: RelaxString,
        pub a1: u8,
        pub a2: RelaxString,
        pub a3: Vec3,
        pub a4: u8,
    }

    #[derive(Debug)]
    pub struct Avatar_receiveFakeShot {
        pub a0: i32,
        pub a1: f32,
        pub a2: Vec3,
        pub a3: Vec3,
        pub a4: u8,
    }

    #[derive(Debug)]
    pub struct Avatar_logStreamCorruption {
        pub a0: i16,
        pub a1: i32,
        pub a2: i32,
        pub a3: i32,
        pub a4: i32,
    }

}

// Method for Avatar on cell
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct Avatar_autoAim {
        pub a0: OBJECT_ID,
        pub a1: BOOL,
    }

    #[derive(Debug)]
    pub struct Avatar_moveTo {
        pub a0: Vec3,
    }

    #[derive(Debug)]
    pub struct Avatar_bindToVehicle {
        pub a0: OBJECT_ID,
    }

    #[derive(Debug)]
    pub struct Avatar_monitorVehicleDamagedDevices {
        pub a0: OBJECT_ID,
    }

    #[derive(Debug)]
    pub struct Avatar_activateEquipment {
        pub a0: u16,
        pub a1: i16,
    }

    #[derive(Debug)]
    pub struct Avatar_setEquipmentApplicationPoint {
        pub a0: u16,
        pub a1: Vec3,
        pub a2: Vec2,
    }

    #[derive(Debug)]
    pub struct Avatar_switchViewPointOrBindToVehicle {
        pub a0: BOOL,
        pub a1: OBJECT_ID,
    }

    #[derive(Debug)]
    pub struct Avatar_setDualGunCharger {
        pub a0: BOOL,
    }

    #[derive(Debug)]
    pub struct Avatar_reportClientStats {
        pub a0: CLIENT_STATUS_STATISTICS,
    }

    #[derive(Debug)]
    pub struct Avatar_vehicle_moveWith {
        pub a0: u8,
    }

    #[derive(Debug)]
    pub struct Avatar_vehicle_shoot {
    }

    #[derive(Debug)]
    pub struct Avatar_vehicle_trackWorldPointWithGun {
        pub a0: Vec3,
    }

    #[derive(Debug)]
    pub struct Avatar_vehicle_trackRelativePointWithGun {
        pub a0: Vec3,
    }

    #[derive(Debug)]
    pub struct Avatar_vehicle_stopTrackingWithGun {
        pub a0: f32,
        pub a1: f32,
    }

    #[derive(Debug)]
    pub struct Avatar_setupAmmo {
        pub a0: i64,
    }

    #[derive(Debug)]
    pub struct Avatar_vehicle_changeSetting {
        pub a0: u8,
        pub a1: i32,
    }

    #[derive(Debug)]
    pub struct Avatar_setServerMarker {
        pub a0: BOOL,
    }

    #[derive(Debug)]
    pub struct Avatar_setSendKillCamSimulationData {
        pub a0: BOOL,
    }

}

// Entity methods for Avatar on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum Avatar_Client {
        RecoveryMechanic_Avatar_notifyCannotStartRecovering(0x00, 0),
        RecoveryMechanic_Avatar_notifyCancelled(0x01, 0),
        RespawnController_Avatar_updatePlayerLives(0x02, 1),
        AvatarEpic_enteringProtectionZone(0x03, 1),
        AvatarEpic_leavingProtectionZone(0x04, 1),
        AvatarEpic_protectionZoneShooting(0x05, 1),
        AvatarEpic_onSectorShooting(0x06, 1),
        AvatarEpic_onRankUpdate(0x07, 1),
        Avatar_onAutoAimVehicleLost(0x08, 1),
        Avatar_onKickedFromArena(0x09, 1),
        AvatarEpic_onXPUpdated(0x0A, 2),
        Avatar_onRoundFinished(0x0B, 2),
        RespawnController_Avatar_explodeVehicleBeforeRespawn(0x0C, 4),
        VehicleRemovalController_Avatar_removeVehicle(0x0D, 4),
        Avatar_updateTargetVehicleID(0x0E, 4),
        AvatarEpic_onDestructibleDestroyed(0x0F, 5),
        Avatar_updateResourceAmount(0x10, 5),
        Avatar_updateVehicleQuickShellChanger(0x11, 5),
        AvatarEpic_onSectorBaseAction(0x12, 6),
        Avatar_onRepairPointAction(0x13, 6),
        Avatar_updateVehicleHealth(0x14, 9),
        Avatar_updateVehicleSetting(0x15, 9),
        AvatarEpic_onStepRepairPointAction(0x16, 11),
        VehicleHealthBroadcastListenerComponent_Avatar_onVehicleHealthChanged(0x17, 12),
        AvatarEpic_welcomeToSector(0x18, 12),
        Avatar_enemySPGHit(0x19, 12),
        RecoveryMechanic_Avatar_updateState(0x1A, 13),
        AvatarEpic_onCrewRoleFactorAndRankUpdate(0x1B, 13),
        Avatar_onCombatEquipmentShotLaunched(0x1C, 14),
        Avatar_onSwitchViewpoint(0x1D, 16),
        Avatar_stopTracer(0x1E, 16),
        Avatar_onCollisionWithVehicle(0x1F, 16),
        Avatar_onSmoke(0x20, 16),
        Avatar_onFrictionWithVehicle(0x21, 17),
        Avatar_updateVehicleAmmo(0x22, 18),
        Avatar_showOwnVehicleHitDirection(0x23, 21),
        Avatar_enemySPGShotSound(0x24, 24),
        Avatar_showHittingArea(0x25, 34),
        Avatar_showCarpetBombing(0x26, 34),
        Avatar_battleEventsSummary(0x27, 34),
        Avatar_updateTargetingInfo(0x28, 36),
        Avatar_showTracer(0x29, 43),
        Chat_onChatAction(0x2A, var8),
        PlayerMessenger_chat2_messenger_onActionByServer_chat2(0x2B, var8),
        ClientCommandsPort_onCmdResponse(0x2C, var8),
        ClientCommandsPort_onCmdResponseExt(0x2D, var8),
        InvitationsClient_processInvitations(0x2E, var8),
        AccountAuthTokenProviderClient_onTokenReceived(0x2F, var8),
        TeamHealthBar_Avatar_updateTeamsHealthPercentage(0x30, var8),
        RespawnController_Avatar_redrawVehicleOnRespawn(0x31, var8),
        RespawnController_Avatar_updateRespawnVehicles(0x32, var8),
        RespawnController_Avatar_updateRespawnCooldowns(0x33, var8),
        RespawnController_Avatar_updateRespawnInfo(0x34, var8),
        RespawnController_Avatar_updateVehicleLimits(0x35, var8),
        RespawnController_Avatar_onTeamLivesRestored(0x36, var8),
        TriggersController_Avatar_externalTrigger(0x37, var8),
        AvatarEpic_syncPurchasedAbilities(0x38, var8),
        AvatarEpic_onRandomReserveOffer(0x39, var8),
        AvatarEpic_showDestructibleShotResults(0x3A, var8),
        Avatar_update(0x3B, var8),
        Avatar_onKickedFromServer(0x3C, var8),
        Avatar_onIGRTypeChanged(0x3D, var8),
        Avatar_receiveAccountStats(0x3E, var8),
        Avatar_showOtherVehicleDamagedDevices(0x3F, var8),
        Avatar_showShotResults(0x40, var8),
        Avatar_showDevelopmentInfo(0x41, var8),
        Avatar_explodeProjectile(0x42, var8),
        Avatar_onBattleEvents(0x43, var8),
        Avatar_updateArena(0x44, var8),
        Avatar_updatePositions(0x45, var8),
        Avatar_receivePhysicsDebugInfo(0x46, var8),
        Avatar_updateCarriedFlagPositions(0x47, var8),
        Avatar_receiveNotification(0x48, var8),
        Avatar_updateAvatarPrivateStats(0x49, var8),
        Avatar_updateQuestProgress(0x4A, var8),
        Avatar_handleScriptEventFromServer(0x4B, var8),
        Avatar_setUpdatedGoodiesSnapshot(0x4C, var8),
        Avatar_onRandomEvent(0x4D, var8),
        VehiclesSpawnListStorage_Avatar_updateSpawnList(0x4E, var16),
    }
}

// Entity methods for Avatar on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum Avatar_Base {
        RespawnController_Avatar_respawnController_performRespawn(0x00, 0),
        Avatar_logLag(0x01, 0),
        Avatar_setClientReady(0x02, 0),
        Avatar_leaveArena(0x03, 0),
        Avatar_onLoginToCellFailed(0x04, 0),
        Avatar_confirmBattleResultsReceiving(0x05, 0),
        Avatar_vehicle_replenishAmmo(0x06, 0),
        RespawnController_Avatar_respawnController_requestRespawnGroupChange(0x07, 1),
        AvatarEpic_enableFrontLineDevInfo(0x08, 1),
        RespawnController_Avatar_respawnController_chooseVehicleForRespawn(0x09, 2),
        Chat_onStreamComplete(0x0A, 3),
        Avatar_requestToken(0x0B, 3),
        ClientCommandsPort_doCmdNoArgs(0x0C, 4),
        RespawnController_Avatar_respawnController_switchSetup(0x0D, 4),
        Avatar_makeDenunciation(0x0E, 9),
        ClientCommandsPort_doCmdInt(0x0F, 12),
        RespawnController_Avatar_respawnController_chooseRespawnZone(0x10, 12),
        Avatar_vehicle_teleport(0x11, 16),
        Avatar_logStreamCorruption(0x12, 18),
        ClientCommandsPort_doCmdInt2(0x13, 20),
        ClientCommandsPort_doCmdInt3(0x14, 28),
        ClientCommandsPort_doCmdInt4(0x15, 28),
        Chat_ackCommand(0x16, 33),
        Avatar_receiveFakeShot(0x17, 33),
        Chat_chatCommandFromClient(0x18, var8),
        Chat_inviteCommand(0x19, var8),
        PlayerMessenger_chat2_messenger_onActionByClient_chat2(0x1A, var8),
        ClientCommandsPort_doCmdStr(0x1B, var8),
        ClientCommandsPort_doCmdInt2Str(0x1C, var8),
        ClientCommandsPort_doCmdInt3Str(0x1D, var8),
        ClientCommandsPort_doCmdIntArr(0x1E, var8),
        ClientCommandsPort_doCmdIntStr(0x1F, var8),
        ClientCommandsPort_doCmdIntStrArr(0x20, var8),
        ClientCommandsPort_doCmdIntArrStrArr(0x21, var8),
        ClientCommandsPort_doCmdStrArr(0x22, var8),
        Avatar_banUnbanUser(0x23, var8),
        Avatar_sendAccountStats(0x24, var8),
        Avatar_setClientCtx(0x25, var8),
        Avatar_setDevelopmentFeature(0x26, var8),
        Avatar_addBotToArena(0x27, var8),
    }
}

// Entity methods for Avatar on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum Avatar_Cell {
        Avatar_vehicle_shoot(0x00, 0),
        AvatarObserver_switchObserverFPV(0x01, 1),
        Avatar_setDualGunCharger(0x02, 1),
        Avatar_vehicle_moveWith(0x03, 1),
        Avatar_setServerMarker(0x04, 1),
        Avatar_setSendKillCamSimulationData(0x05, 1),
        Avatar_bindToVehicle(0x06, 4),
        Avatar_monitorVehicleDamagedDevices(0x07, 4),
        Avatar_activateEquipment(0x08, 4),
        Avatar_autoAim(0x09, 5),
        Avatar_switchViewPointOrBindToVehicle(0x0A, 5),
        Avatar_vehicle_changeSetting(0x0B, 5),
        Avatar_vehicle_stopTrackingWithGun(0x0C, 8),
        Avatar_setupAmmo(0x0D, 8),
        Avatar_moveTo(0x0E, 12),
        Avatar_vehicle_trackWorldPointWithGun(0x0F, 12),
        Avatar_vehicle_trackRelativePointWithGun(0x10, 12),
        Avatar_setEquipmentApplicationPoint(0x11, 22),
        Avatar_reportClientStats(0x12, 24),
    }
}

impl Avatar {
    const TYPE_ID: u16 = 0x02;
}

impl DataTypeEntity for Avatar {
    type ClientMethod = Avatar_Client;
    type BaseMethod = Avatar_Base;
    type CellMethod = Avatar_Cell;
}

// Entity 0x03
// Interface ArenaInfo
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct ArenaInfo {
        pub i_PlaneTrajectoryArenaInfo: PlaneTrajectoryArenaInfo,
    }
}

// Method for ArenaInfo on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct ArenaInfo_showCarpetBombing {
        pub a0: u16,
        pub a1: Vec3,
        pub a2: Vec3,
        pub a3: f32,
    }

}

// Method for ArenaInfo on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for ArenaInfo on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for ArenaInfo on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ArenaInfo_Client {
        ArenaInfo_showCarpetBombing(0x00, 30),
    }
}

// Entity methods for ArenaInfo on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ArenaInfo_Base {
    }
}

// Entity methods for ArenaInfo on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ArenaInfo_Cell {
    }
}

impl ArenaInfo {
    const TYPE_ID: u16 = 0x03;
}

impl DataTypeEntity for ArenaInfo {
    type ClientMethod = ArenaInfo_Client;
    type BaseMethod = ArenaInfo_Base;
    type CellMethod = ArenaInfo_Cell;
}

// Entity 0x04
// Interface ClientSelectableObject
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct ClientSelectableObject {
        pub modelName: RelaxString,
        pub selectionId: RelaxString,
        pub mouseOverSoundName: RelaxString,
        pub isOver3DSound: BOOL,
        pub clickSoundName: RelaxString,
        pub isClick3DSound: BOOL,
        pub edgeMode: u8,
    }
}

// Method for ClientSelectableObject on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for ClientSelectableObject on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for ClientSelectableObject on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for ClientSelectableObject on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ClientSelectableObject_Client {
    }
}

// Entity methods for ClientSelectableObject on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ClientSelectableObject_Base {
    }
}

// Entity methods for ClientSelectableObject on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ClientSelectableObject_Cell {
    }
}

impl ClientSelectableObject {
    const TYPE_ID: u16 = 0x04;
}

impl DataTypeEntity for ClientSelectableObject {
    type ClientMethod = ClientSelectableObject_Client;
    type BaseMethod = ClientSelectableObject_Base;
    type CellMethod = ClientSelectableObject_Cell;
}

// Entity 0x05
// Interface HangarVehicle
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct HangarVehicle {
    }
}

// Method for HangarVehicle on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for HangarVehicle on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for HangarVehicle on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for HangarVehicle on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum HangarVehicle_Client {
    }
}

// Entity methods for HangarVehicle on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum HangarVehicle_Base {
    }
}

// Entity methods for HangarVehicle on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum HangarVehicle_Cell {
    }
}

impl HangarVehicle {
    const TYPE_ID: u16 = 0x05;
}

impl DataTypeEntity for HangarVehicle {
    type ClientMethod = HangarVehicle_Client;
    type BaseMethod = HangarVehicle_Base;
    type CellMethod = HangarVehicle_Cell;
}

// Entity 0x06
// Interface Vehicle
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct Vehicle {
        pub i_VehicleObserver: VehicleObserver,
        pub i_Wheels: Wheels,
        pub i_Perks_Vehicle: Perks_Vehicle,
        pub isStrafing: BOOL,
        pub postmortemViewPointName: RelaxString,
        pub isHidden: BOOL,
        pub physicsMode: u8,
        pub siegeState: u8,
        pub gunAnglesPacked: u16,
        pub publicInfo: PUBLIC_VEHICLE_INFO,
        pub health: i16,
        pub isCrewActive: BOOL,
        pub engineMode: Box<[u8; 2]>,
        pub damageStickers: Vec<u64>,
        pub publicStateModifiers: Vec<EXTRA_ID>,
        pub stunInfo: STUN_INFO,
        pub crewCompactDescrs: Vec<RelaxString>,
        pub enhancements: Python,
        pub setups: Python,
        pub setupsIndexes: Python,
        pub customRoleSlotTypeId: u8,
        pub vehPerks: Python,
        pub vehPostProgression: Vec<i32>,
        pub disabledSwitches: Vec<i32>,
        pub avatarID: OBJECT_ID,
        pub masterVehID: u32,
        pub arenaTypeID: i32,
        pub arenaBonusType: u8,
        pub arenaUniqueID: u64,
        pub inspiringEffect: BUFF_EFFECT,
        pub healingEffect: BUFF_EFFECT,
        pub dotEffect: DOT_EFFECT,
        pub inspired: INSPIRED_EFFECT,
        pub healing: BUFF_EFFECT_INACTIVATION,
        pub healOverTime: HOT_EFFECT,
        pub debuff: i32,
        pub isSpeedCapturing: BOOL,
        pub isBlockingCapture: BOOL,
        pub dogTag: BATTLE_DOG_TAG,
        pub isMyVehicle: BOOL,
        pub quickShellChangerFactor: f32,
        pub onRespawnReloadTimeFactor: f32,
        pub ownVehiclePosition: OWN_VEHICLE_POSITION,
        pub enableExternalRespawn: BOOL,
        pub botDisplayStatus: u8,
    }
}

// Method for Vehicle on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct Vehicle_onVehiclePickup {
    }

    #[derive(Debug)]
    pub struct Vehicle_onExtraHitted {
        pub a0: i16,
        pub a1: Vec3,
    }

    #[derive(Debug)]
    pub struct Vehicle_onHealthChanged {
        pub a0: i16,
        pub a1: i16,
        pub a2: OBJECT_ID,
        pub a3: u8,
        pub a4: i8,
    }

    #[derive(Debug)]
    pub struct Vehicle_showShooting {
        pub a0: u8,
        pub a1: i8,
    }

    #[derive(Debug)]
    pub struct Vehicle_updateLaserSight {
        pub a0: OBJECT_ID,
        pub a1: BOOL,
        pub a2: RelaxString,
    }

    #[derive(Debug)]
    pub struct Vehicle_showDamageFromShot {
        pub a0: OBJECT_ID,
        pub a1: Vec<u64>,
        pub a2: u8,
        pub a3: i32,
        pub a4: u8,
        pub a5: BOOL,
    }

    #[derive(Debug)]
    pub struct Vehicle_showDamageFromExplosion {
        pub a0: OBJECT_ID,
        pub a1: Vec3,
        pub a2: u8,
        pub a3: i32,
        pub a4: u8,
    }

    #[derive(Debug)]
    pub struct Vehicle_showAmmoBayEffect {
        pub a0: u8,
        pub a1: f32,
        pub a2: f32,
    }

    #[derive(Debug)]
    pub struct Vehicle_onPushed {
        pub a0: f32,
        pub a1: f32,
    }

    #[derive(Debug)]
    pub struct Vehicle_onStaticCollision {
        pub a0: f32,
        pub a1: Vec3,
        pub a2: Vec3,
        pub a3: u8,
        pub a4: f32,
        pub a5: i8,
        pub a6: u16,
    }

    #[derive(Debug)]
    pub struct Vehicle_showRammingEffect {
        pub a0: f32,
        pub a1: Vec3,
    }

}

// Method for Vehicle on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for Vehicle on cell
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct Vehicle_moveWith {
        pub a0: u8,
    }

    #[derive(Debug)]
    pub struct Vehicle_trackWorldPointWithGun {
        pub a0: Vec3,
    }

    #[derive(Debug)]
    pub struct Vehicle_trackRelativePointWithGun {
        pub a0: Vec3,
    }

    #[derive(Debug)]
    pub struct Vehicle_stopTrackingWithGun {
        pub a0: f32,
        pub a1: f32,
    }

    #[derive(Debug)]
    pub struct Vehicle_changeSetting {
        pub a0: u8,
        pub a1: i32,
    }

    #[derive(Debug)]
    pub struct Vehicle_sendVisibilityDevelopmentInfo {
        pub a0: OBJECT_ID,
        pub a1: Vec3,
    }

    #[derive(Debug)]
    pub struct Vehicle_sendStateToOwnClient {
    }

    #[derive(Debug)]
    pub struct Vehicle_switchSetup {
        pub a0: u8,
        pub a1: u8,
    }

}

// Entity methods for Vehicle on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum Vehicle_Client {
        Vehicle_onVehiclePickup(0x00, 0),
        Vehicle_showShooting(0x01, 2),
        Vehicle_onPushed(0x02, 8),
        Vehicle_showAmmoBayEffect(0x03, 9),
        Vehicle_onHealthChanged(0x04, 10),
        Vehicle_onExtraHitted(0x05, 14),
        Vehicle_showRammingEffect(0x06, 16),
        Vehicle_showDamageFromExplosion(0x07, 22),
        Vehicle_onStaticCollision(0x08, 36),
        Vehicle_updateLaserSight(0x09, var8),
        Vehicle_showDamageFromShot(0x0A, var8),
    }
}

// Entity methods for Vehicle on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum Vehicle_Base {
    }
}

// Entity methods for Vehicle on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum Vehicle_Cell {
        RecoveryMechanic_Vehicle_recoveryMechanic_startRecovering(0x00, 0),
        RecoveryMechanic_Vehicle_recoveryMechanic_stopRecovering(0x01, 0),
        Vehicle_sendStateToOwnClient(0x02, 0),
        Vehicle_moveWith(0x03, 1),
        Vehicle_switchSetup(0x04, 2),
        Vehicle_changeSetting(0x05, 5),
        Vehicle_stopTrackingWithGun(0x06, 8),
        Vehicle_trackWorldPointWithGun(0x07, 12),
        Vehicle_trackRelativePointWithGun(0x08, 12),
        Vehicle_sendVisibilityDevelopmentInfo(0x09, 16),
        VehicleObserver_setRemoteCamera(0x0A, 22),
    }
}

impl Vehicle {
    const TYPE_ID: u16 = 0x06;
}

impl DataTypeEntity for Vehicle {
    type ClientMethod = Vehicle_Client;
    type BaseMethod = Vehicle_Base;
    type CellMethod = Vehicle_Cell;
}

// Entity 0x07
// Interface AreaDestructibles
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AreaDestructibles {
        pub destroyedModules: Vec<Box<[u8; 3]>>,
        pub destroyedFragiles: Vec<Box<[u8; 3]>>,
        pub fallenColumns: Vec<Box<[u8; 3]>>,
        pub fallenTrees: Vec<Box<[u8; 5]>>,
    }
}

// Method for AreaDestructibles on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for AreaDestructibles on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for AreaDestructibles on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for AreaDestructibles on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum AreaDestructibles_Client {
    }
}

// Entity methods for AreaDestructibles on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum AreaDestructibles_Base {
    }
}

// Entity methods for AreaDestructibles on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum AreaDestructibles_Cell {
    }
}

impl AreaDestructibles {
    const TYPE_ID: u16 = 0x07;
}

impl DataTypeEntity for AreaDestructibles {
    type ClientMethod = AreaDestructibles_Client;
    type BaseMethod = AreaDestructibles_Base;
    type CellMethod = AreaDestructibles_Cell;
}

// Entity 0x08
// Interface OfflineEntity
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct OfflineEntity {
    }
}

// Method for OfflineEntity on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for OfflineEntity on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for OfflineEntity on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for OfflineEntity on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum OfflineEntity_Client {
    }
}

// Entity methods for OfflineEntity on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum OfflineEntity_Base {
    }
}

// Entity methods for OfflineEntity on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum OfflineEntity_Cell {
    }
}

impl OfflineEntity {
    const TYPE_ID: u16 = 0x08;
}

impl DataTypeEntity for OfflineEntity {
    type ClientMethod = OfflineEntity_Client;
    type BaseMethod = OfflineEntity_Base;
    type CellMethod = OfflineEntity_Cell;
}

// Entity 0x09
// Interface Flock
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct Flock {
        pub modelName: RelaxString,
        pub modelName2: RelaxString,
        pub modelCount: u8,
        pub yawSpeed: f32,
        pub pitchSpeed: f32,
        pub rollSpeed: f32,
        pub animSpeedMin: f32,
        pub animSpeedMax: f32,
        pub height: f32,
        pub radius: f32,
        pub deadZoneRadius: f32,
        pub speedAtBottom: f32,
        pub speedAtTop: f32,
        pub decisionTime: f32,
        pub flyAroundCenter: BOOL,
    }
}

// Method for Flock on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for Flock on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for Flock on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for Flock on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum Flock_Client {
    }
}

// Entity methods for Flock on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum Flock_Base {
    }
}

// Entity methods for Flock on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum Flock_Cell {
    }
}

impl Flock {
    const TYPE_ID: u16 = 0x09;
}

impl DataTypeEntity for Flock {
    type ClientMethod = Flock_Client;
    type BaseMethod = Flock_Base;
    type CellMethod = Flock_Cell;
}

// Entity 0x0A
// Interface FlockExotic
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct FlockExotic {
        pub animSpeedMax: f32,
        pub animSpeedMin: f32,
        pub modelCount: u8,
        pub modelName: RelaxString,
        pub modelName2: RelaxString,
        pub speed: f32,
        pub initSpeedRandom: Vec2,
        pub speedRandom: Vec2,
        pub accelerationTime: f32,
        pub triggerRadius: f32,
        pub explosionRadius: Vec2,
        pub spawnRadius: f32,
        pub spawnHeight: f32,
        pub flightRadius: f32,
        pub flightHeight: f32,
        pub flightAngleMin: f32,
        pub flightAngleMax: f32,
        pub flightOffsetFromOrigin: f32,
        pub lifeTime: f32,
        pub respawnTime: f32,
        pub flightSound: RelaxString,
    }
}

// Method for FlockExotic on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for FlockExotic on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for FlockExotic on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for FlockExotic on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum FlockExotic_Client {
    }
}

// Entity methods for FlockExotic on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum FlockExotic_Base {
    }
}

// Entity methods for FlockExotic on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum FlockExotic_Cell {
    }
}

impl FlockExotic {
    const TYPE_ID: u16 = 0x0A;
}

impl DataTypeEntity for FlockExotic {
    type ClientMethod = FlockExotic_Client;
    type BaseMethod = FlockExotic_Base;
    type CellMethod = FlockExotic_Cell;
}

// Entity 0x0B
// Interface Login
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct Login {
        pub accountDBID_s: RelaxString,
    }
}

// Method for Login on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct Login_onKickedFromServer {
        pub a0: i32,
    }

    #[derive(Debug)]
    pub struct Login_receiveLoginQueueNumber {
        pub a0: u64,
    }

    #[derive(Debug)]
    pub struct Login_setPeripheryRoutingGroup {
        pub a0: RelaxString,
        pub a1: Python,
    }

}

// Method for Login on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for Login on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for Login on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum Login_Client {
        Login_onKickedFromServer(0x00, 4),
        Login_receiveLoginQueueNumber(0x01, 8),
        Login_setPeripheryRoutingGroup(0x02, var8),
    }
}

// Entity methods for Login on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum Login_Base {
    }
}

// Entity methods for Login on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum Login_Cell {
    }
}

impl Login {
    const TYPE_ID: u16 = 0x0B;
}

impl DataTypeEntity for Login {
    type ClientMethod = Login_Client;
    type BaseMethod = Login_Base;
    type CellMethod = Login_Cell;
}

// Entity 0x0C
// Interface DetachedTurret
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct DetachedTurret {
        pub vehicleCompDescr: RelaxString,
        pub outfitCD: RelaxString,
        pub isUnderWater: BOOL,
        pub isCollidingWithWorld: BOOL,
        pub vehicleID: i32,
    }
}

// Method for DetachedTurret on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct DetachedTurret_onStaticCollision {
        pub a0: f32,
        pub a1: Vec3,
        pub a2: Vec3,
    }

    #[derive(Debug)]
    pub struct DetachedTurret_showDamageFromShot {
        pub a0: Vec<u64>,
        pub a1: u8,
    }

}

// Method for DetachedTurret on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for DetachedTurret on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for DetachedTurret on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum DetachedTurret_Client {
        DetachedTurret_onStaticCollision(0x00, 28),
        DetachedTurret_showDamageFromShot(0x01, var8),
    }
}

// Entity methods for DetachedTurret on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum DetachedTurret_Base {
    }
}

// Entity methods for DetachedTurret on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum DetachedTurret_Cell {
    }
}

impl DetachedTurret {
    const TYPE_ID: u16 = 0x0C;
}

impl DataTypeEntity for DetachedTurret {
    type ClientMethod = DetachedTurret_Client;
    type BaseMethod = DetachedTurret_Base;
    type CellMethod = DetachedTurret_Cell;
}

// Entity 0x0D
// Interface DebugDrawEntity
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct DebugDrawEntity {
        pub drawObjects: Vec<ANON180>,
    }
}

// Method for DebugDrawEntity on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for DebugDrawEntity on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for DebugDrawEntity on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for DebugDrawEntity on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum DebugDrawEntity_Client {
    }
}

// Entity methods for DebugDrawEntity on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum DebugDrawEntity_Base {
    }
}

// Entity methods for DebugDrawEntity on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum DebugDrawEntity_Cell {
    }
}

impl DebugDrawEntity {
    const TYPE_ID: u16 = 0x0D;
}

impl DataTypeEntity for DebugDrawEntity {
    type ClientMethod = DebugDrawEntity_Client;
    type BaseMethod = DebugDrawEntity_Base;
    type CellMethod = DebugDrawEntity_Cell;
}

// Entity 0x0E
// Interface ClientSelectableCameraObject
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct ClientSelectableCameraObject {
    }
}

// Method for ClientSelectableCameraObject on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for ClientSelectableCameraObject on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for ClientSelectableCameraObject on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for ClientSelectableCameraObject on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ClientSelectableCameraObject_Client {
    }
}

// Entity methods for ClientSelectableCameraObject on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ClientSelectableCameraObject_Base {
    }
}

// Entity methods for ClientSelectableCameraObject on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ClientSelectableCameraObject_Cell {
    }
}

impl ClientSelectableCameraObject {
    const TYPE_ID: u16 = 0x0E;
}

impl DataTypeEntity for ClientSelectableCameraObject {
    type ClientMethod = ClientSelectableCameraObject_Client;
    type BaseMethod = ClientSelectableCameraObject_Base;
    type CellMethod = ClientSelectableCameraObject_Cell;
}

// Entity 0x0F
// Interface ClientSelectableCameraVehicle
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct ClientSelectableCameraVehicle {
        pub modelName: RelaxString,
    }
}

// Method for ClientSelectableCameraVehicle on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for ClientSelectableCameraVehicle on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for ClientSelectableCameraVehicle on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for ClientSelectableCameraVehicle on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ClientSelectableCameraVehicle_Client {
    }
}

// Entity methods for ClientSelectableCameraVehicle on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ClientSelectableCameraVehicle_Base {
    }
}

// Entity methods for ClientSelectableCameraVehicle on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ClientSelectableCameraVehicle_Cell {
    }
}

impl ClientSelectableCameraVehicle {
    const TYPE_ID: u16 = 0x0F;
}

impl DataTypeEntity for ClientSelectableCameraVehicle {
    type ClientMethod = ClientSelectableCameraVehicle_Client;
    type BaseMethod = ClientSelectableCameraVehicle_Base;
    type CellMethod = ClientSelectableCameraVehicle_Cell;
}

// Entity 0x10
// Interface ClientSelectableWebLinksOpener
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct ClientSelectableWebLinksOpener {
        pub url: RelaxString,
    }
}

// Method for ClientSelectableWebLinksOpener on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for ClientSelectableWebLinksOpener on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for ClientSelectableWebLinksOpener on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for ClientSelectableWebLinksOpener on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ClientSelectableWebLinksOpener_Client {
    }
}

// Entity methods for ClientSelectableWebLinksOpener on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ClientSelectableWebLinksOpener_Base {
    }
}

// Entity methods for ClientSelectableWebLinksOpener on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ClientSelectableWebLinksOpener_Cell {
    }
}

impl ClientSelectableWebLinksOpener {
    const TYPE_ID: u16 = 0x10;
}

impl DataTypeEntity for ClientSelectableWebLinksOpener {
    type ClientMethod = ClientSelectableWebLinksOpener_Client;
    type BaseMethod = ClientSelectableWebLinksOpener_Base;
    type CellMethod = ClientSelectableWebLinksOpener_Cell;
}

// Entity 0x11
// Interface ClientSelectableEasterEgg
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct ClientSelectableEasterEgg {
        pub imageName: RelaxString,
        pub multiLanguageSupport: BOOL,
        pub outlineModelName: RelaxString,
        pub animationSequence: RelaxString,
    }
}

// Method for ClientSelectableEasterEgg on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for ClientSelectableEasterEgg on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for ClientSelectableEasterEgg on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for ClientSelectableEasterEgg on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ClientSelectableEasterEgg_Client {
    }
}

// Entity methods for ClientSelectableEasterEgg on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ClientSelectableEasterEgg_Base {
    }
}

// Entity methods for ClientSelectableEasterEgg on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ClientSelectableEasterEgg_Cell {
    }
}

impl ClientSelectableEasterEgg {
    const TYPE_ID: u16 = 0x11;
}

impl DataTypeEntity for ClientSelectableEasterEgg {
    type ClientMethod = ClientSelectableEasterEgg_Client;
    type BaseMethod = ClientSelectableEasterEgg_Base;
    type CellMethod = ClientSelectableEasterEgg_Cell;
}

// Entity 0x12
// Interface EmptyEntity
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct EmptyEntity {
    }
}

// Method for EmptyEntity on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for EmptyEntity on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for EmptyEntity on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for EmptyEntity on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum EmptyEntity_Client {
    }
}

// Entity methods for EmptyEntity on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum EmptyEntity_Base {
    }
}

// Entity methods for EmptyEntity on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum EmptyEntity_Cell {
    }
}

impl EmptyEntity {
    const TYPE_ID: u16 = 0x12;
}

impl DataTypeEntity for EmptyEntity {
    type ClientMethod = EmptyEntity_Client;
    type BaseMethod = EmptyEntity_Base;
    type CellMethod = EmptyEntity_Cell;
}

// Entity 0x13
// Interface LimitedVisibilityEntity
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct LimitedVisibilityEntity {
    }
}

// Method for LimitedVisibilityEntity on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for LimitedVisibilityEntity on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for LimitedVisibilityEntity on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for LimitedVisibilityEntity on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum LimitedVisibilityEntity_Client {
    }
}

// Entity methods for LimitedVisibilityEntity on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum LimitedVisibilityEntity_Base {
    }
}

// Entity methods for LimitedVisibilityEntity on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum LimitedVisibilityEntity_Cell {
    }
}

impl LimitedVisibilityEntity {
    const TYPE_ID: u16 = 0x13;
}

impl DataTypeEntity for LimitedVisibilityEntity {
    type ClientMethod = LimitedVisibilityEntity_Client;
    type BaseMethod = LimitedVisibilityEntity_Base;
    type CellMethod = LimitedVisibilityEntity_Cell;
}

// Entity 0x14
// Interface HeroTank
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct HeroTank {
        pub markerHeightFactor: f32,
        pub vehicleTurretYaw: f32,
        pub vehicleGunPitch: f32,
    }
}

// Method for HeroTank on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for HeroTank on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for HeroTank on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for HeroTank on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum HeroTank_Client {
    }
}

// Entity methods for HeroTank on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum HeroTank_Base {
    }
}

// Entity methods for HeroTank on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum HeroTank_Cell {
    }
}

impl HeroTank {
    const TYPE_ID: u16 = 0x14;
}

impl DataTypeEntity for HeroTank {
    type ClientMethod = HeroTank_Client;
    type BaseMethod = HeroTank_Base;
    type CellMethod = HeroTank_Cell;
}

// Entity 0x15
// Interface PlatoonTank
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct PlatoonTank {
        pub markerHeightFactor: f32,
        pub vehicleTurretYaw: f32,
        pub vehicleGunPitch: f32,
        pub slotIndex: i32,
    }
}

// Method for PlatoonTank on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for PlatoonTank on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for PlatoonTank on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for PlatoonTank on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum PlatoonTank_Client {
    }
}

// Entity methods for PlatoonTank on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum PlatoonTank_Base {
    }
}

// Entity methods for PlatoonTank on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum PlatoonTank_Cell {
    }
}

impl PlatoonTank {
    const TYPE_ID: u16 = 0x15;
}

impl DataTypeEntity for PlatoonTank {
    type ClientMethod = PlatoonTank_Client;
    type BaseMethod = PlatoonTank_Base;
    type CellMethod = PlatoonTank_Cell;
}

// Entity 0x16
// Interface PlatoonLighting
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct PlatoonLighting {
        pub animationStateMachine: RelaxString,
    }
}

// Method for PlatoonLighting on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for PlatoonLighting on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for PlatoonLighting on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for PlatoonLighting on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum PlatoonLighting_Client {
    }
}

// Entity methods for PlatoonLighting on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum PlatoonLighting_Base {
    }
}

// Entity methods for PlatoonLighting on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum PlatoonLighting_Cell {
    }
}

impl PlatoonLighting {
    const TYPE_ID: u16 = 0x16;
}

impl DataTypeEntity for PlatoonLighting {
    type ClientMethod = PlatoonLighting_Client;
    type BaseMethod = PlatoonLighting_Base;
    type CellMethod = PlatoonLighting_Cell;
}

// Entity 0x17
// Interface SectorBase
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct SectorBase {
        pub isActive: BOOL,
        pub team: u8,
        pub baseID: u8,
        pub sectorID: u8,
        pub maxPoints: f32,
        pub pointsPercentage: u8,
        pub capturingStopped: BOOL,
        pub onDamageCooldownTime: f32,
        pub radius: f32,
        pub isCaptured: BOOL,
        pub invadersCount: u8,
        pub expectedCaptureTime: f32,
    }
}

// Method for SectorBase on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for SectorBase on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for SectorBase on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for SectorBase on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum SectorBase_Client {
    }
}

// Entity methods for SectorBase on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum SectorBase_Base {
    }
}

// Entity methods for SectorBase on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum SectorBase_Cell {
    }
}

impl SectorBase {
    const TYPE_ID: u16 = 0x17;
}

impl DataTypeEntity for SectorBase {
    type ClientMethod = SectorBase_Client;
    type BaseMethod = SectorBase_Base;
    type CellMethod = SectorBase_Cell;
}

// Entity 0x18
// Interface Sector
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct Sector {
        pub groupID: u8,
        pub sectorID: u8,
        pub playerGroup: u8,
        pub IDInPlayerGroup: u8,
        pub lengthX: f32,
        pub lengthZ: f32,
        pub team: u8,
        pub state: u8,
        pub transitionTime: f32,
        pub endOfTransitionPeriod: f32,
    }
}

// Method for Sector on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct Sector_showBomb {
        pub a0: Vec3,
    }

}

// Method for Sector on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for Sector on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for Sector on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum Sector_Client {
        Sector_showBomb(0x00, 12),
    }
}

// Entity methods for Sector on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum Sector_Base {
    }
}

// Entity methods for Sector on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum Sector_Cell {
    }
}

impl Sector {
    const TYPE_ID: u16 = 0x18;
}

impl DataTypeEntity for Sector {
    type ClientMethod = Sector_Client;
    type BaseMethod = Sector_Base;
    type CellMethod = Sector_Cell;
}

// Entity 0x19
// Interface DestructibleEntity
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct DestructibleEntity {
        pub isActive: BOOL,
        pub team: u8,
        pub destructibleEntityID: u8,
        pub health: f32,
        pub maxHealth: f32,
        pub isDestructibleDestroyed: BOOL,
        pub typeID: u8,
        pub linkedMapActivities: RelaxString,
        pub damageStickers: Vec<u64>,
    }
}

// Method for DestructibleEntity on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct DestructibleEntity_onHealthChanged {
        pub a0: i16,
        pub a1: OBJECT_ID,
        pub a2: u8,
        pub a3: i32,
    }

    #[derive(Debug)]
    pub struct DestructibleEntity_showDamageFromShot {
        pub a0: OBJECT_ID,
        pub a1: u8,
        pub a2: i32,
    }

    #[derive(Debug)]
    pub struct DestructibleEntity_showDamageFromExplosion {
        pub a0: OBJECT_ID,
        pub a1: i32,
    }

}

// Method for DestructibleEntity on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for DestructibleEntity on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for DestructibleEntity on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum DestructibleEntity_Client {
        DestructibleEntity_showDamageFromExplosion(0x00, 8),
        DestructibleEntity_showDamageFromShot(0x01, 9),
        DestructibleEntity_onHealthChanged(0x02, 11),
    }
}

// Entity methods for DestructibleEntity on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum DestructibleEntity_Base {
    }
}

// Entity methods for DestructibleEntity on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum DestructibleEntity_Cell {
    }
}

impl DestructibleEntity {
    const TYPE_ID: u16 = 0x19;
}

impl DataTypeEntity for DestructibleEntity {
    type ClientMethod = DestructibleEntity_Client;
    type BaseMethod = DestructibleEntity_Base;
    type CellMethod = DestructibleEntity_Cell;
}

// Entity 0x1A
// Interface StepRepairPoint
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct StepRepairPoint {
        pub team: u8,
        pub radius: f32,
    }
}

// Method for StepRepairPoint on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for StepRepairPoint on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for StepRepairPoint on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for StepRepairPoint on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum StepRepairPoint_Client {
    }
}

// Entity methods for StepRepairPoint on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum StepRepairPoint_Base {
    }
}

// Entity methods for StepRepairPoint on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum StepRepairPoint_Cell {
    }
}

impl StepRepairPoint {
    const TYPE_ID: u16 = 0x1A;
}

impl DataTypeEntity for StepRepairPoint {
    type ClientMethod = StepRepairPoint_Client;
    type BaseMethod = StepRepairPoint_Base;
    type CellMethod = StepRepairPoint_Cell;
}

// Entity 0x1B
// Interface ProtectionZone
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct ProtectionZone {
        pub zoneID: u8,
        pub lengthX: f32,
        pub lengthZ: f32,
        pub team: u8,
        pub isActive: BOOL,
    }
}

// Method for ProtectionZone on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for ProtectionZone on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for ProtectionZone on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for ProtectionZone on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ProtectionZone_Client {
    }
}

// Entity methods for ProtectionZone on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ProtectionZone_Base {
    }
}

// Entity methods for ProtectionZone on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ProtectionZone_Cell {
    }
}

impl ProtectionZone {
    const TYPE_ID: u16 = 0x1B;
}

impl DataTypeEntity for ProtectionZone {
    type ClientMethod = ProtectionZone_Client;
    type BaseMethod = ProtectionZone_Base;
    type CellMethod = ProtectionZone_Cell;
}

// Entity 0x1C
// Interface HangarPoster
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct HangarPoster {
        pub minAlpha: f32,
        pub maxAlphaDistance: f32,
    }
}

// Method for HangarPoster on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for HangarPoster on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for HangarPoster on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for HangarPoster on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum HangarPoster_Client {
    }
}

// Entity methods for HangarPoster on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum HangarPoster_Base {
    }
}

// Entity methods for HangarPoster on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum HangarPoster_Cell {
    }
}

impl HangarPoster {
    const TYPE_ID: u16 = 0x1C;
}

impl DataTypeEntity for HangarPoster {
    type ClientMethod = HangarPoster_Client;
    type BaseMethod = HangarPoster_Base;
    type CellMethod = HangarPoster_Cell;
}

// Entity 0x1D
// Interface TeamInfo
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct TeamInfo {
        pub teamID: i32,
    }
}

// Method for TeamInfo on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct TeamInfo_onCombatEquipmentUsed {
        pub a0: OBJECT_ID,
        pub a1: OBJECT_ID,
    }

    #[derive(Debug)]
    pub struct TeamInfo_showHittingArea {
        pub a0: u16,
        pub a1: Vec3,
        pub a2: Vec3,
        pub a3: f64,
    }

}

// Method for TeamInfo on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for TeamInfo on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for TeamInfo on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum TeamInfo_Client {
        TeamInfo_onCombatEquipmentUsed(0x00, 8),
        TeamInfo_showHittingArea(0x01, 34),
    }
}

// Entity methods for TeamInfo on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum TeamInfo_Base {
    }
}

// Entity methods for TeamInfo on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum TeamInfo_Cell {
    }
}

impl TeamInfo {
    const TYPE_ID: u16 = 0x1D;
}

impl DataTypeEntity for TeamInfo {
    type ClientMethod = TeamInfo_Client;
    type BaseMethod = TeamInfo_Base;
    type CellMethod = TeamInfo_Cell;
}

// Entity 0x1E
// Interface AvatarInfo
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AvatarInfo {
        pub avatarID: OBJECT_ID,
    }
}

// Method for AvatarInfo on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for AvatarInfo on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for AvatarInfo on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for AvatarInfo on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum AvatarInfo_Client {
    }
}

// Entity methods for AvatarInfo on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum AvatarInfo_Base {
    }
}

// Entity methods for AvatarInfo on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum AvatarInfo_Cell {
    }
}

impl AvatarInfo {
    const TYPE_ID: u16 = 0x1E;
}

impl DataTypeEntity for AvatarInfo {
    type ClientMethod = AvatarInfo_Client;
    type BaseMethod = AvatarInfo_Base;
    type CellMethod = AvatarInfo_Cell;
}

// Entity 0x1F
// Interface ArenaObserverInfo
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct ArenaObserverInfo {
    }
}

// Method for ArenaObserverInfo on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for ArenaObserverInfo on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for ArenaObserverInfo on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for ArenaObserverInfo on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ArenaObserverInfo_Client {
    }
}

// Entity methods for ArenaObserverInfo on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ArenaObserverInfo_Base {
    }
}

// Entity methods for ArenaObserverInfo on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ArenaObserverInfo_Cell {
    }
}

impl ArenaObserverInfo {
    const TYPE_ID: u16 = 0x1F;
}

impl DataTypeEntity for ArenaObserverInfo {
    type ClientMethod = ArenaObserverInfo_Client;
    type BaseMethod = ArenaObserverInfo_Base;
    type CellMethod = ArenaObserverInfo_Cell;
}

// Entity 0x20
// Interface AreaOfEffect
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AreaOfEffect {
        pub vehicleID: i32,
        pub equipmentID: i32,
        pub launchTime: f64,
        pub strikeTime: f64,
    }
}

// Method for AreaOfEffect on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct AreaOfEffect_playEffect {
        pub a0: RelaxString,
        pub a1: Vec3,
        pub a2: f32,
    }

}

// Method for AreaOfEffect on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for AreaOfEffect on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for AreaOfEffect on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum AreaOfEffect_Client {
        AreaOfEffect_playEffect(0x00, var8),
    }
}

// Entity methods for AreaOfEffect on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum AreaOfEffect_Base {
    }
}

// Entity methods for AreaOfEffect on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum AreaOfEffect_Cell {
    }
}

impl AreaOfEffect {
    const TYPE_ID: u16 = 0x20;
}

impl DataTypeEntity for AreaOfEffect {
    type ClientMethod = AreaOfEffect_Client;
    type BaseMethod = AreaOfEffect_Base;
    type CellMethod = AreaOfEffect_Cell;
}

// Entity 0x21
// Interface AttackBomber
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AttackBomber {
    }
}

// Method for AttackBomber on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for AttackBomber on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for AttackBomber on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for AttackBomber on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum AttackBomber_Client {
    }
}

// Entity methods for AttackBomber on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum AttackBomber_Base {
    }
}

// Entity methods for AttackBomber on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum AttackBomber_Cell {
    }
}

impl AttackBomber {
    const TYPE_ID: u16 = 0x21;
}

impl DataTypeEntity for AttackBomber {
    type ClientMethod = AttackBomber_Client;
    type BaseMethod = AttackBomber_Base;
    type CellMethod = AttackBomber_Cell;
}

// Entity 0x22
// Interface AttackArtilleryFort
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct AttackArtilleryFort {
        pub team: i32,
    }
}

// Method for AttackArtilleryFort on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for AttackArtilleryFort on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for AttackArtilleryFort on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for AttackArtilleryFort on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum AttackArtilleryFort_Client {
    }
}

// Entity methods for AttackArtilleryFort on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum AttackArtilleryFort_Base {
    }
}

// Entity methods for AttackArtilleryFort on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum AttackArtilleryFort_Cell {
    }
}

impl AttackArtilleryFort {
    const TYPE_ID: u16 = 0x22;
}

impl DataTypeEntity for AttackArtilleryFort {
    type ClientMethod = AttackArtilleryFort_Client;
    type BaseMethod = AttackArtilleryFort_Base;
    type CellMethod = AttackArtilleryFort_Cell;
}

// Entity 0x23
// Interface PersonalDeathZone
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct PersonalDeathZone {
    }
}

// Method for PersonalDeathZone on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for PersonalDeathZone on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for PersonalDeathZone on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for PersonalDeathZone on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum PersonalDeathZone_Client {
    }
}

// Entity methods for PersonalDeathZone on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum PersonalDeathZone_Base {
    }
}

// Entity methods for PersonalDeathZone on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum PersonalDeathZone_Cell {
    }
}

impl PersonalDeathZone {
    const TYPE_ID: u16 = 0x23;
}

impl DataTypeEntity for PersonalDeathZone {
    type ClientMethod = PersonalDeathZone_Client;
    type BaseMethod = PersonalDeathZone_Base;
    type CellMethod = PersonalDeathZone_Cell;
}

// Entity 0x24
// Interface ClientSelectableRankedObject
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct ClientSelectableRankedObject {
    }
}

// Method for ClientSelectableRankedObject on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for ClientSelectableRankedObject on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for ClientSelectableRankedObject on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for ClientSelectableRankedObject on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ClientSelectableRankedObject_Client {
    }
}

// Entity methods for ClientSelectableRankedObject on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ClientSelectableRankedObject_Base {
    }
}

// Entity methods for ClientSelectableRankedObject on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ClientSelectableRankedObject_Cell {
    }
}

impl ClientSelectableRankedObject {
    const TYPE_ID: u16 = 0x24;
}

impl DataTypeEntity for ClientSelectableRankedObject {
    type ClientMethod = ClientSelectableRankedObject_Client;
    type BaseMethod = ClientSelectableRankedObject_Base;
    type CellMethod = ClientSelectableRankedObject_Cell;
}

// Entity 0x25
// Interface SimulatedVehicle
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct SimulatedVehicle {
        pub publicInfo: PUBLIC_VEHICLE_INFO,
        pub isPlayerVehicle: BOOL,
        pub realVehicleID: OBJECT_ID,
        pub simulationData_position: Vec3,
        pub simulationData_rotation: Vec3,
        pub simulationData_velocity: Vec3,
        pub simulationData_angVelocity: Vec3,
        pub simulationData_simulationType: RelaxString,
        pub simulationData_health: i16,
        pub simulationData_engineMode: Box<[u8; 2]>,
        pub simulationData_gunAngles: Vec2,
        pub simulationData_turretAndGunSpeed: Vec2,
        pub simulationData_damageStickers: Vec<u64>,
        pub simulationData_brokenTracks: Vec<TRACK_STATE>,
        pub simulationData_siegeState: BOOL,
        pub simulationData_wheelsState: u16,
        pub simulationData_wheelsSteering: Vec<f32>,
        pub simulationData_tracksInAir: Box<[BOOL; 2]>,
    }
}

// Method for SimulatedVehicle on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for SimulatedVehicle on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for SimulatedVehicle on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for SimulatedVehicle on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum SimulatedVehicle_Client {
    }
}

// Entity methods for SimulatedVehicle on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum SimulatedVehicle_Base {
    }
}

// Entity methods for SimulatedVehicle on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum SimulatedVehicle_Cell {
    }
}

impl SimulatedVehicle {
    const TYPE_ID: u16 = 0x25;
}

impl DataTypeEntity for SimulatedVehicle {
    type ClientMethod = SimulatedVehicle_Client;
    type BaseMethod = SimulatedVehicle_Base;
    type CellMethod = SimulatedVehicle_Cell;
}

// Entity 0x26
// Interface ClientSelectableHangarsSwitcher
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct ClientSelectableHangarsSwitcher {
        pub destHangar: RelaxString,
    }
}

// Method for ClientSelectableHangarsSwitcher on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for ClientSelectableHangarsSwitcher on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for ClientSelectableHangarsSwitcher on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for ClientSelectableHangarsSwitcher on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ClientSelectableHangarsSwitcher_Client {
    }
}

// Entity methods for ClientSelectableHangarsSwitcher on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ClientSelectableHangarsSwitcher_Base {
    }
}

// Entity methods for ClientSelectableHangarsSwitcher on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ClientSelectableHangarsSwitcher_Cell {
    }
}

impl ClientSelectableHangarsSwitcher {
    const TYPE_ID: u16 = 0x26;
}

impl DataTypeEntity for ClientSelectableHangarsSwitcher {
    type ClientMethod = ClientSelectableHangarsSwitcher_Client;
    type BaseMethod = ClientSelectableHangarsSwitcher_Base;
    type CellMethod = ClientSelectableHangarsSwitcher_Cell;
}

// Entity 0x27
// Interface StaticDeathZone
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct StaticDeathZone {
        pub zoneId: RelaxString,
        pub isActive: BOOL,
        pub vehiclesUnderFire: Vec<VEHICLE_IN_DEATHZONE>,
        pub maskingPolygonsCount: u8,
        pub proximityMarkerStyle: RelaxString,
    }
}

// Method for StaticDeathZone on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct StaticDeathZone_onDeathZoneDamage {
        pub a0: OBJECT_ID,
        pub a1: RelaxString,
    }

    #[derive(Debug)]
    pub struct StaticDeathZone_onDeathZoneNotification {
        pub a0: BOOL,
        pub a1: OBJECT_ID,
        pub a2: f32,
        pub a3: f32,
    }

    #[derive(Debug)]
    pub struct StaticDeathZone_onEntityEnteredInZone {
        pub a0: OBJECT_ID,
    }

    #[derive(Debug)]
    pub struct StaticDeathZone_onEntityLeftZone {
        pub a0: OBJECT_ID,
    }

}

// Method for StaticDeathZone on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for StaticDeathZone on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for StaticDeathZone on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum StaticDeathZone_Client {
        StaticDeathZone_onEntityEnteredInZone(0x00, 4),
        StaticDeathZone_onEntityLeftZone(0x01, 4),
        StaticDeathZone_onDeathZoneNotification(0x02, 13),
        StaticDeathZone_onDeathZoneDamage(0x03, var8),
    }
}

// Entity methods for StaticDeathZone on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum StaticDeathZone_Base {
    }
}

// Entity methods for StaticDeathZone on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum StaticDeathZone_Cell {
    }
}

impl StaticDeathZone {
    const TYPE_ID: u16 = 0x27;
}

impl DataTypeEntity for StaticDeathZone {
    type ClientMethod = StaticDeathZone_Client;
    type BaseMethod = StaticDeathZone_Base;
    type CellMethod = StaticDeathZone_Cell;
}

// Entity 0x28
// Interface BasicMine
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct BasicMine {
        pub equipmentID: u32,
        pub ownerVehicleID: u32,
        pub isDetonated: BOOL,
        pub isActivated: BOOL,
        pub activationTimeDelay: u32,
        pub mineNumber: u8,
        pub isMarkerEnabled: BOOL,
    }
}

// Method for BasicMine on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for BasicMine on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for BasicMine on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for BasicMine on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum BasicMine_Client {
    }
}

// Entity methods for BasicMine on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum BasicMine_Base {
    }
}

// Entity methods for BasicMine on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum BasicMine_Cell {
    }
}

impl BasicMine {
    const TYPE_ID: u16 = 0x28;
}

impl DataTypeEntity for BasicMine {
    type ClientMethod = BasicMine_Client;
    type BaseMethod = BasicMine_Base;
    type CellMethod = BasicMine_Cell;
}

// Entity 0x29
// Interface ApplicationPoint
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct ApplicationPoint {
        pub vehicleID: i32,
        pub equipmentID: i32,
        pub launchTime: f32,
        pub level: i32,
    }
}

// Method for ApplicationPoint on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for ApplicationPoint on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for ApplicationPoint on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for ApplicationPoint on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ApplicationPoint_Client {
    }
}

// Entity methods for ApplicationPoint on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ApplicationPoint_Base {
    }
}

// Entity methods for ApplicationPoint on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum ApplicationPoint_Cell {
    }
}

impl ApplicationPoint {
    const TYPE_ID: u16 = 0x29;
}

impl DataTypeEntity for ApplicationPoint {
    type ClientMethod = ApplicationPoint_Client;
    type BaseMethod = ApplicationPoint_Base;
    type CellMethod = ApplicationPoint_Cell;
}

// Entity 0x2A
// Interface NetworkEntity
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct NetworkEntity {
        pub unique_id: RelaxString,
        pub prefab_path: RelaxString,
        pub scale: Vec3,
        pub goState: Vec<GAME_OBJECT_STATE>,
        pub name: RelaxString,
    }
}

// Method for NetworkEntity on client
wgtk::__bootstrap_struct_data_type! {

    #[derive(Debug)]
    pub struct NetworkEntity_activateGameObject {
    }

    #[derive(Debug)]
    pub struct NetworkEntity_activateGameObjectUnique {
    }

    #[derive(Debug)]
    pub struct NetworkEntity_deactivateGameObject {
    }

    #[derive(Debug)]
    pub struct NetworkEntity_deactivateGameObjectUnique {
    }

    #[derive(Debug)]
    pub struct NetworkEntity_createGameObject {
    }

    #[derive(Debug)]
    pub struct NetworkEntity_removeGameObject {
    }

    #[derive(Debug)]
    pub struct NetworkEntity_removeGameObjectUnique {
    }

}

// Method for NetworkEntity on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for NetworkEntity on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for NetworkEntity on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum NetworkEntity_Client {
        NetworkEntity_activateGameObject(0x00, 0),
        NetworkEntity_activateGameObjectUnique(0x01, 0),
        NetworkEntity_deactivateGameObject(0x02, 0),
        NetworkEntity_deactivateGameObjectUnique(0x03, 0),
        NetworkEntity_createGameObject(0x04, 0),
        NetworkEntity_removeGameObject(0x05, 0),
        NetworkEntity_removeGameObjectUnique(0x06, 0),
    }
}

// Entity methods for NetworkEntity on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum NetworkEntity_Base {
    }
}

// Entity methods for NetworkEntity on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum NetworkEntity_Cell {
    }
}

impl NetworkEntity {
    const TYPE_ID: u16 = 0x2A;
}

impl DataTypeEntity for NetworkEntity {
    type ClientMethod = NetworkEntity_Client;
    type BaseMethod = NetworkEntity_Base;
    type CellMethod = NetworkEntity_Cell;
}

// Entity 0x2B
// Interface Comp7Lighting
wgtk::__bootstrap_struct_data_type! {
    #[derive(Debug)]
    pub struct Comp7Lighting {
        pub animationStateMachine: RelaxString,
    }
}

// Method for Comp7Lighting on client
wgtk::__bootstrap_struct_data_type! {

}

// Method for Comp7Lighting on base
wgtk::__bootstrap_struct_data_type! {

}

// Method for Comp7Lighting on cell
wgtk::__bootstrap_struct_data_type! {

}

// Entity methods for Comp7Lighting on client
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum Comp7Lighting_Client {
    }
}

// Entity methods for Comp7Lighting on base
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum Comp7Lighting_Base {
    }
}

// Entity methods for Comp7Lighting on cell
wgtk::__bootstrap_enum_methods! {
    #[derive(Debug)]
    pub enum Comp7Lighting_Cell {
    }
}

impl Comp7Lighting {
    const TYPE_ID: u16 = 0x2B;
}

impl DataTypeEntity for Comp7Lighting {
    type ClientMethod = Comp7Lighting_Client;
    type BaseMethod = Comp7Lighting_Base;
    type CellMethod = Comp7Lighting_Cell;
}

