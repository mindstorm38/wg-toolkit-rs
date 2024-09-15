#![allow(non_camel_case_types, non_snake_case)]

use crate::aliases::*;

// Wheels

pub enum Wheels_Client { }

pub enum Wheels_Base { }

pub enum Wheels_Cell { }

// VehiclesSpawnListStorage_Avatar

pub enum VehiclesSpawnListStorage_Avatar_Client { 
    updateSpawnList(String, ),
}

pub enum VehiclesSpawnListStorage_Avatar_Base { 
    vehiclesSpawnListStorage_updateSpawnList(String, ),
}

pub enum VehiclesSpawnListStorage_Avatar_Cell { }

// VehicleRemovalController_Avatar

pub enum VehicleRemovalController_Avatar_Client { 
    removeVehicle(i32, ),
}

pub enum VehicleRemovalController_Avatar_Base { }

pub enum VehicleRemovalController_Avatar_Cell { }

// VehicleObserver

pub enum VehicleObserver_Client { }

pub enum VehicleObserver_Base { }

pub enum VehicleObserver_Cell { 
    setRemoteCamera(REMOTE_CAMERA_DATA, ),
}

// VehicleHealthBroadcastListenerComponent_Avatar

pub enum VehicleHealthBroadcastListenerComponent_Avatar_Client { 
    onVehicleHealthChanged(i32, i16, i32, u8, ),
}

pub enum VehicleHealthBroadcastListenerComponent_Avatar_Base { }

pub enum VehicleHealthBroadcastListenerComponent_Avatar_Cell { }

// VehicleAIProxy

pub enum VehicleAIProxy_Client { }

pub enum VehicleAIProxy_Base { }

pub enum VehicleAIProxy_Cell { }

// TriggersController_Avatar

pub enum TriggersController_Avatar_Client { 
    externalTrigger(String, Python, ),
}

pub enum TriggersController_Avatar_Base { }

pub enum TriggersController_Avatar_Cell { }

// TransactionUser

pub enum TransactionUser_Client { }

pub enum TransactionUser_Base { 
    onTransactionMessage(Mailbox, u8, Python, ),
}

pub enum TransactionUser_Cell { }

// ThrottledMethods

pub enum ThrottledMethods_Client { }

pub enum ThrottledMethods_Base { }

pub enum ThrottledMethods_Cell { }

// TeamHealthBar_Avatar

pub enum TeamHealthBar_Avatar_Client { 
    updateTeamsHealthPercentage(Vec<u8>, ),
}

pub enum TeamHealthBar_Avatar_Base { 
    teamHealthBar_updateTeamsHealthPercentage(Vec<u8>, ),
}

pub enum TeamHealthBar_Avatar_Cell { }

// TeamBase_Vehicle

pub enum TeamBase_Vehicle_Client { }

pub enum TeamBase_Vehicle_Base { }

pub enum TeamBase_Vehicle_Cell { 
    teamBase_receivePoints(u8, f32, ),
    teamBase_onCaptured(u8, f32, u8, ),
}

// StepRepairPoint_Vehicle

pub enum StepRepairPoint_Vehicle_Client { }

pub enum StepRepairPoint_Vehicle_Base { }

pub enum StepRepairPoint_Vehicle_Cell { 
    stepRepair_onAction(i32, u8, f32, ),
}

// SmokeController_Vehicle

pub enum SmokeController_Vehicle_Client { }

pub enum SmokeController_Vehicle_Base { }

pub enum SmokeController_Vehicle_Cell { 
    smokeController_onEnterSmoke(u32, f32, ),
    smokeController_onLeaveSmoke(u32, ),
}

// SessionTracker

pub enum SessionTracker_Client { }

pub enum SessionTracker_Base { 
    sessionTracker_processSessionTrackerData(Mailbox, i32, u8, String, ),
}

pub enum SessionTracker_Cell { }

// ServerSideReplays

pub enum ServerSideReplays_Client { }

pub enum ServerSideReplays_Base { }

pub enum ServerSideReplays_Cell { }

// Sector_Vehicle

pub enum Sector_Vehicle_Client { }

pub enum Sector_Vehicle_Base { }

pub enum Sector_Vehicle_Cell { 
    sector_onEnterSector(i32, u8, u8, u8, u8, f32, f32, u8, ),
    sector_onLeaveSector(u8, u8, ),
    sector_onSectorShooting(u8, ),
}

