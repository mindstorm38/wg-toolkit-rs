#![allow(non_camel_case_types, non_snake_case)]

use crate::aliases::*;
use crate::interfaces::*;

// Vehicle

pub struct Vehicle {
    isStrafing: u8, // all clients
    postmortemViewPointName: String, // all clients
    isHidden: u8, // all clients
    physicsMode: u8, // all clients
    siegeState: u8, // all clients
    gunAnglesPacked: u16, // all clients
    publicInfo: PUBLIC_VEHICLE_INFO, // all clients
    health: i16, // all clients
    isCrewActive: u8, // all clients
    engineMode: Box<[u8; 2]>, // all clients
    damageStickers: Vec<u64>, // all clients
    publicStateModifiers: Vec<u8>, // all clients
    compDescr: String, // cell public
    stunInfo: f64, // all clients
    status: i8, // cell public
    invisibility: f32, // cell public
    radioDistance: f32, // cell public
    circularVisionRadius: f32, // cell public
    detectedVehicles: Vec<i32>, // cell public
    isObservedByEnemy: u8, // cell public
    rammingBonus: f32, // cell public
    componentsData: Python, // cell public
    ammo: Vec<i32>, // cell private
    crewCompactDescrs: Vec<String>, // all clients
    enhancements: Python, // all clients
    setups: Python, // all clients
    setupsIndexes: Python, // all clients
    customRoleSlotTypeId: u8, // all clients
    vehPerks: Python, // all clients
    vehPostProgression: Vec<i32>, // all clients
    disabledSwitches: Vec<i32>, // all clients
    isClientConnected: i8, // cell private
    avatar: Mailbox, // cell private
    avatarID: i32, // all clients
    arenaBase: Mailbox, // cell private
    botKind: u8, // cell public
    eventBotKind: u8, // cell private
    respawnOffset: Vector3, // cell private
    xRayFactor: f32, // cell private
    foliageInvisibilityFactor: f32, // cell private
    masterVehID: u32, // all clients
    needsCount: u8, // cell private
    arenaTypeID: i32, // own client
    arenaBonusType: u8, // own client
    tkillRating: f32, // cell private
    cp: Python, // cell private
    arenaUniqueID: u64, // own client
    accountDBID: i64, // cell private
    historyLoggingFlags: u64, // cell private
    heatmapLoggingFlags: u32, // cell private
    state: u8, // base
    arena: Mailbox, // base
    inspiringEffect: BUFF_EFFECT, // all clients
    healingEffect: BUFF_EFFECT, // all clients
    dotEffect: DOT_EFFECT, // all clients
    inspired: INSPIRED_EFFECT, // all clients
    healing: BUFF_EFFECT_INACTIVATION, // all clients
    healOverTime: HOT_EFFECT, // all clients
    debuff: i32, // all clients
    isSpeedCapturing: u8, // all clients
    isBlockingCapture: u8, // all clients
    dogTag: BATTLE_DOG_TAG, // all clients
    isMyVehicle: u8, // all clients
    quickShellChangerFactor: f32, // all clients
    onRespawnReloadTimeFactor: f32, // all clients
    ownVehiclePosition: OWN_VEHICLE_POSITION, // all clients
    enableExternalRespawn: u8, // all clients
    botDisplayStatus: u8, // all clients
}

pub enum Vehicle_Client { 
    VehicleAIProxy(VehicleAIProxy_Client),
    TeamBase_Vehicle(TeamBase_Vehicle_Client),
    SectorBase_Vehicle(SectorBase_Vehicle_Client),
    RepairBase_Vehicle(RepairBase_Vehicle_Client),
    VehicleObserver(VehicleObserver_Client),
    BattleFeedback(BattleFeedback_Client),
    Harm(Harm_Client),
    Sector_Vehicle(Sector_Vehicle_Client),
    ProtectionZone_Vehicle(ProtectionZone_Vehicle_Client),
    StepRepairPoint_Vehicle(StepRepairPoint_Vehicle_Client),
    DestructibleEntity_Vehicle(DestructibleEntity_Vehicle_Client),
    DefenderBonusController_Vehicle(DefenderBonusController_Vehicle_Client),
    RecoveryMechanic_Vehicle(RecoveryMechanic_Vehicle_Client),
    RespawnController_Vehicle(RespawnController_Vehicle_Client),
    SmokeController_Vehicle(SmokeController_Vehicle_Client),
    Wheels(Wheels_Client),
    Perks_Vehicle(Perks_Vehicle_Client),
    onExtraHitted(i16, Vector3, ),
    onHealthChanged(i16, i16, i32, u8, ),
    showShooting(u8, i8, ),
    updateLaserSight(i32, u8, String, ),
    showDamageFromShot(i32, Vec<u64>, u8, i32, u8, u8, ),
    showDamageFromExplosion(i32, Vector3, u8, i32, u8, ),
    showAmmoBayEffect(u8, f32, f32, ),
    onPushed(f32, f32, ),
    onStaticCollision(f32, Vector3, Vector3, u8, f32, i8, u16, ),
    showRammingEffect(f32, Vector3, ),
}

pub enum Vehicle_Base { 
    VehicleAIProxy(VehicleAIProxy_Base),
    TeamBase_Vehicle(TeamBase_Vehicle_Base),
    SectorBase_Vehicle(SectorBase_Vehicle_Base),
    RepairBase_Vehicle(RepairBase_Vehicle_Base),
    VehicleObserver(VehicleObserver_Base),
    BattleFeedback(BattleFeedback_Base),
    Harm(Harm_Base),
    Sector_Vehicle(Sector_Vehicle_Base),
    ProtectionZone_Vehicle(ProtectionZone_Vehicle_Base),
    StepRepairPoint_Vehicle(StepRepairPoint_Vehicle_Base),
    DestructibleEntity_Vehicle(DestructibleEntity_Vehicle_Base),
    DefenderBonusController_Vehicle(DefenderBonusController_Vehicle_Base),
    RecoveryMechanic_Vehicle(RecoveryMechanic_Vehicle_Base),
    RespawnController_Vehicle(RespawnController_Vehicle_Base),
    SmokeController_Vehicle(SmokeController_Vehicle_Base),
    Wheels(Wheels_Base),
    Perks_Vehicle(Perks_Vehicle_Base),
    setAvatar(Mailbox, ),
    logAimMetrics(Python, ),
}

