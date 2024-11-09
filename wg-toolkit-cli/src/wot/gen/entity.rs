use wgtk::net::app::common::element::Entity;

use super::alias::*;
use super::interface::*;
use super::{base, cell, client};

/// Entity 0x01
/// Interface Account
#[derive(Debug)]
pub struct Account {
    pub i_AccountVersion: AccountVersion,
    pub name: String,
    pub incarnationID: u64,
    pub initialServerSettings: Python,
}

/// Entity 0x02
/// Interface Avatar
#[derive(Debug)]
pub struct Avatar {
    pub i_AvatarObserver: AvatarObserver,
    pub name: String,
    pub sessionID: String,
    pub arenaUniqueID: u64,
    pub arenaTypeID: i32,
    pub arenaBonusType: u8,
    pub arenaGuiType: u8,
    pub arenaExtraData: Python,
    pub weatherPresetID: u8,
    pub denunciationsLeft: i16,
    pub clientCtx: String,
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

/// Entity 0x03
/// Interface ArenaInfo
#[derive(Debug)]
pub struct ArenaInfo {
    pub i_PlaneTrajectoryArenaInfo: PlaneTrajectoryArenaInfo,
}

/// Entity 0x04
/// Interface ClientSelectableObject
#[derive(Debug)]
pub struct ClientSelectableObject {
    pub modelName: String,
    pub selectionId: String,
    pub mouseOverSoundName: String,
    pub isOver3DSound: BOOL,
    pub clickSoundName: String,
    pub isClick3DSound: BOOL,
    pub edgeMode: u8,
}

/// Entity 0x05
/// Interface HangarVehicle
#[derive(Debug)]
pub struct HangarVehicle {
}

/// Entity 0x06
/// Interface Vehicle
#[derive(Debug)]
pub struct Vehicle {
    pub i_VehicleObserver: VehicleObserver,
    pub i_Wheels: Wheels,
    pub i_Perks_Vehicle: Perks_Vehicle,
    pub isStrafing: BOOL,
    pub postmortemViewPointName: String,
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
    pub crewCompactDescrs: Vec<String>,
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

/// Entity 0x07
/// Interface AreaDestructibles
#[derive(Debug)]
pub struct AreaDestructibles {
    pub destroyedModules: Vec<Box<[u8; 3]>>,
    pub destroyedFragiles: Vec<Box<[u8; 3]>>,
    pub fallenColumns: Vec<Box<[u8; 3]>>,
    pub fallenTrees: Vec<Box<[u8; 5]>>,
}

/// Entity 0x08
/// Interface OfflineEntity
#[derive(Debug)]
pub struct OfflineEntity {
}

/// Entity 0x09
/// Interface Flock
#[derive(Debug)]
pub struct Flock {
    pub modelName: String,
    pub modelName2: String,
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

/// Entity 0x0A
/// Interface FlockExotic
#[derive(Debug)]
pub struct FlockExotic {
    pub animSpeedMax: f32,
    pub animSpeedMin: f32,
    pub modelCount: u8,
    pub modelName: String,
    pub modelName2: String,
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
    pub flightSound: String,
}

/// Entity 0x0B
/// Interface Login
#[derive(Debug)]
pub struct Login {
    pub accountDBID_s: String,
}

/// Entity 0x0C
/// Interface DetachedTurret
#[derive(Debug)]
pub struct DetachedTurret {
    pub vehicleCompDescr: String,
    pub outfitCD: String,
    pub isUnderWater: BOOL,
    pub isCollidingWithWorld: BOOL,
    pub vehicleID: i32,
}

/// Entity 0x0D
/// Interface DebugDrawEntity
#[derive(Debug)]
pub struct DebugDrawEntity {
    pub drawObjects: Vec<ANON180>,
}

/// Entity 0x0E
/// Interface ClientSelectableCameraObject
#[derive(Debug)]
pub struct ClientSelectableCameraObject {
}

/// Entity 0x0F
/// Interface ClientSelectableCameraVehicle
#[derive(Debug)]
pub struct ClientSelectableCameraVehicle {
    pub modelName: String,
}

/// Entity 0x10
/// Interface ClientSelectableWebLinksOpener
#[derive(Debug)]
pub struct ClientSelectableWebLinksOpener {
    pub url: String,
}

/// Entity 0x11
/// Interface ClientSelectableEasterEgg
#[derive(Debug)]
pub struct ClientSelectableEasterEgg {
    pub imageName: String,
    pub multiLanguageSupport: BOOL,
    pub outlineModelName: String,
    pub animationSequence: String,
}

/// Entity 0x12
/// Interface EmptyEntity
#[derive(Debug)]
pub struct EmptyEntity {
}

/// Entity 0x13
/// Interface LimitedVisibilityEntity
#[derive(Debug)]
pub struct LimitedVisibilityEntity {
}

/// Entity 0x14
/// Interface HeroTank
#[derive(Debug)]
pub struct HeroTank {
    pub markerHeightFactor: f32,
    pub vehicleTurretYaw: f32,
    pub vehicleGunPitch: f32,
}

/// Entity 0x15
/// Interface PlatoonTank
#[derive(Debug)]
pub struct PlatoonTank {
    pub markerHeightFactor: f32,
    pub vehicleTurretYaw: f32,
    pub vehicleGunPitch: f32,
    pub slotIndex: i32,
}

/// Entity 0x16
/// Interface PlatoonLighting
#[derive(Debug)]
pub struct PlatoonLighting {
    pub animationStateMachine: String,
}

/// Entity 0x17
/// Interface SectorBase
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

/// Entity 0x18
/// Interface Sector
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

/// Entity 0x19
/// Interface DestructibleEntity
#[derive(Debug)]
pub struct DestructibleEntity {
    pub isActive: BOOL,
    pub team: u8,
    pub destructibleEntityID: u8,
    pub health: f32,
    pub maxHealth: f32,
    pub isDestructibleDestroyed: BOOL,
    pub typeID: u8,
    pub linkedMapActivities: String,
    pub damageStickers: Vec<u64>,
}

/// Entity 0x1A
/// Interface StepRepairPoint
#[derive(Debug)]
pub struct StepRepairPoint {
    pub team: u8,
    pub radius: f32,
}

/// Entity 0x1B
/// Interface ProtectionZone
#[derive(Debug)]
pub struct ProtectionZone {
    pub zoneID: u8,
    pub lengthX: f32,
    pub lengthZ: f32,
    pub team: u8,
    pub isActive: BOOL,
}

/// Entity 0x1C
/// Interface HangarPoster
#[derive(Debug)]
pub struct HangarPoster {
    pub minAlpha: f32,
    pub maxAlphaDistance: f32,
}

/// Entity 0x1D
/// Interface TeamInfo
#[derive(Debug)]
pub struct TeamInfo {
    pub teamID: i32,
}

/// Entity 0x1E
/// Interface AvatarInfo
#[derive(Debug)]
pub struct AvatarInfo {
    pub avatarID: OBJECT_ID,
}

/// Entity 0x1F
/// Interface ArenaObserverInfo
#[derive(Debug)]
pub struct ArenaObserverInfo {
}

/// Entity 0x20
/// Interface AreaOfEffect
#[derive(Debug)]
pub struct AreaOfEffect {
    pub vehicleID: i32,
    pub equipmentID: i32,
    pub launchTime: f64,
    pub strikeTime: f64,
}

/// Entity 0x21
/// Interface AttackBomber
#[derive(Debug)]
pub struct AttackBomber {
}

/// Entity 0x22
/// Interface AttackArtilleryFort
#[derive(Debug)]
pub struct AttackArtilleryFort {
    pub team: i32,
}

/// Entity 0x23
/// Interface PersonalDeathZone
#[derive(Debug)]
pub struct PersonalDeathZone {
}

/// Entity 0x24
/// Interface ClientSelectableRankedObject
#[derive(Debug)]
pub struct ClientSelectableRankedObject {
}

/// Entity 0x25
/// Interface SimulatedVehicle
#[derive(Debug)]
pub struct SimulatedVehicle {
    pub publicInfo: PUBLIC_VEHICLE_INFO,
    pub isPlayerVehicle: BOOL,
    pub realVehicleID: OBJECT_ID,
    pub simulationData_position: Vec3,
    pub simulationData_rotation: Vec3,
    pub simulationData_velocity: Vec3,
    pub simulationData_angVelocity: Vec3,
    pub simulationData_simulationType: String,
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

/// Entity 0x26
/// Interface ClientSelectableHangarsSwitcher
#[derive(Debug)]
pub struct ClientSelectableHangarsSwitcher {
    pub destHangar: String,
}

/// Entity 0x27
/// Interface StaticDeathZone
#[derive(Debug)]
pub struct StaticDeathZone {
    pub zoneId: String,
    pub isActive: BOOL,
    pub vehiclesUnderFire: Vec<VEHICLE_IN_DEATHZONE>,
    pub maskingPolygonsCount: u8,
    pub proximityMarkerStyle: String,
}

/// Entity 0x28
/// Interface BasicMine
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

/// Entity 0x29
/// Interface ApplicationPoint
#[derive(Debug)]
pub struct ApplicationPoint {
    pub vehicleID: i32,
    pub equipmentID: i32,
    pub launchTime: f32,
    pub level: i32,
}

/// Entity 0x2A
/// Interface NetworkEntity
#[derive(Debug)]
pub struct NetworkEntity {
    pub unique_id: String,
    pub prefab_path: String,
    pub scale: Vec3,
    pub goState: Vec<GAME_OBJECT_STATE>,
    pub name: String,
}

/// Entity 0x2B
/// Interface Comp7Lighting
#[derive(Debug)]
pub struct Comp7Lighting {
    pub animationStateMachine: String,
}