// SectorBase_Vehicle

pub enum SectorBase_Vehicle_Client { }

pub enum SectorBase_Vehicle_Base { }

pub enum SectorBase_Vehicle_Cell { 
    sectorBase_receivePoints(u8, f32, ),
    sectorBase_onCaptured(u8, f32, u8, ),
    sectorBase_onAction(u8, u8, f32, ),
    sectorBase_onCapturePointsBlocked(u8, f32, ),
    sectorBase_onCapturePointsBlockedComplete(u8, ),
}

// RespawnController_Vehicle

pub enum RespawnController_Vehicle_Client { }

pub enum RespawnController_Vehicle_Base { }

pub enum RespawnController_Vehicle_Cell { 
    respawnController_respawn(RESPAWN_INFO_VEHICLE, Vec<Mailbox>, ),
}

// RespawnController_Avatar

pub enum RespawnController_Avatar_Client { 
    redrawVehicleOnRespawn(i32, String, String, ),
    explodeVehicleBeforeRespawn(i32, ),
    updateRespawnVehicles(Vec<RESPAWN_AVAILABLE_VEHICLE>, ),
    updateRespawnCooldowns(Vec<RESPAWN_COOLDOWN_ITEM>, ),
    updateRespawnInfo(RESPAWN_INFO, ),
    updateVehicleLimits(Vec<RESPAWN_LIMITED_VEHICLES>, ),
    updatePlayerLives(u8, ),
    onTeamLivesRestored(Vec<u8>, ),
}

pub enum RespawnController_Avatar_Base { 
    respawnController_requestRespawnGroupChange(u8, ),
    respawnController_chooseVehicleForRespawn(u16, ),
    respawnController_performRespawn(),
    respawnController_redrawVehicleOnRespawn(i32, String, String, ),
    respawnController_updateRespawnVehicles(Vec<RESPAWN_AVAILABLE_VEHICLE>, ),
    respawnController_updateRespawnCooldowns(Vec<RESPAWN_COOLDOWN_ITEM>, ),
    respawnController_updateRespawnInfo(RESPAWN_INFO, ),
    respawnController_updateVehicleLimits(Vec<RESPAWN_LIMITED_VEHICLES>, ),
    respawnController_onTeamLivesRestored(Vec<u8>, ),
    respawnController_updatePlayerLives(u8, ),
    respawnController_chooseRespawnZone(Vector3, ),
    respawnController_switchSetup(u16, u8, u8, ),
}

pub enum RespawnController_Avatar_Cell { }

// RepairBase_Vehicle

pub enum RepairBase_Vehicle_Client { }

pub enum RepairBase_Vehicle_Base { }

pub enum RepairBase_Vehicle_Cell { 
    repairBase_onAction(u8, u8, f32, ),
}

// RecoveryMechanic_Vehicle

pub enum RecoveryMechanic_Vehicle_Client { }

pub enum RecoveryMechanic_Vehicle_Base { }

pub enum RecoveryMechanic_Vehicle_Cell { 
    recoveryMechanic_startRecovering(),
    recoveryMechanic_stopRecovering(),
}

// RecoveryMechanic_Avatar

pub enum RecoveryMechanic_Avatar_Client { 
    updateState(u8, i32, i32, f32, ),
}

pub enum RecoveryMechanic_Avatar_Base { 
    recoveryMechanic_updateState(u8, i32, i32, f32, ),
}

pub enum RecoveryMechanic_Avatar_Cell { }

// QuestProcessor

pub enum QuestProcessor_Client { }

pub enum QuestProcessor_Base { 
    questProcessor_handleEvent(i32, Python, ),
}

pub enum QuestProcessor_Cell { }

// ProtectionZone_Vehicle

pub enum ProtectionZone_Vehicle_Client { }

pub enum ProtectionZone_Vehicle_Base { }

pub enum ProtectionZone_Vehicle_Cell { 
    protectionZone_onEnterProtectionZone(u8, ),
    protectionZone_onLeaveProtectionZone(u8, ),
    protectionZone_onProtectionZoneShooting(u8, ),
}

// ProtectionZoneController_Avatar

pub enum ProtectionZoneController_Avatar_Client { }

pub enum ProtectionZoneController_Avatar_Base { 
    protectionZone_enteringProtectionZone(u8, ),
    protectionZone_leavingProtectionZone(u8, ),
    protectionZone_protectionZoneShooting(u8, ),
}