pub enum Vehicle_Cell { 
    VehicleAIProxy(VehicleAIProxy_Cell),
    TeamBase_Vehicle(TeamBase_Vehicle_Cell),
    SectorBase_Vehicle(SectorBase_Vehicle_Cell),
    RepairBase_Vehicle(RepairBase_Vehicle_Cell),
    VehicleObserver(VehicleObserver_Cell),
    BattleFeedback(BattleFeedback_Cell),
    Harm(Harm_Cell),
    Sector_Vehicle(Sector_Vehicle_Cell),
    ProtectionZone_Vehicle(ProtectionZone_Vehicle_Cell),
    StepRepairPoint_Vehicle(StepRepairPoint_Vehicle_Cell),
    DestructibleEntity_Vehicle(DestructibleEntity_Vehicle_Cell),
    DefenderBonusController_Vehicle(DefenderBonusController_Vehicle_Cell),
    RecoveryMechanic_Vehicle(RecoveryMechanic_Vehicle_Cell),
    RespawnController_Vehicle(RespawnController_Vehicle_Cell),
    SmokeController_Vehicle(SmokeController_Vehicle_Cell),
    Wheels(Wheels_Cell),
    Perks_Vehicle(Perks_Vehicle_Cell),
    updatePrebattleID(i32, ),
    moveWith(u8, ),
    trackWorldPointWithGun(Vector3, ),
    trackRelativePointWithGun(Vector3, ),
    stopTrackingWithGun(f32, f32, ),
    trackVehicleWithGun(i32, u8, ),
    changeSetting(u8, i32, ),
    sendVisibilityDevelopmentInfo(i32, Vector3, ),
    shoot(f32, ),
    teleportTo(Vector3, f32, ),
    setDevelopmentFeature(String, i32, String, ),
    receiveFakeShot(i32, f32, Vector3, Vector3, u8, ),
    setAvatar(Mailbox, ),
    registerObserver(Mailbox, u8, ),
    onClientConnected(u8, ),
    onBattleRunning(u8, u8, ),
    sendStateToOwnClient(),
    onEnemyVehicleShot(i32, Vec<i32>, u8, f32, f32, ),
    scheduleExtraCheck(i32, f32, ),
    onDetectedByEnemy(i32, u8, u8, ),
    onConcealedFromEnemy(i32, ),
    updateVehicleAmmo(i32, u16, u8, u8, i16, i16, i16, ),
    onFlagAction(u8, Python, u8, ),
    receiveAssistsFromArena(Vec<u8>, Vec<i32>, ),
    receiveFirstDetectionFromArena(i32, u8, u16, ),
    requestDamagedDevicesFromFor(i32, Mailbox, ),
    sendDamagedDevicesTo(Mailbox, ),
    setHonorTitle(String, ),
    receiveTaggedDestructibleKill(u8, ),
    setOnFireByExplosion(ATTACKER_INFO, i32, ),
    onReceiveSpatialData(Vec<VEHICLE_SPATIAL_INFO>, ),
    onResourceAbsorbed(u16, ),
    setInsideResourcePoint(u8, ),
    grantWinPoints(u16, ),
    pauseMechanics(u64, ),
    startOrUpdateExtraFromOutside(String, Python, ),
    damageByEquipment(i32, u8, ),
    updateOwnClientRTT(f32, ),
    receiveVisibilityUpdate(i32, u8, u8, ),
    requestVisibilityLists(Mailbox, ),
    switchSetup(u8, u8, ),
    setEquipmentApplicationPoint(u16, Vector3, Vector2, ),
}

// TeamInfo

pub struct TeamInfo {
    teamID: i32, // all clients
}

pub enum TeamInfo_Client { 
    ThrottledMethods(ThrottledMethods_Client),
    onCombatEquipmentUsed(i32, i32, ),
    showHittingArea(u16, Vector3, Vector3, f64, ),
}

pub enum TeamInfo_Base { 
    ThrottledMethods(ThrottledMethods_Base),
}

pub enum TeamInfo_Cell { 
    ThrottledMethods(ThrottledMethods_Cell),
}

// StepRepairPoint

pub struct StepRepairPoint {
    initTeam: u8, // cell private
    team: u8, // all clients
    pointsPerStep: i16, // cell public
    repairTime: f32, // cell private
    secondsCooldownPerStep: f32, // cell public
    baseSecondsCooldownAfterRepair: f32, // cell public
    percentageShellsPerStep: u8, // cell public
    radius: f32, // all clients
    healGroup: u8, // cell private
}

pub enum StepRepairPoint_Client { }

pub enum StepRepairPoint_Base { }

pub enum StepRepairPoint_Cell { 
    changeOwnerTeam(u8, ),
    setCooldownAfterRepair(i32, f32, u8, ),
}

// StaticDeathZone

pub struct StaticDeathZone {
    zoneId: String, // all clients
    isActive: u8, // all clients
    vehiclesUnderFire: Vec<VEHICLE_IN_DEATHZONE>, // all clients
    maskingPolygonsCount: u8, // all clients
    proximityMarkerStyle: String, // all clients
}

pub enum StaticDeathZone_Client { 
    onDeathZoneDamage(i32, String, ),
    onDeathZoneNotification(u8, i32, f32, f32, ),
    onEntityEnteredInZone(i32, ),
    onEntityLeftZone(i32, ),
}

pub enum StaticDeathZone_Base { }

pub enum StaticDeathZone_Cell { }

// SimulatedVehicle

pub struct SimulatedVehicle {
    publicInfo: PUBLIC_VEHICLE_INFO, // all clients
    isPlayerVehicle: u8, // all clients
    realVehicleID: i32, // all clients
    simulationData_position: Vector3, // all clients
    simulationData_rotation: Vector3, // all clients
    simulationData_velocity: Vector3, // all clients
    simulationData_angVelocity: Vector3, // all clients
    simulationData_simulationType: String, // all clients
    simulationData_health: i16, // all clients
    simulationData_engineMode: Box<[u8; 2]>, // all clients
    simulationData_gunAngles: Vector2, // all clients
    simulationData_turretAndGunSpeed: Vector2, // all clients
    simulationData_damageStickers: Vec<u64>, // all clients
    simulationData_brokenTracks: Vec<TRACK_STATE>, // all clients
    simulationData_siegeState: u8, // all clients
    simulationData_wheelsState: u16, // all clients
    simulationData_wheelsSteering: Vec<f32>, // all clients
    simulationData_tracksInAir: Box<[u8; 2]>, // all clients
}

pub enum SimulatedVehicle_Client { }

pub enum SimulatedVehicle_Base { }

pub enum SimulatedVehicle_Cell { }

// SectorBase

