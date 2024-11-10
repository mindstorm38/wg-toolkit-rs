
use super::super::alias::*;
use super::interface::*;

/// Entity 0x01
/// Methods for Account on client component
pub enum AccountMethod { 
    Chat(ChatMethod),
    PlayerMessenger_chat2(PlayerMessenger_chat2Method),
    ClientCommandsPort(ClientCommandsPortMethod),
    AccountAuthTokenProviderClient(AccountAuthTokenProviderClientMethod),
    InvitationsClient(InvitationsClientMethod),
    onKickedFromServer(String, u8, u32), // idx(0)
    onEnqueued(u8), // idx(1)
    onEnqueueFailure(u8, u8, String), // idx(2)
    onDequeued(u8), // idx(3)
    onKickedFromQueue(u8), // idx(4)
    onArenaCreated(), // idx(5)
    onIGRTypeChanged(String), // idx(6)
    onArenaJoinFailure(u8, String), // idx(7)
    onPrebattleJoined(OBJECT_ID), // idx(8)
    onPrebattleJoinFailure(u8), // idx(9)
    onPrebattleLeft(), // idx(10)
    onKickedFromArena(u8), // idx(11)
    onKickedFromPrebattle(u8), // idx(12)
    onCenterIsLongDisconnected(BOOL), // idx(13)
    showGUI(String), // idx(14)
    receiveActiveArenas(Vec<PUBLIC_ARENA_INFO>), // idx(15)
    receiveServerStats(SERVER_STATISTICS), // idx(16)
    receiveQueueInfo(QUEUE_INFO), // idx(17)
    updatePrebattle(u8, String), // idx(18)
    update(String), // idx(19)
    resyncDossiers(BOOL), // idx(20)
    reloadShop(), // idx(21)
    onUnitUpdate(u64, String, String), // idx(22)
    onUnitCallOk(i32), // idx(23)
    onUnitNotify(u64, i32, String, Python), // idx(24)
    onUnitError(i32, u64, i32, String), // idx(25)
    onUnitBrowserError(i32, String), // idx(26)
    onUnitBrowserResultsSet(String), // idx(27)
    onUnitBrowserResultsUpdate(String), // idx(28)
    onGlobalMapUpdate(String, String), // idx(29)
    onGlobalMapReply(u64, i32, String), // idx(30)
    onSendPrebattleInvites(DB_ID, String, DB_ID, String, u64, u8), // idx(31)
    onClanInfoReceived(DB_ID, String, String, String, String), // idx(32)
    receiveNotification(String), // idx(33)
}

// 0: onArenaCreated [Fixed(0)] @ []
// 1: onPrebattleLeft [Fixed(0)] @ []
// 2: reloadShop [Fixed(0)] @ []
// 3: onEnqueued [Fixed(1)] @ []
// 4: onDequeued [Fixed(1)] @ []
// 5: onKickedFromQueue [Fixed(1)] @ []
// 6: onPrebattleJoinFailure [Fixed(1)] @ []
// 7: onKickedFromArena [Fixed(1)] @ []
// 8: onKickedFromPrebattle [Fixed(1)] @ []
// 9: onCenterIsLongDisconnected [Fixed(1)] @ []
// 10: resyncDossiers [Fixed(1)] @ []
// 11: onPrebattleJoined [Fixed(4)] @ []
// 12: onUnitCallOk [Fixed(4)] @ []
// 13: receiveServerStats [Fixed(8)] @ []
// 14: onChatAction [Variable(Variable8)] @ ["Chat"]
// 15: messenger_onActionByServer_chat2 [Variable(Variable8)] @ ["PlayerMessenger_chat2"]
// 16: onCmdResponse [Variable(Variable8)] @ ["ClientCommandsPort"]
// 17: onCmdResponseExt [Variable(Variable8)] @ ["ClientCommandsPort"]
// 18: onTokenReceived [Variable(Variable8)] @ ["AccountAuthTokenProviderClient"]
// 19: processInvitations [Variable(Variable8)] @ ["InvitationsClient"]
// 20: onKickedFromServer [Variable(Variable8)] @ []
// 21: onEnqueueFailure [Variable(Variable8)] @ []
// 22: onIGRTypeChanged [Variable(Variable8)] @ []
// 23: onArenaJoinFailure [Variable(Variable8)] @ []
// 24: receiveActiveArenas [Variable(Variable8)] @ []
// 25: receiveQueueInfo [Variable(Variable8)] @ []
// 26: updatePrebattle [Variable(Variable8)] @ []
// 27: update [Variable(Variable8)] @ []
// 28: onUnitUpdate [Variable(Variable8)] @ []
// 29: onUnitNotify [Variable(Variable8)] @ []
// 30: onUnitError [Variable(Variable8)] @ []
// 31: onUnitBrowserError [Variable(Variable8)] @ []
// 32: onUnitBrowserResultsSet [Variable(Variable8)] @ []
// 33: onUnitBrowserResultsUpdate [Variable(Variable8)] @ []
// 34: onGlobalMapUpdate [Variable(Variable8)] @ []
// 35: onGlobalMapReply [Variable(Variable8)] @ []
// 36: onSendPrebattleInvites [Variable(Variable8)] @ []
// 37: onClanInfoReceived [Variable(Variable8)] @ []
// 38: receiveNotification [Variable(Variable8)] @ []
// 39: showGUI [Variable(Variable16)] @ []