pub enum ProtectionZoneController_Avatar_Cell { }

// PlayerMessenger_chat2

pub enum PlayerMessenger_chat2_Client { 
    messenger_onActionByServer_chat2(i16, u16, GENERIC_MESSENGER_ARGS_chat2, ),
}

pub enum PlayerMessenger_chat2_Base { 
    messenger_onActionByServer_chat2(Mailbox, i16, i32, GENERIC_MESSENGER_ARGS_chat2, ),
    messenger_onActionByClient_chat2(i16, u16, GENERIC_MESSENGER_ARGS_chat2, ),
    messenger_onActionForClient_chat2(Mailbox, i16, i32, GENERIC_MESSENGER_ARGS_chat2, ),
}

pub enum PlayerMessenger_chat2_Cell { }

// PlayLimits

pub enum PlayLimits_Client { }

pub enum PlayLimits_Base { 
    playLimits_submitReadinessValidation(i32, ),
}

pub enum PlayLimits_Cell { }

// PlaneTrajectoryArenaInfo

pub enum PlaneTrajectoryArenaInfo_Client { }

pub enum PlaneTrajectoryArenaInfo_Base { }

pub enum PlaneTrajectoryArenaInfo_Cell { }

// Perks_Vehicle

pub enum Perks_Vehicle_Client { }

pub enum Perks_Vehicle_Base { }

pub enum Perks_Vehicle_Cell { }

// Invoicing

pub enum Invoicing_Client { }

pub enum Invoicing_Base { }

pub enum Invoicing_Cell { }

// InvitationsClient

pub enum InvitationsClient_Client { 
    processInvitations(Python, ),
}

pub enum InvitationsClient_Base { }

pub enum InvitationsClient_Cell { }

// Invitations

pub enum Invitations_Client { }

pub enum Invitations_Base { 
    invitations_syncInvitationState(i8, Python, ),
}

pub enum Invitations_Cell { }

// InterclusterSender

pub enum InterclusterSender_Client { }

pub enum InterclusterSender_Base { 
    acknowledgeInterclusterRequest(i64, ),
}

pub enum InterclusterSender_Cell { }

// Harm

pub enum Harm_Client { }

pub enum Harm_Base { }

pub enum Harm_Cell { 
    harm_receiveShot(ATTACKER_INFO, i32, i32, u8, f32, Vector3, Vector3, Box<[f32; 2]>, Vec<Box<[f32; 4]>>, f32, Vec<f32>, f32, ),
    harm_receiveExplosion(ATTACKER_INFO, i32, i32, Vector3, f32, f32, f32, f32, u8, f32, ),
    harm_receivePressure(u8, i32, f32, ),
    harm_receiveRamming(i32, u8, Vec<f32>, f32, Vec<u8>, f32, f32, f32, u8, ),
    harm_receiveMiss(i32, u16, ),
    harm_receiveCeilingHit(i32, Vector3, ),
    harm_receiveAttackResults(ATTACK_RESULTS, ),
    harm_onCombatEquipmentShootingStarted(u16, Vector3, f32, Vec<i32>, ),
}

// EntityTrap

pub enum EntityTrap_Client { }

pub enum EntityTrap_Base { }

pub enum EntityTrap_Cell { }

// DestructibleEntity_Vehicle

pub enum DestructibleEntity_Vehicle_Client { }

pub enum DestructibleEntity_Vehicle_Base { }

pub enum DestructibleEntity_Vehicle_Cell { 
    destructibleEntity_showShotResults(u8, Vec<u32>, ),
    destructibleEntity_onDamaged(u8, u32, u8, ),
    destructibleEntity_onDestroyed(u8, i32, u8, ),
    destructibleEntity_onDefended(u8, ),
}

// DestructibleEntity_Avatar

pub enum DestructibleEntity_Avatar_Client { }

pub enum DestructibleEntity_Avatar_Base { 
    destructibleEntity_onDestroyed(u8, i32, ),
}

pub enum DestructibleEntity_Avatar_Cell { }

// Destructible

pub enum Destructible_Client { }

pub enum Destructible_Base { }