pub struct SectorBase {
    pointsPerSecond: f32, // cell private
    maxPointsPerSecond: f32, // cell private
    ownerStopsCapturing: u8, // cell private
    defaultPointsPerSecond: f32, // cell private
    defaultMaxPointsPerSecond: f32, // cell private
    isActive: u8, // all clients
    team: u8, // all clients
    baseID: u8, // all clients
    sectorID: u8, // all clients
    maxPoints: f32, // all clients
    initMaxPoints: f32, // cell private
    pointsPercentage: u8, // all clients
    capturingStopped: u8, // all clients
    onDamageCooldownTime: f32, // all clients
    radius: f32, // all clients
    isCaptured: u8, // all clients
    invadersCount: u8, // all clients
    expectedCaptureTime: f32, // all clients
}

pub enum SectorBase_Client { 
    EntityTrap(EntityTrap_Client),
}

pub enum SectorBase_Base { 
    EntityTrap(EntityTrap_Base),
}

pub enum SectorBase_Cell { 
    EntityTrap(EntityTrap_Cell),
    setMaxPointsFactor(f32, ),
}

// Sector

pub struct Sector {
    arena: Mailbox, // base
    arenaBase: Mailbox, // cell private
    arenaTypeID: i32, // cell private
    groupID: u8, // all clients
    sectorID: u8, // all clients
    playerGroup: u8, // all clients
    IDInPlayerGroup: u8, // all clients
    lengthX: f32, // all clients
    lengthZ: f32, // all clients
    team: u8, // all clients
    state: u8, // all clients
    initialState: u8, // cell private
    transitionTime: f32, // all clients
    endOfTransitionPeriod: f32, // all clients
    mainDirection: u8, // cell public
    isActive: u8, // cell public
    cp: Python, // cell private
}

pub enum Sector_Client { 
    showBomb(Vector3, ),
}

pub enum Sector_Base { }

pub enum Sector_Cell { 
    harm_receiveAttackResults(ATTACK_RESULTS, ),
}

// RepairBase

pub struct RepairBase {
    baseID: u8, // cell private
    team: u8, // cell private
    repairTime: f32, // cell private
    repairCooldown: f32, // cell private
}

pub enum RepairBase_Client { 
    ControlPoint(ControlPoint_Client),
}

pub enum RepairBase_Base { 
    ControlPoint(ControlPoint_Base),
}

pub enum RepairBase_Cell { 
    ControlPoint(ControlPoint_Cell),
}

// ProtectionZone

pub struct ProtectionZone {
    arena: Mailbox, // base
    arenaBase: Mailbox, // cell private
    zoneID: u8, // all clients
    lengthX: f32, // all clients
    lengthZ: f32, // all clients
    team: u8, // all clients
    maxStayTime: f32, // cell public
    numberOfTurrets: u8, // cell private
    minTurretShootInterval: f32, // cell private
    minShootingTime: f32, // cell private
    shotDuration: f32, // cell private
    shotRadius: f32, // cell private
    shotShellNation: String, // cell private
    shotShellName: String, // cell private
    shotPiercingPower: f32, // cell private
    mainDirection: u8, // cell private
    isActive: u8, // all clients
    cp: Python, // cell private
}

pub enum ProtectionZone_Client { }

pub enum ProtectionZone_Base { }

pub enum ProtectionZone_Cell { 
    harm_receiveAttackResults(ATTACK_RESULTS, ),
}

// PlatoonTank

pub struct PlatoonTank {
    markerHeightFactor: f32, // all clients
    vehicleTurretYaw: f32, // all clients
    vehicleGunPitch: f32, // all clients
    slotIndex: i32, // all clients
}

pub enum PlatoonTank_Client { }

pub enum PlatoonTank_Base { }

pub enum PlatoonTank_Cell { }

// PlatoonLighting

pub struct PlatoonLighting {
    animationStateMachine: String, // all clients
}

pub enum PlatoonLighting_Client { }

pub enum PlatoonLighting_Base { }

pub enum PlatoonLighting_Cell { }

// PersonalDeathZone

pub struct PersonalDeathZone {
}

pub enum PersonalDeathZone_Client { }

pub enum PersonalDeathZone_Base { }

pub enum PersonalDeathZone_Cell { }

// OfflineEntity

pub struct OfflineEntity {
}

pub enum OfflineEntity_Client { }

pub enum OfflineEntity_Base { }

pub enum OfflineEntity_Cell { }

// NetworkEntity

pub struct NetworkEntity {
    unique_id: String, // all clients
    prefab_path: String, // all clients
    scale: Vector3, // all clients
    goState: Vec<GAME_OBJECT_STATE>, // all clients
    useDetailedPosition: u8, // cell private
    name: String, // all clients
}

pub enum NetworkEntity_Client { 
    activateGameObject(),
    activateGameObjectUnique(),
    deactivateGameObject(),
    deactivateGameObjectUnique(),
    createGameObject(),
    removeGameObject(),
    removeGameObjectUnique(),
}

pub enum NetworkEntity_Base { }

pub enum NetworkEntity_Cell { }

// Login

pub struct Login {
    accountDBID_s: String, // base and client
    loginPriority: u32, // base
}

pub enum Login_Client { 
    onKickedFromServer(i32, ),
    receiveLoginQueueNumber(u64, ),
    setPeripheryRoutingGroup(String, Python, ),
}

pub enum Login_Base { 
    onEnqueued(String, u64, ),
    onAccountClientReleased(Mailbox, ),
}

pub enum Login_Cell { }

// LimitedVisibilityEntity

pub struct LimitedVisibilityEntity {
    team: u8, // cell private
    vehicleIDs: Vec<i32>, // cell private
}

pub enum LimitedVisibilityEntity_Client { }

pub enum LimitedVisibilityEntity_Base { }

pub enum LimitedVisibilityEntity_Cell { }

// HeroTank

pub struct HeroTank {
    markerHeightFactor: f32, // all clients
    vehicleTurretYaw: f32, // all clients
    vehicleGunPitch: f32, // all clients
}

pub enum HeroTank_Client { }

pub enum HeroTank_Base { }

pub enum HeroTank_Cell { }

// HangarVehicle

pub struct HangarVehicle {
}

pub enum HangarVehicle_Client { }

pub enum HangarVehicle_Base { }

pub enum HangarVehicle_Cell { }

// HangarPoster

pub struct HangarPoster {
    minAlpha: f32, // all clients
    maxAlphaDistance: f32, // all clients
}

pub enum HangarPoster_Client { }

pub enum HangarPoster_Base { }

pub enum HangarPoster_Cell { }

// FlockExotic

pub struct FlockExotic {
    animSpeedMax: f32, // all clients
    animSpeedMin: f32, // all clients
    modelCount: u8, // all clients
    modelName: String, // all clients
    modelName2: String, // all clients
    speed: f32, // all clients
    initSpeedRandom: Vector2, // all clients
    speedRandom: Vector2, // all clients
    accelerationTime: f32, // all clients
    triggerRadius: f32, // all clients
    explosionRadius: Vector2, // all clients
    spawnRadius: f32, // all clients
    spawnHeight: f32, // all clients
    flightRadius: f32, // all clients
    flightHeight: f32, // all clients
    flightAngleMin: f32, // all clients
    flightAngleMax: f32, // all clients
    flightOffsetFromOrigin: f32, // all clients
    lifeTime: f32, // all clients
    respawnTime: f32, // all clients
    flightSound: String, // all clients
}

