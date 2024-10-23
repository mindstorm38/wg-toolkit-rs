use super::super::alias::*;

/// Methods for Wheels on client component
pub enum WheelsMethod { 
}

/// Methods for VehiclesSpawnListStorage_Avatar on client component
pub enum VehiclesSpawnListStorage_AvatarMethod { 
    updateSpawnList(String), // idx(0)
}

/// Methods for VehicleRemovalController_Avatar on client component
pub enum VehicleRemovalController_AvatarMethod { 
    removeVehicle(OBJECT_ID), // idx(0)
}

/// Methods for VehicleObserver on client component
pub enum VehicleObserverMethod { 
}

/// Methods for VehicleHealthBroadcastListenerComponent_Avatar on client component
pub enum VehicleHealthBroadcastListenerComponent_AvatarMethod { 
    onVehicleHealthChanged(OBJECT_ID, i16, OBJECT_ID, u8), // idx(0)
}

/// Methods for VehicleAIProxy on client component
pub enum VehicleAIProxyMethod { 
}

/// Methods for TriggersController_Avatar on client component
pub enum TriggersController_AvatarMethod { 
    externalTrigger(String, Python), // idx(0)
}

/// Methods for TransactionUser on client component
pub enum TransactionUserMethod { 
}

/// Methods for ThrottledMethods on client component
pub enum ThrottledMethodsMethod { 
}

/// Methods for TeamHealthBar_Avatar on client component
pub enum TeamHealthBar_AvatarMethod { 
    updateTeamsHealthPercentage(Vec<u8>), // idx(0)
}

/// Methods for TeamBase_Vehicle on client component
pub enum TeamBase_VehicleMethod { 
}

/// Methods for StepRepairPoint_Vehicle on client component
pub enum StepRepairPoint_VehicleMethod { 
}

/// Methods for SmokeController_Vehicle on client component
pub enum SmokeController_VehicleMethod { 
}

/// Methods for SessionTracker on client component
pub enum SessionTrackerMethod { 
}

/// Methods for ServerSideReplays on client component
pub enum ServerSideReplaysMethod { 
}

/// Methods for Sector_Vehicle on client component
pub enum Sector_VehicleMethod { 
}

/// Methods for SectorBase_Vehicle on client component
pub enum SectorBase_VehicleMethod { 
}

/// Methods for RespawnController_Vehicle on client component
pub enum RespawnController_VehicleMethod { 
}

/// Methods for RespawnController_Avatar on client component
pub enum RespawnController_AvatarMethod { 
    redrawVehicleOnRespawn(OBJECT_ID, String, String), // idx(0)
    explodeVehicleBeforeRespawn(OBJECT_ID), // idx(1)
    updateRespawnVehicles(Vec<RESPAWN_AVAILABLE_VEHICLE>), // idx(2)
    updateRespawnCooldowns(Vec<RESPAWN_COOLDOWN_ITEM>), // idx(3)
    updateRespawnInfo(RESPAWN_INFO), // idx(4)
    updateVehicleLimits(Vec<RESPAWN_LIMITED_VEHICLES>), // idx(5)
    updatePlayerLives(u8), // idx(6)
    onTeamLivesRestored(Vec<u8>), // idx(7)
}

/// Methods for RepairBase_Vehicle on client component
pub enum RepairBase_VehicleMethod { 
}

/// Methods for RecoveryMechanic_Vehicle on client component
pub enum RecoveryMechanic_VehicleMethod { 
}

/// Methods for RecoveryMechanic_Avatar on client component
pub enum RecoveryMechanic_AvatarMethod { 
    updateState(BOOL, i32, i32, f32), // idx(0)
}

/// Methods for QuestProcessor on client component
pub enum QuestProcessorMethod { 
}

/// Methods for ProtectionZone_Vehicle on client component
pub enum ProtectionZone_VehicleMethod { 
}

/// Methods for ProtectionZoneController_Avatar on client component
pub enum ProtectionZoneController_AvatarMethod { 
}

/// Methods for PlayerMessenger_chat2 on client component
pub enum PlayerMessenger_chat2Method { 
    messenger_onActionByServer_chat2(i16, u16, GENERIC_MESSENGER_ARGS_chat2), // idx(0)
}

/// Methods for PlayLimits on client component
pub enum PlayLimitsMethod { 
}

/// Methods for PlaneTrajectoryArenaInfo on client component
pub enum PlaneTrajectoryArenaInfoMethod { 
}

/// Methods for Perks_Vehicle on client component
pub enum Perks_VehicleMethod { 
}

/// Methods for Invoicing on client component
pub enum InvoicingMethod { 
}

/// Methods for InvitationsClient on client component
pub enum InvitationsClientMethod { 
    processInvitations(Python), // idx(0)
}