/// Entity 0x02
/// Methods for Avatar on client component
pub enum AvatarMethod { 
    Chat(ChatMethod),
    PlayerMessenger_chat2(PlayerMessenger_chat2Method),
    ClientCommandsPort(ClientCommandsPortMethod),
    InvitationsClient(InvitationsClientMethod),
    AccountAuthTokenProviderClient(AccountAuthTokenProviderClientMethod),
    TeamHealthBar_Avatar(TeamHealthBar_AvatarMethod),
    RecoveryMechanic_Avatar(RecoveryMechanic_AvatarMethod),
    RespawnController_Avatar(RespawnController_AvatarMethod),
    VehiclesSpawnListStorage_Avatar(VehiclesSpawnListStorage_AvatarMethod),
    VehicleRemovalController_Avatar(VehicleRemovalController_AvatarMethod),
    VehicleHealthBroadcastListenerComponent_Avatar(VehicleHealthBroadcastListenerComponent_AvatarMethod),
    TriggersController_Avatar(TriggersController_AvatarMethod),
    AvatarEpic(AvatarEpicMethod),
    update(String), // idx(0)
    onKickedFromServer(String, u8, u32), // idx(1)
    onIGRTypeChanged(String), // idx(2)
    onAutoAimVehicleLost(u8), // idx(3)
    receiveAccountStats(u32, String), // idx(4)
    updateVehicleHealth(OBJECT_ID, i16, i8, BOOL, BOOL), // idx(5)
    updateVehicleAmmo(OBJECT_ID, i32, u16, u8, u8, i16, i16, i16), // idx(6)
    onSwitchViewpoint(OBJECT_ID, Vec3), // idx(7)
    updateVehicleSetting(OBJECT_ID, u8, i32), // idx(8)
    updateTargetingInfo(f32, f32, f32, f32, f32, f32, f32, f32, f32), // idx(9)
    updateTargetVehicleID(OBJECT_ID), // idx(10)
    showOwnVehicleHitDirection(f32, OBJECT_ID, u16, u32, BOOL, BOOL, OBJECT_ID, u8), // idx(11)
    showOtherVehicleDamagedDevices(OBJECT_ID, Vec<EXTRA_ID>, Vec<EXTRA_ID>), // idx(12)
    showShotResults(Vec<u64>), // idx(13)
    showDevelopmentInfo(u8, String), // idx(14)
    showHittingArea(u16, Vec3, Vec3, f64), // idx(15)
    showCarpetBombing(u16, Vec3, Vec3, f64), // idx(16)
    showTracer(OBJECT_ID, SHOT_ID, BOOL, u8, Vec3, Vec3, f32, f32, u8), // idx(17)
    stopTracer(SHOT_ID, Vec3), // idx(18)
    explodeProjectile(SHOT_ID, u8, u8, Vec3, Vec3, Vec<u32>), // idx(19)
    onRoundFinished(i8, u8), // idx(20)
    onKickedFromArena(u8), // idx(21)
    onBattleEvents(Vec<BATTLE_EVENT>), // idx(22)
    battleEventsSummary(BATTLE_EVENTS_SUMMARY), // idx(23)
    updateArena(u8, String), // idx(24)
    updatePositions(Vec<u16>, Vec<i16>), // idx(25)
    receivePhysicsDebugInfo(String), // idx(26)
    updateCarriedFlagPositions(Vec<u8>, Vec<i16>), // idx(27)
    receiveNotification(String), // idx(28)
    onRepairPointAction(u8, u8, f32), // idx(29)
    updateAvatarPrivateStats(String), // idx(30)
    updateResourceAmount(u8, u32), // idx(31)
    onFrictionWithVehicle(OBJECT_ID, Vec3, u8), // idx(32)
    onCollisionWithVehicle(Vec3, f32), // idx(33)
    onSmoke(SMOKE_INFO), // idx(34)
    onCombatEquipmentShotLaunched(u16, Vec3), // idx(35)
    updateQuestProgress(String, Python), // idx(36)
    updateVehicleQuickShellChanger(OBJECT_ID, BOOL), // idx(37)
    enemySPGHit(Vec3), // idx(38)
    enemySPGShotSound(Vec3, Vec3), // idx(39)
    handleScriptEventFromServer(String, String, String, String, String), // idx(40)
    setUpdatedGoodiesSnapshot(Vec<BATTLE_GOODIE_RECORD>), // idx(41)
    onRandomEvent(String), // idx(42)
}