pub enum FlockExotic_Client { }

pub enum FlockExotic_Base { }

pub enum FlockExotic_Cell { }

// Flock

pub struct Flock {
    modelName: String, // all clients
    modelName2: String, // all clients
    modelCount: u8, // all clients
    yawSpeed: f32, // all clients
    pitchSpeed: f32, // all clients
    rollSpeed: f32, // all clients
    animSpeedMin: f32, // all clients
    animSpeedMax: f32, // all clients
    height: f32, // all clients
    radius: f32, // all clients
    deadZoneRadius: f32, // all clients
    speedAtBottom: f32, // all clients
    speedAtTop: f32, // all clients
    decisionTime: f32, // all clients
    flyAroundCenter: u8, // all clients
}

pub enum Flock_Client { }

pub enum Flock_Base { }

pub enum Flock_Cell { }

// EmptyEntity

pub struct EmptyEntity {
}

pub enum EmptyEntity_Client { }

pub enum EmptyEntity_Base { }

pub enum EmptyEntity_Cell { }

// DetachedTurret

pub struct DetachedTurret {
    cp: Python, // cell private
    arenaTypeID: i32, // cell private
    velocity: Vector3, // cell public
    arenaBase: Mailbox, // cell private
    angularVelocity: Vector3, // cell private
    vehicleCompDescr: String, // all clients
    outfitCD: String, // all clients
    isUnderWater: u8, // all clients
    isCollidingWithWorld: u8, // all clients
    vehicleID: i32, // all clients
    attackerInfo: ATTACKER_INFO, // cell public
    vehicleInfo: ATTACKER_INFO, // cell public
    isAtackerProxy: u8, // cell public
}

pub enum DetachedTurret_Client { 
    onStaticCollision(f32, Vector3, Vector3, ),
    showDamageFromShot(Vec<u64>, u8, ),
}

pub enum DetachedTurret_Base { }

pub enum DetachedTurret_Cell { 
    receiveShot(ATTACKER_INFO, i32, i32, Vector3, Vector3, Vector3, ),
    receiveExplosion(Vector3, f32, f32, u8, ),
    applyForceToCOM(Vector3, ),
}

// DestructibleEntity

pub struct DestructibleEntity {
    arena: Mailbox, // base
    arenaBase: Mailbox, // cell private
    isActive: u8, // all clients
    team: u8, // all clients
    destructibleEntityID: u8, // all clients
    health: f32, // all clients
    maxHealth: f32, // all clients
    isDestructibleDestroyed: u8, // all clients
    udoTypeID: u8, // cell private
    typeID: u8, // all clients
    initActive: u8, // base
    linkedMapActivities: String, // all clients
    damageStickers: Vec<u64>, // all clients
    explosionDamageFactor: f32, // cell private
    cp: Python, // cell private
}

pub enum DestructibleEntity_Client { 
    Destructible(Destructible_Client),
    onHealthChanged(i16, i32, u8, i32, ),
    showDamageFromShot(i32, u8, i32, ),
    showDamageFromExplosion(i32, i32, ),
}

pub enum DestructibleEntity_Base { 
    Destructible(Destructible_Base),
}

pub enum DestructibleEntity_Cell { 
    Destructible(Destructible_Cell),
    onEnemyVehicleShot(i32, Vec<i32>, u8, f32, f32, ),
    scheduleExtraCheck(i32, f32, ),
}

// DebugDrawEntity

pub struct DebugDrawEntity {
    drawObjects: Vec<Anonymous142>, // all clients
}

pub enum DebugDrawEntity_Client { }

pub enum DebugDrawEntity_Base { }

pub enum DebugDrawEntity_Cell { 
    remove(String, ),
    clear(String, ),
    drawLine(Vector3, Vector3, f32, String, ),
    drawLines(Vec<Vector3>, f32, Python, ),
    drawPath(Vec<Vector3>, f32, Python, ),
    drawCube(Vector3, f32, Python, ),
    drawCubes(Vec<Vector3>, f32, Python, ),
    drawSphere(Vector3, f32, Python, ),
    drawSpheres(Vec<Vector3>, f32, Python, ),
    write3DText(String, Vector3, Python, ),
    setLifeTime(String, f32, ),
}

// Comp7Lighting

pub struct Comp7Lighting {
    animationStateMachine: String, // all clients
}

pub enum Comp7Lighting_Client { }

pub enum Comp7Lighting_Base { }

pub enum Comp7Lighting_Cell { }

// ClientSelectableWebLinksOpener

pub struct ClientSelectableWebLinksOpener {
    url: String, // all clients
}

pub enum ClientSelectableWebLinksOpener_Client { }

pub enum ClientSelectableWebLinksOpener_Base { }

pub enum ClientSelectableWebLinksOpener_Cell { }

// ClientSelectableRankedObject

pub struct ClientSelectableRankedObject {
}

pub enum ClientSelectableRankedObject_Client { }

pub enum ClientSelectableRankedObject_Base { }

pub enum ClientSelectableRankedObject_Cell { }

// ClientSelectableObject

pub struct ClientSelectableObject {
    modelName: String, // all clients
    selectionId: String, // all clients
    mouseOverSoundName: String, // all clients
    isOver3DSound: u8, // all clients
    clickSoundName: String, // all clients
    isClick3DSound: u8, // all clients
    edgeMode: u8, // all clients
}

pub enum ClientSelectableObject_Client { }

pub enum ClientSelectableObject_Base { }

pub enum ClientSelectableObject_Cell { }

// ClientSelectableHangarsSwitcher

pub struct ClientSelectableHangarsSwitcher {
    destHangar: String, // all clients
}

pub enum ClientSelectableHangarsSwitcher_Client { }

pub enum ClientSelectableHangarsSwitcher_Base { }

pub enum ClientSelectableHangarsSwitcher_Cell { }

// ClientSelectableEasterEgg

pub struct ClientSelectableEasterEgg {
    imageName: String, // all clients
    multiLanguageSupport: u8, // all clients
    outlineModelName: String, // all clients
    animationSequence: String, // all clients
}

pub enum ClientSelectableEasterEgg_Client { }

pub enum ClientSelectableEasterEgg_Base { }

pub enum ClientSelectableEasterEgg_Cell { }

// ClientSelectableCameraVehicle