pub enum Destructible_Cell { 
    receiveAndTakeOverProjectile(u8, u8, Vector3, Vector3, Vector3, f32, f32, ATTACKER_INFO, i32, i32, u8, u8, Vec<Mailbox>, Box<[f32; 2]>, f32, f32, f64, Vector3, Vector3, f32, f32, f32, f32, f64, ),
    receiveExplosion(Vector3, f32, f32, Vec<Box<[f32; 3]>>, ATTACKER_INFO, i32, i32, ),
    receiveRamming(u16, u8, f32, f32, ATTACKER_INFO, ),
}

// DefenderBonusController_Vehicle

pub enum DefenderBonusController_Vehicle_Client { }

pub enum DefenderBonusController_Vehicle_Base { }

pub enum DefenderBonusController_Vehicle_Cell { }

// ControlPoint

pub enum ControlPoint_Client { }

pub enum ControlPoint_Base { }

pub enum ControlPoint_Cell { }

// ClientCommandsPort

pub enum ClientCommandsPort_Client { 
    onCmdResponse(i16, i16, String, ),
    onCmdResponseExt(i16, i16, String, String, ),
}

pub enum ClientCommandsPort_Base { 
    doCmdNoArgs(i16, i16, ),
    doCmdStr(i16, i16, String, ),
    doCmdInt(i16, i16, i64, ),
    doCmdInt2(i16, i16, i64, i64, ),
    doCmdInt3(i16, i16, i64, i64, i64, ),
    doCmdInt4(i16, i16, i64, i64, i32, i32, ),
    doCmdInt2Str(i16, i16, i64, i64, String, ),
    doCmdInt3Str(i16, i16, i64, i64, i64, String, ),
    doCmdIntArr(i16, i16, Vec<i32>, ),
    doCmdIntStr(i16, i16, i64, String, ),
    doCmdIntStrArr(i16, i16, i64, Vec<String>, ),
    doCmdIntArrStrArr(i16, i16, Vec<i64>, Vec<String>, ),
    doCmdStrArr(i16, i16, Vec<String>, ),
}

pub enum ClientCommandsPort_Cell { }

// Chat

pub enum Chat_Client { 
    onChatAction(CHAT_ACTION_DATA, ),
}

pub enum Chat_Base { 
    joinChatChannel(i32, String, ),
    leaveChatChannel(i32, ),
    onChatAction(CHAT_ACTION_DATA, ),
    onUserChatChannelCreated(Mailbox, Python, ),
    chatCommandFromClient(i64, u8, i32, i64, i16, String, String, ),
    chatCommand(Mailbox, i64, u8, i32, i64, i16, String, String, ),
    inviteCommand(i64, u8, i8, String, i64, i16, String, String, ),
    ackCommand(i64, u8, f64, i64, i64, ),
    keepAlive(i32, i16, ),
    keepAlive(i32, i16, ),
    onStreamComplete(i16, u8, ),
    setState(u8, ),
    setCollectOnlyMode(u8, ),
    setBattleChannels(Python, ),
    changeMemberStatus(Python, i32, ),
    setWorkingState(u8, ),
    streamStringToClient(String, String, i16, ),
}

pub enum Chat_Cell { }

// BattleResultProcessor

pub enum BattleResultProcessor_Client { }

pub enum BattleResultProcessor_Base { 
    battleResultProcessor_onBattleResultsReceived(i32, String, ),
    battleResultProcessor_onPlayerLeftArena(i32, i32, String, u64, Python, Python, i32, i64, ),
}

pub enum BattleResultProcessor_Cell { }

// BattleFeedback

pub enum BattleFeedback_Client { }

pub enum BattleFeedback_Base { }

pub enum BattleFeedback_Cell { 
    battleFeedback_onBattleEvent(u8, i32, u32, u8, ),
    battleFeedback_onDeath(i32, u8, ),
}

// AvatarObserver

pub enum AvatarObserver_Client { }

pub enum AvatarObserver_Base { }

pub enum AvatarObserver_Cell { 
    switchObserverFPV(u8, ),
    setRemoteCamera(REMOTE_CAMERA_DATA, ),
    setNumOfObservers(u8, ),
}

// AvatarEpic