// 0: notifyCannotStartRecovering [Fixed(0)] @ ["RecoveryMechanic_Avatar"]
// 1: notifyCancelled [Fixed(0)] @ ["RecoveryMechanic_Avatar"]
// 2: updatePlayerLives [Fixed(1)] @ ["RespawnController_Avatar"]
// 3: enteringProtectionZone [Fixed(1)] @ ["AvatarEpic"]
// 4: leavingProtectionZone [Fixed(1)] @ ["AvatarEpic"]
// 5: protectionZoneShooting [Fixed(1)] @ ["AvatarEpic"]
// 6: onSectorShooting [Fixed(1)] @ ["AvatarEpic"]
// 7: onRankUpdate [Fixed(1)] @ ["AvatarEpic"]
// 8: onAutoAimVehicleLost [Fixed(1)] @ []
// 9: onKickedFromArena [Fixed(1)] @ []
// 10: onXPUpdated [Fixed(2)] @ ["AvatarEpic"]
// 11: onRoundFinished [Fixed(2)] @ []
// 12: explodeVehicleBeforeRespawn [Fixed(4)] @ ["RespawnController_Avatar"]
// 13: removeVehicle [Fixed(4)] @ ["VehicleRemovalController_Avatar"]
// 14: updateTargetVehicleID [Fixed(4)] @ []
// 15: onDestructibleDestroyed [Fixed(5)] @ ["AvatarEpic"]
// 16: updateResourceAmount [Fixed(5)] @ []
// 17: updateVehicleQuickShellChanger [Fixed(5)] @ []
// 18: onSectorBaseAction [Fixed(6)] @ ["AvatarEpic"]
// 19: onRepairPointAction [Fixed(6)] @ []
// 20: updateVehicleHealth [Fixed(9)] @ []
// 21: updateVehicleSetting [Fixed(9)] @ []
// 22: onStepRepairPointAction [Fixed(11)] @ ["AvatarEpic"]
// 23: onVehicleHealthChanged [Fixed(12)] @ ["VehicleHealthBroadcastListenerComponent_Avatar"]
// 24: welcomeToSector [Fixed(12)] @ ["AvatarEpic"]
// 25: enemySPGHit [Fixed(12)] @ []
// 26: updateState [Fixed(13)] @ ["RecoveryMechanic_Avatar"]
// 27: onCrewRoleFactorAndRankUpdate [Fixed(13)] @ ["AvatarEpic"]
// 28: onCombatEquipmentShotLaunched [Fixed(14)] @ []
// 29: onSwitchViewpoint [Fixed(16)] @ []
// 30: stopTracer [Fixed(16)] @ []
// 31: onCollisionWithVehicle [Fixed(16)] @ []
// 32: onSmoke [Fixed(16)] @ []
// 33: onFrictionWithVehicle [Fixed(17)] @ []
// 34: updateVehicleAmmo [Fixed(18)] @ []
// 35: showOwnVehicleHitDirection [Fixed(21)] @ []
// 36: enemySPGShotSound [Fixed(24)] @ []
// 37: showHittingArea [Fixed(34)] @ []
// 38: showCarpetBombing [Fixed(34)] @ []
// 39: battleEventsSummary [Fixed(34)] @ []
// 40: updateTargetingInfo [Fixed(36)] @ []
// 41: showTracer [Fixed(43)] @ []
// 42: onChatAction [Variable(Variable8)] @ ["Chat"]
// 43: messenger_onActionByServer_chat2 [Variable(Variable8)] @ ["PlayerMessenger_chat2"]
// 44: onCmdResponse [Variable(Variable8)] @ ["ClientCommandsPort"]
// 45: onCmdResponseExt [Variable(Variable8)] @ ["ClientCommandsPort"]
// 46: processInvitations [Variable(Variable8)] @ ["InvitationsClient"]
// 47: onTokenReceived [Variable(Variable8)] @ ["AccountAuthTokenProviderClient"]
// 48: updateTeamsHealthPercentage [Variable(Variable8)] @ ["TeamHealthBar_Avatar"]
// 49: redrawVehicleOnRespawn [Variable(Variable8)] @ ["RespawnController_Avatar"]
// 50: updateRespawnVehicles [Variable(Variable8)] @ ["RespawnController_Avatar"]
// 51: updateRespawnCooldowns [Variable(Variable8)] @ ["RespawnController_Avatar"]
// 52: updateRespawnInfo [Variable(Variable8)] @ ["RespawnController_Avatar"]
// 53: updateVehicleLimits [Variable(Variable8)] @ ["RespawnController_Avatar"]
// 54: onTeamLivesRestored [Variable(Variable8)] @ ["RespawnController_Avatar"]
// 55: externalTrigger [Variable(Variable8)] @ ["TriggersController_Avatar"]
// 56: syncPurchasedAbilities [Variable(Variable8)] @ ["AvatarEpic"]
// 57: onRandomReserveOffer [Variable(Variable8)] @ ["AvatarEpic"]
// 58: showDestructibleShotResults [Variable(Variable8)] @ ["AvatarEpic"]
// 59: update [Variable(Variable8)] @ []
// 60: onKickedFromServer [Variable(Variable8)] @ []
// 61: onIGRTypeChanged [Variable(Variable8)] @ []
// 62: receiveAccountStats [Variable(Variable8)] @ []
// 63: showOtherVehicleDamagedDevices [Variable(Variable8)] @ []
// 64: showShotResults [Variable(Variable8)] @ []
// 65: showDevelopmentInfo [Variable(Variable8)] @ []
// 66: explodeProjectile [Variable(Variable8)] @ []
// 67: onBattleEvents [Variable(Variable8)] @ []
// 68: updateArena [Variable(Variable8)] @ []
// 69: updatePositions [Variable(Variable8)] @ []
// 70: receivePhysicsDebugInfo [Variable(Variable8)] @ []
// 71: updateCarriedFlagPositions [Variable(Variable8)] @ []
// 72: receiveNotification [Variable(Variable8)] @ []
// 73: updateAvatarPrivateStats [Variable(Variable8)] @ []
// 74: updateQuestProgress [Variable(Variable8)] @ []
// 75: handleScriptEventFromServer [Variable(Variable8)] @ []
// 76: setUpdatedGoodiesSnapshot [Variable(Variable8)] @ []
// 77: onRandomEvent [Variable(Variable8)] @ []
// 78: updateSpawnList [Variable(Variable16)] @ ["VehiclesSpawnListStorage_Avatar"]