pub struct ClientSelectableCameraVehicle {
    modelName: String, // all clients
}

pub enum ClientSelectableCameraVehicle_Client { }

pub enum ClientSelectableCameraVehicle_Base { }

pub enum ClientSelectableCameraVehicle_Cell { }

// ClientSelectableCameraObject

pub struct ClientSelectableCameraObject {
}

pub enum ClientSelectableCameraObject_Client { }

pub enum ClientSelectableCameraObject_Base { }

pub enum ClientSelectableCameraObject_Cell { }

// BasicMine

pub struct BasicMine {
    equipmentID: u32, // all clients
    ownerVehicleID: u32, // all clients
    isDetonated: u8, // all clients
    isActivated: u8, // all clients
    activationTimeDelay: u32, // all clients
    mineNumber: u8, // all clients
    isMarkerEnabled: u8, // all clients
}

pub enum BasicMine_Client { }

pub enum BasicMine_Base { }

pub enum BasicMine_Cell { }

// AvatarInfo

pub struct AvatarInfo {
    avatarID: i32, // all clients
}

pub enum AvatarInfo_Client { }

pub enum AvatarInfo_Base { }

pub enum AvatarInfo_Cell { }

// Avatar

pub struct Avatar {
    state: u16, // base
    name: String, // base and client
    sessionID: String, // base and client
    account: Mailbox, // base
    playerVehicle: Mailbox, // base
    arena: Mailbox, // base
    arenaVehiclesIDs: Python, // base
    accountStartedAt: Python, // base
    accountSessionID: String, // base
    arenaUniqueID: u64, // base and client
    syncData: Python, // base
    arenaTypeID: i32, // base and client
    accountDBID: i64, // base
    arenaBonusType: u8, // base and client
    arenaGuiType: u8, // base and client
    arenaExtraData: Python, // base and client
    weatherPresetID: u8, // base and client
    denunciationsLeft: i16, // base and client
    clientCtx: String, // base and client
    tkillIsSuspected: u8, // base and client
    arenaBase: Mailbox, // cell private
    team: u8, // own client
    playerVehicleBase: Mailbox, // cell private
    playerVehicleID: i32, // own client
    playerVehicleTypeCompDescr: u32, // cell private
    isObserverBothTeams: u8, // own client
    observableTeamID: u8, // own client
    isGunLocked: u8, // own client
    ownVehicleGear: u8, // own client
    ownVehicleAuxPhysicsData: u64, // own client
    ownVehicleHullAimingPitchPacked: u16, // own client
    ammo: Vec<i32>, // cell private
    ammoViews: AVATAR_AMMO_VIEWS, // all clients
    ammoAdvanced: AVATAR_AMMO_FOR_CELL, // cell private
    cp: Python, // cell private
    historyLoggingFlags: u64, // cell private
    heatmapLoggingFlags: u32, // cell private
    gameParamsRev: u64, // cell private
    accountDBIDOnCell: i64, // cell private
    arenaUniqueIDOnCell: u64, // cell private
    arenaTypeIDOnCell: i32, // cell private
    arenaBonusTypeOnCell: u8, // cell private
    orderingRoster: Vec<AVATAR_VEHICLE_ROSTER>, // cell private
    viewpoints: Vec<Vector3>, // cell private
    customizationDisplayType: u8, // own client
    inBattleQuestNames: Vec<String>, // cell private
    inBattleQuestProgresses: Python, // cell private
    playLimits: PLAY_LIMITS, // base and client
    arenaGameParamRev: u64, // base
    goodiesSnapshot: Vec<BATTLE_GOODIE_RECORD>, // base and client
    fairplayState: Python, // cell private
    needCheckPenalties: u8, // cell private
    shouldSendKillcamSimulationData: u8, // own client
}

pub enum Avatar_Client { 
    Chat(Chat_Client),
    PlayerMessenger_chat2(PlayerMessenger_chat2_Client),
    ClientCommandsPort(ClientCommandsPort_Client),
    InvitationsClient(InvitationsClient_Client),
    AccountAuthTokenProviderClient(AccountAuthTokenProviderClient_Client),
    AvatarObserver(AvatarObserver_Client),
    TeamHealthBar_Avatar(TeamHealthBar_Avatar_Client),
    ProtectionZoneController_Avatar(ProtectionZoneController_Avatar_Client),
    RecoveryMechanic_Avatar(RecoveryMechanic_Avatar_Client),
    DestructibleEntity_Avatar(DestructibleEntity_Avatar_Client),
    RespawnController_Avatar(RespawnController_Avatar_Client),
    VehiclesSpawnListStorage_Avatar(VehiclesSpawnListStorage_Avatar_Client),
    VehicleRemovalController_Avatar(VehicleRemovalController_Avatar_Client),
    VehicleHealthBroadcastListenerComponent_Avatar(VehicleHealthBroadcastListenerComponent_Avatar_Client),
    TriggersController_Avatar(TriggersController_Avatar_Client),
    AvatarEpic(AvatarEpic_Client),
    update(String, ),
    onKickedFromServer(String, u8, u32, ),
    onIGRTypeChanged(String, ),
    onAutoAimVehicleLost(u8, ),
    receiveAccountStats(u32, String, ),
    updateVehicleHealth(i32, i16, i8, u8, u8, ),
    updateVehicleAmmo(i32, i32, u16, u8, u8, i16, i16, i16, ),
    onSwitchViewpoint(i32, Vector3, ),
    updateVehicleSetting(i32, u8, i32, ),
    updateTargetingInfo(f32, f32, f32, f32, f32, f32, f32, f32, f32, ),
    updateTargetVehicleID(i32, ),
    showOwnVehicleHitDirection(f32, i32, u16, u32, u8, u8, i32, u8, ),
    showOtherVehicleDamagedDevices(i32, Vec<u8>, Vec<u8>, ),
    showShotResults(Vec<u64>, ),
    showDevelopmentInfo(u8, String, ),
    showHittingArea(u16, Vector3, Vector3, f64, ),
    showCarpetBombing(u16, Vector3, Vector3, f64, ),
    showTracer(i32, i32, u8, u8, Vector3, Vector3, f32, f32, u8, ),
    stopTracer(i32, Vector3, ),
    explodeProjectile(i32, u8, u8, Vector3, Vector3, Vec<u32>, ),
    onRoundFinished(i8, u8, ),
    onKickedFromArena(u8, ),
    onBattleEvents(Vec<BATTLE_EVENT>, ),
    battleEventsSummary(BATTLE_EVENTS_SUMMARY, ),
    updateArena(u8, String, ),
    updatePositions(Vec<u8>, Vec<i16>, ),
    receivePhysicsDebugInfo(String, ),
    updateCarriedFlagPositions(Vec<u8>, Vec<i16>, ),
    receiveNotification(String, ),
    onRepairPointAction(u8, u8, f32, ),
    updateAvatarPrivateStats(String, ),
    updateResourceAmount(u8, u32, ),
    onFrictionWithVehicle(i32, Vector3, u8, ),
    onCollisionWithVehicle(Vector3, f32, ),
    onSmoke(SMOKE_INFO, ),
    onCombatEquipmentShotLaunched(u16, Vector3, ),
    updateQuestProgress(String, Python, ),
    updateVehicleQuickShellChanger(i32, u8, ),
    enemySPGHit(Vector3, ),
    enemySPGShotSound(Vector3, Vector3, ),
    handleScriptEventFromServer(String, String, String, String, String, ),
    setUpdatedGoodiesSnapshot(Vec<BATTLE_GOODIE_RECORD>, ),
    onRandomEvent(String, ),
}