pub enum AvatarEpic_Client { 
    welcomeToSector(u8, u8, u8, u8, f32, f32, ),
    onStepRepairPointAction(i32, u8, f32, u16, ),
    onSectorBaseAction(u8, u8, f32, ),
    enteringProtectionZone(u8, ),
    leavingProtectionZone(u8, ),
    protectionZoneShooting(u8, ),
    onSectorShooting(u8, ),
    onXPUpdated(i16, ),
    onCrewRoleFactorAndRankUpdate(f32, i64, u8, ),
    syncPurchasedAbilities(Vec<i64>, ),
    onRandomReserveOffer(Vec<i32>, Vec<u8>, u8, ),
    onRankUpdate(u8, ),
    showDestructibleShotResults(u8, Vec<u32>, ),
    onDestructibleDestroyed(u8, i32, ),
}

pub enum AvatarEpic_Base { 
    enableFrontLineDevInfo(u8, ),
    onXPUpdated(i16, ),
}

pub enum AvatarEpic_Cell { 
    onXPUpdated(i16, ),
}

// AvatarCreator

pub enum AvatarCreator_Client { }

pub enum AvatarCreator_Base { 
    avatarCreator_createAvatar(Mailbox, Mailbox, i32, u8, Vector3, f32, Python, u8, i32, Vec<i32>, String, ),
}

pub enum AvatarCreator_Cell { }

// AccountVersion

pub enum AccountVersion_Client { }

pub enum AccountVersion_Base { }

pub enum AccountVersion_Cell { }

// AccountUnitRemote

pub enum AccountUnitRemote_Client { }

pub enum AccountUnitRemote_Base { 
    accountUnitRemote_create(u8, i32, i32, i32, String, String, ),
    accountUnitRemote_createEx(u8, i32, i32, Python, ),
    accountUnitRemote_join(u8, i32, u64, i32, ),
    accountUnitRemote_joinEx(u8, i32, i32, Python, ),
    accountUnitRemote_doCmd(u8, i32, i32, i32, u64, i32, String, ),
    accountUnitRemote_sendInvites(u8, i32, u64, Vec<i64>, String, ),
    accountUnitRemote_setRosterSlots(u8, i32, u64, Vec<i32>, Vec<String>, ),
    accountUnitRemote_validateVehicleList(u8, i32, Python, ),
}

pub enum AccountUnitRemote_Cell { }

// AccountUnitClient

pub enum AccountUnitClient_Client { }

pub enum AccountUnitClient_Base { 
    accountUnitClient_create(i32, i32, i32, String, String, ),
    accountUnitClient_join(i32, u64, i32, ),
    accountUnitClient_doCmd(i32, i32, i32, u64, i32, String, ),
    accountUnitClient_sendInvites(i32, u64, Vec<i64>, String, ),
    accountUnitClient_setRosterSlots(i32, u64, Vec<i32>, Vec<String>, ),
    accountUnitClient_createEx(i32, i32, Python, ),
    accountUnitClient_joinEx(i32, i32, Python, ),
}

pub enum AccountUnitClient_Cell { }

// AccountUnitBrowser

pub enum AccountUnitBrowser_Client { }

pub enum AccountUnitBrowser_Base { 
    accountUnitBrowser_subscribe(i16, u8, ),
    accountUnitBrowser_unsubscribe(),
    accountUnitBrowser_recenter(i32, i16, u8, ),
    accountUnitBrowser_doCmd(i32, ),
    accountUnitBrowser_onError(i32, String, ),
    accountUnitBrowser_onResultsSet(String, ),
    accountUnitBrowser_onResultsUpdate(String, ),
}

pub enum AccountUnitBrowser_Cell { }

// AccountUnitAssembler

pub enum AccountUnitAssembler_Client { }

pub enum AccountUnitAssembler_Base { 
    accountUnitAssembler_onNeedToJoinToUnitMgr(Mailbox, i8, ),
    accountUnitAssembler_onAssemblerEnqueued(Mailbox, u8, ),
    accountUnitAssembler_onAssemblerDequeued(Mailbox, u8, ),
}

pub enum AccountUnitAssembler_Cell { }

// AccountUnit

pub enum AccountUnit_Client { }