/// Entity 0x03
/// Methods for ArenaInfo on client component
pub enum ArenaInfoMethod { 
    showCarpetBombing(u16, Vec3, Vec3, f32), // idx(0)
}

// 0: showCarpetBombing [Fixed(30)] @ []

/// Entity 0x04
/// Methods for ClientSelectableObject on client component
pub enum ClientSelectableObjectMethod { 
}


/// Entity 0x05
/// Methods for HangarVehicle on client component
pub enum HangarVehicleMethod { 
}


/// Entity 0x06
/// Methods for Vehicle on client component
pub enum VehicleMethod { 
    onVehiclePickup(), // idx(0)
    onExtraHitted(i16, Vec3), // idx(1)
    onHealthChanged(i16, i16, OBJECT_ID, u8, i8), // idx(2)
    showShooting(u8, i8), // idx(3)
    updateLaserSight(OBJECT_ID, BOOL, String), // idx(4)
    showDamageFromShot(OBJECT_ID, Vec<u64>, u8, i32, u8, BOOL), // idx(5)
    showDamageFromExplosion(OBJECT_ID, Vec3, u8, i32, u8), // idx(6)
    showAmmoBayEffect(u8, f32, f32), // idx(7)
    onPushed(f32, f32), // idx(8)
    onStaticCollision(f32, Vec3, Vec3, u8, f32, i8, u16), // idx(9)
    showRammingEffect(f32, Vec3), // idx(10)
}