pub enum Avatar_Base { 
    Chat(Chat_Base),
    PlayerMessenger_chat2(PlayerMessenger_chat2_Base),
    ClientCommandsPort(ClientCommandsPort_Base),
    InvitationsClient(InvitationsClient_Base),
    AccountAuthTokenProviderClient(AccountAuthTokenProviderClient_Base),
    AvatarObserver(AvatarObserver_Base),
    TeamHealthBar_Avatar(TeamHealthBar_Avatar_Base),
    ProtectionZoneController_Avatar(ProtectionZoneController_Avatar_Base),
    RecoveryMechanic_Avatar(RecoveryMechanic_Avatar_Base),
    DestructibleEntity_Avatar(DestructibleEntity_Avatar_Base),
    RespawnController_Avatar(RespawnController_Avatar_Base),
    VehiclesSpawnListStorage_Avatar(VehiclesSpawnListStorage_Avatar_Base),
    VehicleRemovalController_Avatar(VehicleRemovalController_Avatar_Base),
    VehicleHealthBroadcastListenerComponent_Avatar(VehicleHealthBroadcastListenerComponent_Avatar_Base),
    TriggersController_Avatar(TriggersController_Avatar_Base),
    AvatarEpic(AvatarEpic_Base),
    logLag(),
    kickSelf(String, u8, f32, ),
    setClientReady(),
    leaveArena(),
    onLoginToCellFailed(),
    unlockUnusedVehicles(Vec<i32>, ),
    confirmBattleResultsReceiving(),
    makeDenunciation(i32, i32, i8, ),
    banUnbanUser(i64, u8, u32, String, i8, ),
    requestToken(u16, u8, ),
    sendAccountStats(u32, Vec<String>, ),
    setClientCtx(String, ),
    updateArena(u8, String, ),
    processDisclosures(Vec<DISCLOSE_EVENT>, ),
    vehicle_teleport(Vector3, f32, ),
    vehicle_replenishAmmo(),
    onRemovedFromArena(u64, ),
    onKickedFromArena(u64, u16, ),
    onRoundFinished(i8, u8, ),
    setDevelopmentFeature(i32, String, i32, String, ),
    addBotToArena(String, u8, String, Vector3, u8, ),
    receiveFakeShot(i32, f32, Vector3, Vector3, u8, ),
    logStreamCorruption(i16, i32, i32, i32, i32, ),
    releaseClientForLogin(u8, ),
    onBattleResultsAvailable(String, ),
    stopByFailure(u8, ),
    leaveArenaByAccount(u8, ),
    addSyncData(Python, ),
    removeSyncData(Python, ),
    updateGoodiesSnapshot(Vec<BATTLE_GOODIE_RECORD>, ),
    onXPUpdatedNotification(i16, ),
    showDevelopmentInfo(u8, String, ),
}

pub enum Avatar_Cell { 
    Chat(Chat_Cell),
    PlayerMessenger_chat2(PlayerMessenger_chat2_Cell),
    ClientCommandsPort(ClientCommandsPort_Cell),
    InvitationsClient(InvitationsClient_Cell),
    AccountAuthTokenProviderClient(AccountAuthTokenProviderClient_Cell),
    AvatarObserver(AvatarObserver_Cell),
    TeamHealthBar_Avatar(TeamHealthBar_Avatar_Cell),
    ProtectionZoneController_Avatar(ProtectionZoneController_Avatar_Cell),
    RecoveryMechanic_Avatar(RecoveryMechanic_Avatar_Cell),
    DestructibleEntity_Avatar(DestructibleEntity_Avatar_Cell),
    RespawnController_Avatar(RespawnController_Avatar_Cell),
    VehiclesSpawnListStorage_Avatar(VehiclesSpawnListStorage_Avatar_Cell),
    VehicleRemovalController_Avatar(VehicleRemovalController_Avatar_Cell),
    VehicleHealthBroadcastListenerComponent_Avatar(VehicleHealthBroadcastListenerComponent_Avatar_Cell),
    TriggersController_Avatar(TriggersController_Avatar_Cell),
    AvatarEpic(AvatarEpic_Cell),
    autoAim(i32, u8, ),
    moveTo(Vector3, ),
    bindToVehicle(i32, ),
    monitorVehicleDamagedDevices(i32, ),
    onOwnVehicleStatusChanged(i8, i8, u8, u8, ),
    forbidUnbindingFromVehicle(i8, ),
    discloseEntities(Vec<DISCLOSE_EVENT>, ),
    receiveVisibilityUpdate(i32, u8, u8, u8, ),
    receiveVisibilityLists(i32, Python, Python, ),
    receiveVehicleDamagedDevices(i32, Vec<u8>, Vec<u8>, ),
    lockGunOnClient(u8, ),
    showShotResults(Vec<u64>, ),
    explodeProjectile(i32, u8, u8, Vector3, Vector3, Vec<u32>, ),
    refreshVehicle(u16, ),
    activateEquipment(u16, i16, ),
    setEquipmentApplicationPoint(u16, Vector3, Vector2, ),
    switchViewPointOrBindToVehicle(u8, i32, ),
    grantRagePoints(u8, f32, ),
    harm_receiveAttackResults(ATTACK_RESULTS, ),
    pauseMechanics(u64, ),
    updateOwnVehicleAuxPhysicsDataAndGear(u64, u8, ),
    setDualGunCharger(u8, ),
    reportClientStats(CLIENT_STATUS_STATISTICS, ),
    onRankUpdated(u8, u8, ),
    vehicle_moveWith(u8, ),
    vehicle_shoot(),
    vehicle_trackWorldPointWithGun(Vector3, ),
    vehicle_trackRelativePointWithGun(Vector3, ),
    vehicle_stopTrackingWithGun(f32, f32, ),
    setupAmmo(i64, ),
    vehicle_changeSetting(u8, i32, ),
    setServerMarker(u8, ),
    setSendKillCamSimulationData(u8, ),
}