pub enum AccountUnit_Base { 
    accountUnit_sendSquadInvitations(i64, Vec<i64>, String, u8, Python, Python, ),
    accountUnit_sendUnitUpdate(u64, String, String, ),
    accountUnit_updateUnitProfileVehicle(i32, ),
    accountUnit_onUnitChangedLeader(Mailbox, u8, ),
    accountUnit_onUnitArenaCreated(Mailbox, u64, u8, i32, u8, i32, Python, Python, Python, ),
    accountUnit_onSquadPlayerAdded(u8, ),
    accountUnit_setQueueingTime(i64, ),
    accountUnit_onUnitJoin(i64, Mailbox, i32, Python, ),
    accountUnit_onClearEventMetaGame(Vec<String>, ),
    accountUnit_onSetEventMetaGame(Python, ),
    accountUnit_onUnitLeave(i64, i32, u64, ),
    accountUnit_onUnitCall(i64, i32, u64, String, Python, ),
    accountUnit_onUnitNotify(u64, i32, Python, ),
    accountUnit_sendExternalNotify(i32, String, Python, ),
}

pub enum AccountUnit_Cell { }

// AccountSysMessenger

pub enum AccountSysMessenger_Client { }

pub enum AccountSysMessenger_Base { 
    accountSysMessenger_onSystemMessage(CHAT_ACTION_DATA, ),
}

pub enum AccountSysMessenger_Cell { }

// AccountSpaProcessor

pub enum AccountSpaProcessor_Client { }

pub enum AccountSpaProcessor_Base { 
    accountSPA_processSpaAttributes(Mailbox, i32, i32, Python, ),
}

pub enum AccountSpaProcessor_Cell { }

// AccountPrebattle

pub enum AccountPrebattle_Client { }

pub enum AccountPrebattle_Base { 
    accountPrebattle_onPrebattleJoined(i32, u8, u32, ),
    accountPrebattle_onPrebattleJoinFailure(i32, u8, ),
    accountPrebattle_onPrebattleLeft(i32, ),
    accountPrebattle_onKickedFromPrebattle(i32, u8, ),
    accountPrebattle_onPrebattleResponse(i32, i16, u8, String, i32, ),
    accountPrebattle_onPrebattleVehicleChanged(i32, i8, i32, i32, ),
    accountPrebattle_createTraining(i32, i32, u8, String, ),
    accountPrebattle_createDevPrebattle(u8, u8, i32, i32, String, ),
    accountPrebattle_createExtTrainingPrebattle(i32, String, u8, Python, ),
    accountPrebattle_sendPrebattleInvites(Vec<i64>, String, ),
    accountPrebattle_receivePrebattleRoster(i32, Python, ),
    accountPrebattle_updatePrebattle(i32, u8, String, ),
    accountPrebattle_addPrebattleInvite(i32, i32, PREBATTLE_INVITE, ),
    accountPrebattle_onSendPrebattleInvites(i64, String, u32, i64, String, u64, u8, ),
    accountPrebattle_updateGroup(i32, u8, ),
    accountPrebattle_removeAccountFromWhiteList(i64, ),
    accountPrebattle_setPrebattleVehiclesLimits(Python, u8, ),
    accountPrebattle_setPrebattleBonusType(u32, ),
}

pub enum AccountPrebattle_Cell { }

// AccountIGRProcessing

pub enum AccountIGRProcessing_Client { }

pub enum AccountIGRProcessing_Base { 
    accountIGR_processIGRData(Mailbox, i32, Python, ),
}

pub enum AccountIGRProcessing_Cell { }

// AccountGlobalMapConnector

pub enum AccountGlobalMapConnector_Client { }

pub enum AccountGlobalMapConnector_Base { 
    accountGlobalMapConnector_callGlobalMapMethod(u64, i32, i64, String, ),
    accountGlobalMapConnector_onGlobalMapUpdate(String, String, ),
    accountGlobalMapConnector_onGlobalMapReply(u64, i32, String, ),
    accountGlobalMapConnector_onSpecBattleRoundEnd(String, ),
    accountGlobalMapConnector_onSpecBattleEnd(String, ),
}

pub enum AccountGlobalMapConnector_Cell { }

// AccountEditor

pub enum AccountEditor_Client { }

pub enum AccountEditor_Base { 
    onAccountPropertiesChanged(i32, i32, ),
    receiveProperties(i32, i64, i32, Python, ),
}

pub enum AccountEditor_Cell { }

// AccountDebugger

pub enum AccountDebugger_Client { }

pub enum AccountDebugger_Base { 
    accountDebugger_runDebugTask(String, ),
    accountDebugger_registerDebugTaskResult(i64, i32, i64, ),
    accountDebugger_sendDebugTaskResultChunk(i64, i64, String, ),
}