// 0: onVehiclePickup [Fixed(0)] @ []
// 1: showShooting [Fixed(2)] @ []
// 2: onPushed [Fixed(8)] @ []
// 3: showAmmoBayEffect [Fixed(9)] @ []
// 4: onHealthChanged [Fixed(10)] @ []
// 5: onExtraHitted [Fixed(14)] @ []
// 6: showRammingEffect [Fixed(16)] @ []
// 7: showDamageFromExplosion [Fixed(22)] @ []
// 8: onStaticCollision [Fixed(36)] @ []
// 9: updateLaserSight [Variable(Variable8)] @ []
// 10: showDamageFromShot [Variable(Variable8)] @ []

/// Entity 0x07
/// Methods for AreaDestructibles on client component
pub enum AreaDestructiblesMethod { 
}


/// Entity 0x08
/// Methods for OfflineEntity on client component
pub enum OfflineEntityMethod { 
}


/// Entity 0x09
/// Methods for Flock on client component
pub enum FlockMethod { 
}


/// Entity 0x0A
/// Methods for FlockExotic on client component
pub enum FlockExoticMethod { 
}


/// Entity 0x0B
/// Methods for Login on client component
pub enum LoginMethod { 
    onKickedFromServer(i32), // idx(0)
    receiveLoginQueueNumber(u64), // idx(1)
    setPeripheryRoutingGroup(String, Python), // idx(2)
}

// 0: onKickedFromServer [Fixed(4)] @ []
// 1: receiveLoginQueueNumber [Fixed(8)] @ []
// 2: setPeripheryRoutingGroup [Variable(Variable8)] @ []

/// Entity 0x0C
/// Methods for DetachedTurret on client component
pub enum DetachedTurretMethod { 
    onStaticCollision(f32, Vec3, Vec3), // idx(0)
    showDamageFromShot(Vec<u64>, u8), // idx(1)
}

// 0: onStaticCollision [Fixed(28)] @ []
// 1: showDamageFromShot [Variable(Variable8)] @ []

/// Entity 0x0D
/// Methods for DebugDrawEntity on client component
pub enum DebugDrawEntityMethod { 
}


/// Entity 0x0E
/// Methods for ClientSelectableCameraObject on client component
pub enum ClientSelectableCameraObjectMethod { 
}


/// Entity 0x0F
/// Methods for ClientSelectableCameraVehicle on client component
pub enum ClientSelectableCameraVehicleMethod { 
}


/// Entity 0x10
/// Methods for ClientSelectableWebLinksOpener on client component
pub enum ClientSelectableWebLinksOpenerMethod { 
}


/// Entity 0x11
/// Methods for ClientSelectableEasterEgg on client component
pub enum ClientSelectableEasterEggMethod { 
}


/// Entity 0x12
/// Methods for EmptyEntity on client component
pub enum EmptyEntityMethod { 
}


/// Entity 0x13
/// Methods for LimitedVisibilityEntity on client component
pub enum LimitedVisibilityEntityMethod { 
}


/// Entity 0x14
/// Methods for HeroTank on client component
pub enum HeroTankMethod { 
}


/// Entity 0x15
/// Methods for PlatoonTank on client component
pub enum PlatoonTankMethod { 
}


/// Entity 0x16
/// Methods for PlatoonLighting on client component
pub enum PlatoonLightingMethod { 
}


/// Entity 0x17
/// Methods for SectorBase on client component
pub enum SectorBaseMethod { 
}


/// Entity 0x18
/// Methods for Sector on client component
pub enum SectorMethod { 
    showBomb(Vec3), // idx(0)
}

// 0: showBomb [Fixed(12)] @ []

/// Entity 0x19
/// Methods for DestructibleEntity on client component
pub enum DestructibleEntityMethod { 
    onHealthChanged(i16, OBJECT_ID, u8, i32), // idx(0)
    showDamageFromShot(OBJECT_ID, u8, i32), // idx(1)
    showDamageFromExplosion(OBJECT_ID, i32), // idx(2)
}