// AttackBomber

pub struct AttackBomber {
}

pub enum AttackBomber_Client { }

pub enum AttackBomber_Base { }

pub enum AttackBomber_Cell { }

// AttackArtilleryFort

pub struct AttackArtilleryFort {
    team: i32, // all clients
}

pub enum AttackArtilleryFort_Client { }

pub enum AttackArtilleryFort_Base { }

pub enum AttackArtilleryFort_Cell { }

// ArenaObserverInfo

pub struct ArenaObserverInfo {
}

pub enum ArenaObserverInfo_Client { }

pub enum ArenaObserverInfo_Base { }

pub enum ArenaObserverInfo_Cell { }

// ArenaInfo

pub struct ArenaInfo {
}

pub enum ArenaInfo_Client { 
    PlaneTrajectoryArenaInfo(PlaneTrajectoryArenaInfo_Client),
    showCarpetBombing(u16, Vector3, Vector3, f32, ),
}

pub enum ArenaInfo_Base { 
    PlaneTrajectoryArenaInfo(PlaneTrajectoryArenaInfo_Base),
}

pub enum ArenaInfo_Cell { 
    PlaneTrajectoryArenaInfo(PlaneTrajectoryArenaInfo_Cell),
}

// AreaOfEffect

pub struct AreaOfEffect {
    arenaID: i32, // cell public
    vehicleID: i32, // all clients
    equipmentID: i32, // all clients
    launchTime: f64, // all clients
    strikeTime: f64, // all clients
}

pub enum AreaOfEffect_Client { 
    playEffect(String, Vector3, f32, ),
}

pub enum AreaOfEffect_Base { }

pub enum AreaOfEffect_Cell { }

// AreaDestructibles

pub struct AreaDestructibles {
    arenaID: i32, // cell public
    chunkID: u16, // cell public
    arenaTypeID: i32, // cell private
    arenaBonusType: u8, // cell private
    destroyedModules: Vec<Box<[u8; 3]>>, // all clients
    destroyedFragiles: Vec<Box<[u8; 3]>>, // all clients
    fallenColumns: Vec<Box<[u8; 3]>>, // all clients
    fallenTrees: Vec<Box<[u8; 5]>>, // all clients
    destructibles: Python, // cell public
    waters: Vec<Box<[f32; 5]>>, // cell public
    resetCount: u32, // cell public
    arenaBase: Mailbox, // cell private
    arenaGeometryID: i16, // cell private
    cp: Python, // cell private
    arena: Mailbox, // base
}

pub enum AreaDestructibles_Client { }

pub enum AreaDestructibles_Base { 
    createCellNearHere(Mailbox, ),
}

pub enum AreaDestructibles_Cell { 
    damageDestructible(u8, u8, f32, f32, f32, i8, DESTRUCTIBLE_ATTACK_INFO, ),
    receiveTaggedDestructibleKill(u8, ),
}

// ApplicationPoint

pub struct ApplicationPoint {
    vehicleID: i32, // all clients
    equipmentID: i32, // all clients
    launchTime: f32, // all clients
    level: i32, // all clients
}

pub enum ApplicationPoint_Client { }

pub enum ApplicationPoint_Base { }

pub enum ApplicationPoint_Cell { }

// Account

pub struct Account {
    name: String, // base and client
    normalizedName: String, // base
    globalRating: u32, // base
    ver: i16, // base
    accountType: u32, // base
    attrs: u64, // base
    premiumExpiryTime: u32, // base
    autoBanTime: u32, // base
    clanDBID: i64, // base
    lastUserMessageID: i64, // base
    lastSystemMessageID: i64, // base
    lastInternalSystemMessageID: i64, // base
    vivoxCredentials: String, // base
    jabberCredentials: String, // base
    vhID: u64, // base
    incarnationID: u64, // base and client
    peripheryID: i32, // base
    saveTime: i32, // base
    lastPlayerActivityTime: i32, // base
    vehDossiersCutVer: u8, // base
    vehDossiersVer: u8, // base
    nextOffloadToPeripheryTime: i32, // base
    walletID: u64, // base
    extWalletID: u64, // base
    pdata: String, // base
    bp: Python, // base
    initialServerSettings: Python, // base and client
}

pub enum Account_Client { 
    Chat(Chat_Client),
    PlayerMessenger_chat2(PlayerMessenger_chat2_Client),
    AccountEditor(AccountEditor_Client),
    TransactionUser(TransactionUser_Client),
    InterclusterSender(InterclusterSender_Client),
    ClientCommandsPort(ClientCommandsPort_Client),
    AccountAdmin(AccountAdmin_Client),
    AccountAvatar(AccountAvatar_Client),
    AccountClan(AccountClan_Client),
    AccountAuthTokenProvider(AccountAuthTokenProvider_Client),
    AccountAuthTokenProviderClient(AccountAuthTokenProviderClient_Client),
    BattleResultProcessor(BattleResultProcessor_Client),
    Invitations(Invitations_Client),
    InvitationsClient(InvitationsClient_Client),
    Invoicing(Invoicing_Client),
    AccountPrebattle(AccountPrebattle_Client),
    AccountSpaProcessor(AccountSpaProcessor_Client),
    AccountIGRProcessing(AccountIGRProcessing_Client),
    SessionTracker(SessionTracker_Client),
    AccountGlobalMapConnector(AccountGlobalMapConnector_Client),
    AccountSysMessenger(AccountSysMessenger_Client),
    AccountUnit(AccountUnit_Client),
    AccountUnitClient(AccountUnitClient_Client),
    AccountUnitRemote(AccountUnitRemote_Client),
    AccountUnitAssembler(AccountUnitAssembler_Client),
    AccountUnitBrowser(AccountUnitBrowser_Client),
    AccountDebugger(AccountDebugger_Client),
    QuestProcessor(QuestProcessor_Client),
    AvatarCreator(AvatarCreator_Client),
    AccountVersion(AccountVersion_Client),
    PlayLimits(PlayLimits_Client),
    ServerSideReplays(ServerSideReplays_Client),
    onKickedFromServer(String, u8, u32, ),
    onEnqueued(u8, ),
    onEnqueueFailure(u8, u8, String, ),
    onDequeued(u8, ),
    onKickedFromQueue(u8, ),
    onIGRTypeChanged(String, ),
    onArenaJoinFailure(u8, String, ),
    onPrebattleJoined(i32, ),
    onPrebattleJoinFailure(u8, ),
    onKickedFromArena(u8, ),
    onKickedFromPrebattle(u8, ),
    onCenterIsLongDisconnected(u8, ),
    showGUI(String, ),
    receiveActiveArenas(Vec<PUBLIC_ARENA_INFO>, ),
    receiveServerStats(SERVER_STATISTICS, ),
    receiveQueueInfo(Python, ),
    updatePrebattle(u8, String, ),
    update(String, ),
    resyncDossiers(u8, ),
    onUnitUpdate(u64, String, String, ),
    onUnitCallOk(i32, ),
    onUnitNotify(u64, i32, String, Python, ),
    onUnitError(i32, u64, i32, String, ),
    onUnitBrowserError(i32, String, ),
    onUnitBrowserResultsSet(String, ),
    onUnitBrowserResultsUpdate(String, ),
    onGlobalMapUpdate(String, String, ),
    onGlobalMapReply(u64, i32, String, ),
    onSendPrebattleInvites(i64, String, i64, String, u64, u8, ),
    onClanInfoReceived(i64, String, String, String, String, ),
    receiveNotification(String, ),
}