/// Methods for Invitations on client component
pub enum InvitationsMethod { 
}

/// Methods for InterclusterSender on client component
pub enum InterclusterSenderMethod { 
}

/// Methods for Harm on client component
pub enum HarmMethod { 
}

/// Methods for EventTokensController on client component
pub enum EventTokensControllerMethod { 
}

/// Methods for EntityTrap on client component
pub enum EntityTrapMethod { 
}

/// Methods for DestructibleEntity_Vehicle on client component
pub enum DestructibleEntity_VehicleMethod { 
}

/// Methods for DestructibleEntity_Avatar on client component
pub enum DestructibleEntity_AvatarMethod { 
}

/// Methods for Destructible on client component
pub enum DestructibleMethod { 
}

/// Methods for DefenderBonusController_Vehicle on client component
pub enum DefenderBonusController_VehicleMethod { 
}

/// Methods for ControlPoint on client component
pub enum ControlPointMethod { 
    EntityTrap(EntityTrapMethod),
}

/// Methods for ClientCommandsPort on client component
pub enum ClientCommandsPortMethod { 
    onCmdResponse(i16, i16, String), // idx(0)
    onCmdResponseExt(i16, i16, String, String), // idx(1)
}

/// Methods for Chat on client component
pub enum ChatMethod { 
    onChatAction(CHAT_ACTION_DATA), // idx(0)
}

/// Methods for BattleResultProcessor on client component
pub enum BattleResultProcessorMethod { 
}

/// Methods for BattleFeedback on client component
pub enum BattleFeedbackMethod { 
}

/// Methods for AvatarObserver on client component
pub enum AvatarObserverMethod { 
}

/// Methods for AvatarEpic on client component
pub enum AvatarEpicMethod { 
    welcomeToSector(u8, u8, u8, BOOL, f32, f32), // idx(0)
    onStepRepairPointAction(OBJECT_ID, u8, f32, u16), // idx(1)
    onSectorBaseAction(u8, u8, f32), // idx(2)
    enteringProtectionZone(u8), // idx(3)
    leavingProtectionZone(u8), // idx(4)
    protectionZoneShooting(u8), // idx(5)
    onSectorShooting(u8), // idx(6)
    onXPUpdated(i16), // idx(7)
    onCrewRoleFactorAndRankUpdate(f32, i64, u8), // idx(8)
    syncPurchasedAbilities(Vec<i64>), // idx(9)
    onRandomReserveOffer(Vec<i32>, Vec<u8>, u8), // idx(10)
    onRankUpdate(u8), // idx(11)
    showDestructibleShotResults(u8, Vec<u32>), // idx(12)
    onDestructibleDestroyed(u8, OBJECT_ID), // idx(13)
}

/// Methods for AvatarCreator on client component
pub enum AvatarCreatorMethod { 
}

/// Methods for AccountVersion on client component
pub enum AccountVersionMethod { 
}

/// Methods for AccountUnitRemote on client component
pub enum AccountUnitRemoteMethod { 
}

/// Methods for AccountUnitClient on client component
pub enum AccountUnitClientMethod { 
}

/// Methods for AccountUnitBrowser on client component
pub enum AccountUnitBrowserMethod { 
}

/// Methods for AccountUnitAssembler on client component
pub enum AccountUnitAssemblerMethod { 
}

/// Methods for AccountUnit on client component
pub enum AccountUnitMethod { 
}

/// Methods for AccountSysMessenger on client component
pub enum AccountSysMessengerMethod { 
}

/// Methods for AccountSpaProcessor on client component
pub enum AccountSpaProcessorMethod { 
}

/// Methods for AccountPrebattle on client component
pub enum AccountPrebattleMethod { 
}

/// Methods for AccountIGRProcessing on client component
pub enum AccountIGRProcessingMethod { 
}

/// Methods for AccountGlobalMapConnector on client component
pub enum AccountGlobalMapConnectorMethod { 
}

/// Methods for AccountEditor on client component
pub enum AccountEditorMethod { 
}

/// Methods for AccountDebugger on client component
pub enum AccountDebuggerMethod { 
}

/// Methods for AccountClan on client component
pub enum AccountClanMethod { 
}

/// Methods for AccountAvatar on client component
pub enum AccountAvatarMethod { 
}

/// Methods for AccountAuthTokenProviderClient on client component
pub enum AccountAuthTokenProviderClientMethod { 
    onTokenReceived(u16, u8, String), // idx(0)
}

/// Methods for AccountAuthTokenProvider on client component
pub enum AccountAuthTokenProviderMethod { 
}

/// Methods for AccountAdmin on client component
pub enum AccountAdminMethod { 
}