// 0: showDamageFromExplosion [Fixed(8)] @ []
// 1: showDamageFromShot [Fixed(9)] @ []
// 2: onHealthChanged [Fixed(11)] @ []

/// Entity 0x1A
/// Methods for StepRepairPoint on client component
pub enum StepRepairPointMethod { 
}


/// Entity 0x1B
/// Methods for ProtectionZone on client component
pub enum ProtectionZoneMethod { 
}


/// Entity 0x1C
/// Methods for HangarPoster on client component
pub enum HangarPosterMethod { 
}


/// Entity 0x1D
/// Methods for TeamInfo on client component
pub enum TeamInfoMethod { 
    onCombatEquipmentUsed(OBJECT_ID, OBJECT_ID), // idx(0)
    showHittingArea(u16, Vec3, Vec3, f64), // idx(1)
}

// 0: onCombatEquipmentUsed [Fixed(8)] @ []
// 1: showHittingArea [Fixed(34)] @ []

/// Entity 0x1E
/// Methods for AvatarInfo on client component
pub enum AvatarInfoMethod { 
}


/// Entity 0x1F
/// Methods for ArenaObserverInfo on client component
pub enum ArenaObserverInfoMethod { 
}


/// Entity 0x20
/// Methods for AreaOfEffect on client component
pub enum AreaOfEffectMethod { 
    playEffect(String, Vec3, f32), // idx(0)
}

// 0: playEffect [Variable(Variable8)] @ []

/// Entity 0x21
/// Methods for AttackBomber on client component
pub enum AttackBomberMethod { 
}


/// Entity 0x22
/// Methods for AttackArtilleryFort on client component
pub enum AttackArtilleryFortMethod { 
}


/// Entity 0x23
/// Methods for PersonalDeathZone on client component
pub enum PersonalDeathZoneMethod { 
}


/// Entity 0x24
/// Methods for ClientSelectableRankedObject on client component
pub enum ClientSelectableRankedObjectMethod { 
}


/// Entity 0x25
/// Methods for SimulatedVehicle on client component
pub enum SimulatedVehicleMethod { 
}


/// Entity 0x26
/// Methods for ClientSelectableHangarsSwitcher on client component
pub enum ClientSelectableHangarsSwitcherMethod { 
}


/// Entity 0x27
/// Methods for StaticDeathZone on client component
pub enum StaticDeathZoneMethod { 
    onDeathZoneDamage(OBJECT_ID, String), // idx(0)
    onDeathZoneNotification(BOOL, OBJECT_ID, f32, f32), // idx(1)
    onEntityEnteredInZone(OBJECT_ID), // idx(2)
    onEntityLeftZone(OBJECT_ID), // idx(3)
}

// 0: onEntityEnteredInZone [Fixed(4)] @ []
// 1: onEntityLeftZone [Fixed(4)] @ []
// 2: onDeathZoneNotification [Fixed(13)] @ []
// 3: onDeathZoneDamage [Variable(Variable8)] @ []

/// Entity 0x28
/// Methods for BasicMine on client component
pub enum BasicMineMethod { 
}


/// Entity 0x29
/// Methods for ApplicationPoint on client component
pub enum ApplicationPointMethod { 
}


/// Entity 0x2A
/// Methods for NetworkEntity on client component
pub enum NetworkEntityMethod { 
    activateGameObject(), // idx(0)
    activateGameObjectUnique(), // idx(1)
    deactivateGameObject(), // idx(2)
    deactivateGameObjectUnique(), // idx(3)
    createGameObject(), // idx(4)
    removeGameObject(), // idx(5)
    removeGameObjectUnique(), // idx(6)
}

// 0: activateGameObject [Fixed(0)] @ []
// 1: activateGameObjectUnique [Fixed(0)] @ []
// 2: deactivateGameObject [Fixed(0)] @ []
// 3: deactivateGameObjectUnique [Fixed(0)] @ []
// 4: createGameObject [Fixed(0)] @ []
// 5: removeGameObject [Fixed(0)] @ []
// 6: removeGameObjectUnique [Fixed(0)] @ []

/// Entity 0x2B
/// Methods for Comp7Lighting on client component
pub enum Comp7LightingMethod { 
}