pub enum AccountDebugger_Cell { }

// AccountClan

pub enum AccountClan_Client { }

pub enum AccountClan_Base { 
    accountClan_createClan(Mailbox, i32, i64, String, String, String, String, i32, i32, u32, ),
    accountClan_enterLeaveClan(Mailbox, i32, i64, i64, u8, i32, u32, ),
    accountClan_processUrgentOps(String, ),
}

pub enum AccountClan_Cell { }

// AccountAvatar

pub enum AccountAvatar_Client { }

pub enum AccountAvatar_Base { 
    accountAvatar_onAvatarCreated(Mailbox, i32, String, ),
    accountAvatar_onAvatarCreationFailure(Mailbox, ),
    accountAvatar_onAvatarLeftArena(Mailbox, u8, ),
    accountAvatar_unlockUnusedVehicles(Vec<i32>, u64, ),
    accountAvatar_sendAccountStats(u32, Vec<String>, ),
    accountAvatar_removeBattleResultFromCache(u64, ),
    accountAvatar_updateUserRelationsOnArena(u64, u64, String, u8, i64, ),
    accountAvatar_delIntUserSettings(Vec<i32>, ),
    accountAvatar_addIntUserSettings(Python, ),
    accountAvatar_setAvatarClient(Python, u8, ),
    accountAvatar_logClientXMPPEvents(Vec<i64>, Vec<String>, ),
    accountAvatar_setActiveVehSeasonInBattle(i64, i64, u8, ),
    accountAvatar_activateGoodieInBattle(i64, u64, u64, u64, ),
}

pub enum AccountAvatar_Cell { }

// AccountAuthTokenProviderClient

pub enum AccountAuthTokenProviderClient_Client { 
    onTokenReceived(u16, u8, String, ),
}

pub enum AccountAuthTokenProviderClient_Base { }

pub enum AccountAuthTokenProviderClient_Cell { }

// AccountAuthTokenProvider

pub enum AccountAuthTokenProvider_Client { }

pub enum AccountAuthTokenProvider_Base { 
    requestToken(u16, u8, ),
    accountAuthTokenProvider_setToken(i64, i32, ),
}

pub enum AccountAuthTokenProvider_Cell { }

// AccountAdmin

pub enum AccountAdmin_Client { }

pub enum AccountAdmin_Base { 
    accountAdmin_addRemoveRareAchievements(Mailbox, i32, Vec<i32>, i32, u32, ),
    accountAdmin_changeFairPlay(Mailbox, i32, String, i32, i32, u32, i32, u32, ),
    accountAdmin_delRestriction(Mailbox, i32, u8, u64, i32, u32, ),
    accountAdmin_excludeFromFairPlay(Mailbox, i32, u8, i32, u32, ),
    accountAdmin_lockVehicleType(Mailbox, i32, Python, Python, i32, u32, ),
    accountAdmin_resetDailyLimits(Mailbox, i32, Vec<u8>, i32, u32, ),
    accountAdmin_resetWalletAssets(Mailbox, i32, i64, i64, u32, i32, u32, ),
    accountAdmin_resetWalletIDs(Mailbox, i32, u64, u64, i32, u32, ),
    accountAdmin_restoreAccountFromPoint(Mailbox, i32, u64, Python, Vec<u64>, u64, u32, u64, u64, i32, u32, Python, Python, ),
    accountAdmin_setAutoBanTime(Mailbox, i32, u32, i32, u32, ),
    accountAdmin_setFinPassword(Mailbox, i32, String, String, u8, i32, u32, ),
    accountAdmin_setLoginPriority(Mailbox, i32, Vec<String>, Vec<String>, i32, u32, ),
    accountAdmin_setNextBanLevel(Mailbox, i32, String, u8, i32, u32, ),
    accountAdmin_setPlayLimits(Mailbox, i32, i32, String, i32, String, i32, u32, ),
    accountAdmin_setRestriction(Mailbox, i32, u8, String, u32, u32, u32, u16, String, i32, u32, ),
    accountAdmin_setType(Mailbox, i32, i16, i16, i32, u32, ),
    accountAdmin_unlockVehicleType(Mailbox, i32, Python, Python, i32, u32, ),
    accountAdmin_wipe(Mailbox, i32, u8, i32, u32, u8, u8, u8, ),
}

pub enum AccountAdmin_Cell { }