pub enum Account_Base { 
    Chat(Chat_Base),
    PlayerMessenger_chat2(PlayerMessenger_chat2_Base),
    AccountEditor(AccountEditor_Base),
    TransactionUser(TransactionUser_Base),
    InterclusterSender(InterclusterSender_Base),
    ClientCommandsPort(ClientCommandsPort_Base),
    AccountAdmin(AccountAdmin_Base),
    AccountAvatar(AccountAvatar_Base),
    AccountClan(AccountClan_Base),
    AccountAuthTokenProvider(AccountAuthTokenProvider_Base),
    AccountAuthTokenProviderClient(AccountAuthTokenProviderClient_Base),
    BattleResultProcessor(BattleResultProcessor_Base),
    Invitations(Invitations_Base),
    InvitationsClient(InvitationsClient_Base),
    Invoicing(Invoicing_Base),
    AccountPrebattle(AccountPrebattle_Base),
    AccountSpaProcessor(AccountSpaProcessor_Base),
    AccountIGRProcessing(AccountIGRProcessing_Base),
    SessionTracker(SessionTracker_Base),
    AccountGlobalMapConnector(AccountGlobalMapConnector_Base),
    AccountSysMessenger(AccountSysMessenger_Base),
    AccountUnit(AccountUnit_Base),
    AccountUnitClient(AccountUnitClient_Base),
    AccountUnitRemote(AccountUnitRemote_Base),
    AccountUnitAssembler(AccountUnitAssembler_Base),
    AccountUnitBrowser(AccountUnitBrowser_Base),
    AccountDebugger(AccountDebugger_Base),
    QuestProcessor(QuestProcessor_Base),
    AvatarCreator(AvatarCreator_Base),
    AccountVersion(AccountVersion_Base),
    PlayLimits(PlayLimits_Base),
    ServerSideReplays(ServerSideReplays_Base),
    onEnqueued(u8, ),
    onDequeued(u8, ),
    onMapsTrainingEnqueued(u8, ),
    onArenaCreated(Mailbox, u64, u8, i32, u8, i32, Python, ),
    onMapsTrainingArenaCreated(Mailbox, u64, u8, i32, u8, i32, Python, ),
    onKickedFromQueue(u8, ),
    onKickedFromArena(u64, u8, ),
    releaseClientForLogin(Mailbox, Python, Python, ),
    keepAliveFor(Mailbox, i32, u8, u16, ),
    stopKeepingAlive(u8, ),
    kickSelf(String, u8, u32, ),
    destroySelfForPeriphery(i32, Mailbox, ),
    sendPropertiesTo(Mailbox, i32, Vec<String>, ),
    processWalletResponse(Mailbox, i32, Python, ),
    processWGMoneyResponse(Mailbox, i32, Python, ),
    processReceiptServiceNotification(Mailbox, i32, Python, ),
    extraWriteToDB(u8, ),
    writeToDBForTransfer(i64, String, ),
    receiveClanMemberInfo(i64, i64, String, String, String, String, i64, i32, i32, String, String, ),
    receiveClanMembersListDiff(i64, String, ),
    debugRunMethod(String, Python, ),
    receiveExternalNotification(Python, ),
    sendExternalNotificationReply(i64, String, u8, ),
    giveClientTo(Mailbox, ),
    makeDenunciation(i64, i32, i8, ),
    banUnbanUser(i64, u8, u32, String, i8, ),
    requestToken(u16, u8, ),
    invitationSendInBattle(i64, Vec<i64>, String, u8, Python, Python, ),
    invitationAcceptOrDeclineInBattle(i64, i64, i64, u8, ),
    logStreamCorruption(i16, i32, i32, i32, i32, ),
    setKickAtTime(i64, String, String, ),
    resetClanDBID(i64, ),
    updateAntiTeamingStorage(Python, i64, i64, ),
}

pub enum Account_Cell { 
    Chat(Chat_Cell),
    PlayerMessenger_chat2(PlayerMessenger_chat2_Cell),
    AccountEditor(AccountEditor_Cell),
    TransactionUser(TransactionUser_Cell),
    InterclusterSender(InterclusterSender_Cell),
    ClientCommandsPort(ClientCommandsPort_Cell),
    AccountAdmin(AccountAdmin_Cell),
    AccountAvatar(AccountAvatar_Cell),
    AccountClan(AccountClan_Cell),
    AccountAuthTokenProvider(AccountAuthTokenProvider_Cell),
    AccountAuthTokenProviderClient(AccountAuthTokenProviderClient_Cell),
    BattleResultProcessor(BattleResultProcessor_Cell),
    Invitations(Invitations_Cell),
    InvitationsClient(InvitationsClient_Cell),
    Invoicing(Invoicing_Cell),
    AccountPrebattle(AccountPrebattle_Cell),
    AccountSpaProcessor(AccountSpaProcessor_Cell),
    AccountIGRProcessing(AccountIGRProcessing_Cell),
    SessionTracker(SessionTracker_Cell),
    AccountGlobalMapConnector(AccountGlobalMapConnector_Cell),
    AccountSysMessenger(AccountSysMessenger_Cell),
    AccountUnit(AccountUnit_Cell),
    AccountUnitClient(AccountUnitClient_Cell),
    AccountUnitRemote(AccountUnitRemote_Cell),
    AccountUnitAssembler(AccountUnitAssembler_Cell),
    AccountUnitBrowser(AccountUnitBrowser_Cell),
    AccountDebugger(AccountDebugger_Cell),
    QuestProcessor(QuestProcessor_Cell),
    AvatarCreator(AvatarCreator_Cell),
    AccountVersion(AccountVersion_Cell),
    PlayLimits(PlayLimits_Cell),
    ServerSideReplays(ServerSideReplays_Cell),
}

