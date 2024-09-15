#![allow(non_camel_case_types, non_snake_case)]

pub type Vector2 = [f32; 2];
pub type Vector3 = [f32; 3];
pub type Vector4 = [f32; 4];
pub type Python = String;
pub type Mailbox = String;

pub type BOOL = u8;
pub type OBJECT_ID = i32;
pub type SHOT_ID = i32;
pub type DB_ID = i64;
pub type EXTRA_ID = u8;
pub type VEH_TYPE_CD = u32;

#[derive(Debug)]
pub struct SERVER_STATISTICS {
    pub clusterCCU: u32,
    pub regionCCU: u32,
}

pub type QUEUE_INFO = Python;

#[derive(Debug)]
pub struct DEFAULT_QUEUE_INFO {
    pub classes: Vec<u32>,
    pub players: u32,
}

#[derive(Debug)]
pub struct MAPS_TRAINING_QUEUE_INFO {
    pub queues: Python,
}

#[derive(Debug)]
pub struct FUN_RANDOM_QUEUE_INFO {
    pub events: Python,
}

#[derive(Debug)]
pub struct PUBLIC_ARENA_INFO {
    pub id: i32,
    pub roundLength: i32,
    pub typeID: i32,
    pub roundStart: f32,
}

#[derive(Debug)]
pub struct ATTACK_RESULTS {
    pub targetIsOnTheCapturableBase: u8,
    pub targetIsOnTheIgnoredBase: u8,
    pub crits: i32,
    pub targetTeam: u8,
    pub hitFlags: i32,
    pub anyDeviceWasDamaged: u8,
    pub enemiesNearTarget: u8,
    pub stunAssistants: Vec<i32>,
    pub equipmentID: u16,
    pub attackerWasInvisible: u8,
    pub targetTypeCompDescr: u32,
    pub reason: u8,
    pub targetMaxHealth: u16,
    pub targetHealthBeforeDamage: i16,
    pub critBonusFactor: f32,
    pub targetIsTeamKiller: u8,
    pub targetInitialSpeed: f32,
    pub numVehiclesAffected: i16,
    pub attackerHullDamage: u16,
    pub targetIsNotSpotted: u8,
    pub attackerKilledHimself: u8,
    pub circularVisionRadius: f32,
    pub attackerType: u8,
    pub shellCompDescr: i32,
    pub attackerHealthBeforeDamage: i16,
    pub hitIndirection: u8,
    pub isIronShieldDamage: u8,
    pub shotID: i32,
    pub stunDuration: f32,
    pub repairCost: u32,
    pub targetVehicleIndex: u8,
    pub allCrits: i32,
    pub trackAssistants: Vec<i32>,
    pub isRecoil: u8,
    pub smokeAssistants: Vec<i32>,
    pub attackerInitialSpeed: f32,
    pub droppedCapturePoints: f32,
    pub distance: f32,
    pub targetID: i32,
    pub damage: i32,
}

#[derive(Debug)]
pub struct PREBATTLE_INVITE {
    pub comment: String,
    pub creatorClanAbbrev: String,
    pub createTime: u32,
    pub creatorBadges: Python,
    pub r#type: u16,
    pub creatorDBID: i64,
    pub extraData: Python,
    pub creator: String,
}

#[derive(Debug)]
pub struct PREBATTLE_RESULTS {
    pub finishReason: u8,
    pub winner: u8,
    pub kickReason: u8,
    pub extendedResults: Python,
}

#[derive(Debug)]
pub struct PUBLIC_CHAT_CHANNEL_INFO {
    pub id: i32,
    pub channelName: String,
    pub isReadOnly: u8,
    pub flags: u8,
    pub isSecured: u8,
}

#[derive(Debug)]
pub struct PUBLIC_USERS_ROSTER_MEMBER_INFO {
    pub id: i64,
    pub nickName: String,
    pub accessFlags: u8,
}

#[derive(Debug)]
pub struct CHAT_ACTION_DATA {
    pub originatorNickName: String,
    pub action: u8,
    pub sentTime: f64,
    pub channel: i32,
    pub actionResponse: u8,
    pub flags: u8,
    pub originator: i64,
    pub group: u8,
    pub time: f64,
    pub data: Python,
    pub requestID: i64,
}

#[derive(Debug)]
pub struct GENERIC_MESSENGER_ARGS_chat2 {
    pub strArg1: String,
    pub floatArg1: f64,
    pub int64Arg1: i64,
    pub int32Arg1: i32,
    pub strArg2: String,
}

#[derive(Debug)]
pub struct DISCLOSE_EVENT {
    pub vehicleID: i32,
    pub vehicleType: u8,
    pub playerName: u8,
}

#[derive(Debug)]
pub struct DOG_TAG_COMPONENT {
    pub progress: f32,
    pub id: i32,
    pub grade: i8,
}

#[derive(Debug)]
pub struct DOG_TAG {
    pub components: Vec<DOG_TAG_COMPONENT>,
}

#[derive(Debug)]
pub struct BATTLE_DOG_TAG {
    pub defaultDogTag: DOG_TAG,
    pub showDogTagToKiller: u8,
    pub dogTag: DOG_TAG,
}

#[derive(Debug)]
pub struct VEHICLE_DOG_TAG {
    pub vehicleId: i32,
    pub dogTag: DOG_TAG,
}

#[derive(Debug)]
pub struct DOG_TAG_SETTINGS {
    pub showDogTagToKiller: u8,
    pub showVictimsDogTag: u8,
}

#[derive(Debug)]
pub struct PUBLIC_VEHICLE_INFO {
    pub compDescr: String,
    pub outfit: String,
    pub name: String,
    pub team: u8,
    pub marksOnGun: u8,
    pub index: u8,
    pub maxHealth: u16,
    pub prebattleID: i32,
    pub commanderSkinID: u16,
    pub crewGroups: Vec<u16>,
}

#[derive(Debug)]
pub struct ARENA_VEH_INFO {
    pub vehPostProgression: Vec<i32>,
    pub vehAmmo: Vec<i32>,
    pub prestigeLevel: i32,
    pub vehCrew: Vec<String>,
    pub vehPerks: Python,
    pub marksOnGun: u8,
    pub vehDisabledSetupSwitches: Vec<u8>,
    pub isElite: u8,
    pub customRoleSlotTypeId: u8,
    pub vehCrewSkins: Python,
    pub vehSetupsIndexes: Python,
    pub prestigeGradeMarkID: i32,
    pub settings: u16,
    pub vehSetups: Python,
    pub enhancements: Python,
    pub vehInvID: i32,
    pub vehOutfit: String,
    pub activeRent: u8,
    pub isSuitableForReferralBonus: u8,
    pub isRent: u8,
    pub anonymizedPrestigeLevel: i32,
    pub vehCompDescr: String,
    pub vehCrewInvIDs: Vec<i32>,
}

#[derive(Debug)]
pub struct USER_EXTRA_SETTINGS {
    pub commandercam: u8,
    pub postmortemMode: u8,
    pub hideNonHistoric: u8,
    pub crewPerks: u8,
    pub mapsInDevelopment: u8,
    pub dogTagsUserSettings: DOG_TAG_SETTINGS,
    pub moduleRepairTimer: u8,
    pub contourImpenetrableZone: u8,
    pub contourPenetrableZone: u8,
    pub hpinminimap: u8,
    pub minimapMinSpottingRange: u8,
    pub battleNotifier: u8,
    pub additionalzoom: u8,
    pub hpinplayerspanels: u8,
    pub contour: u8,
}

#[derive(Debug)]
pub struct ARENA_ADDPLAYER_INFO {
    pub arenaUniqueID: u64,
    pub team: u8,
    pub potapovQuestInfo: Python,
    pub group: u8,
    pub centerID: i32,
    pub potapovQuestIDs: Vec<u16>,
    pub needCheckPenalties: u8,
    pub vehiclesInfo: Vec<ARENA_VEH_INFO>,
    pub overriddenBadge: u8,
    pub cybersportRating: Vec<f32>,
    pub isSsrPlayEnabled: u8,
    pub clanDBID: i64,
    pub recruiterId: i64,
    pub battlesNum: u32,
    pub isPrebattleCreator: u8,
    pub dogTag: BATTLE_DOG_TAG,
    pub databaseID: i64,
    pub fairplayState: Python,
    pub prebattle: Mailbox,
    pub globalRating: f32,
    pub tkillRating: f32,
    pub avatarAmmo: Vec<i32>,
    pub igrType: i8,
    pub ranked: Vec<u16>,
    pub premMask: i8,
    pub attrs: u64,
    pub isSsrRecordEnabled: u8,
    pub forbidInBattleInvitations: u8,
    pub wtr: i16,
    pub name: String,
    pub needsAnonymization: u8,
    pub clanAbbrev: String,
    pub badges: Python,
    pub userExtraSettings: USER_EXTRA_SETTINGS,
    pub componentsData: Python,
}

#[derive(Debug)]
pub struct AVATAR_VEHICLE_ROSTER {
    pub vehicleID: i32,
    pub observer: u8,
    pub team: i8,
    pub prebattleID: i32,
}

#[derive(Debug)]
pub struct ATTACKER_INFO {
    pub attackerType: u8,
    pub equipmentID: u16,
    pub health: i16,
    pub receiveAttackResultsMB: Mailbox,
    pub attackerInitialSpeed: f32,
    pub attackerTypeCompactDescr: u32,
    pub team: u8,
    pub attackerVehicleIndex: u8,
    pub attackerSiegeState: i8,
    pub attackerMasterVehID: i32,
    pub needsCount: u8,
    pub damageDistributionLowerBound: f32,
    pub piercingDistributionLowerBound: f32,
    pub attackerDualAccuracyState: i8,
    pub position: Vector3,
    pub piercingDistributionUpperBound: f32,
    pub attackerWasInvisible: u8,
    pub criticalHitChanceBoost: f32,
    pub baseMB: Mailbox,
    pub circularVisionRadius: f32,
    pub attackerGunBurstCount: u8,
    pub noOwner: u8,
    pub damageDistributionUpperBound: f32,
}

#[derive(Debug)]
pub struct DESTRUCTIBLE_ATTACK_INFO {
    pub attacker: ATTACKER_INFO,
    pub hitPoint: Vector3,
    pub shotID: i32,
}

#[derive(Debug)]
pub struct CLIENT_STATUS_STATISTICS {
    pub ping_101_150: f32,
    pub ping_gt_400: f32,
    pub lag: f32,
    pub ping_151_400: f32,
    pub ping_lt_50: f32,
    pub ping_51_100: f32,
}

#[derive(Debug)]
pub struct VEHICLE_SPATIAL_INFO {
    pub prebattleID: i32,
    pub position: Vector3,
    pub team: u8,
    pub vehClass: String,
    pub isAlive: u8,
    pub vehicleID: i32,
}

#[derive(Debug)]
pub struct VEHICLE_SYNC_ATTRS {
    pub gunPiercing: f32,
    pub gunShotsSpeed: f32,
    pub circularVisionRadius: u16,
}

#[derive(Debug)]
pub struct IS_OBSERVED_BY_ENEMY {
    pub endTime: f32,
}

#[derive(Debug)]
pub struct SIEGE_STATE_STATUS {
    pub timeLeft: f32,
    pub status: u8,
    pub endTime: f32,
}

#[derive(Debug)]
pub struct BURNOUT_WARNING {
    pub status: u8,
}

#[derive(Debug)]
pub struct DUAL_GUN_STATUS_TIMES {
    pub timeLeft: f32,
    pub endTime: f32,
    pub baseTime: f32,
}

#[derive(Debug)]
pub struct DUAL_GUN_STATUS {
    pub times: DUAL_GUN_STATUS_TIMES,
    pub status: u8,
}

#[derive(Debug)]
pub struct DESTROYED_DEVICE_IS_REPAIRING {
    pub timeLeft: f32,
    pub extraIndex: u8,
    pub repairMode: u8,
    pub endTime: f32,
    pub progress: u8,
}

pub type DESTROYED_DEVICES_IS_REPAIRING = Vec<DESTROYED_DEVICE_IS_REPAIRING>;

#[derive(Debug)]
pub struct IS_OTHER_VEHICLE_DAMAGED_DEVICES_VISIBLE {
    pub status: u8,
}

#[derive(Debug)]
pub struct BURNOUT_UNAVAILABLE {
    pub status: u8,
}

#[derive(Debug)]
pub struct OVERTURN_LEVEL {
    pub times: Vec<f64>,
    pub level: u8,
}

#[derive(Debug)]
pub struct IN_AOE_ZONE_STATUS {
    pub zoneID: i32,
    pub equipmentID: u16,
    pub endTime: f64,
    pub team: u8,
    pub startTime: f64,
}

pub type IN_AOE_ZONE = Vec<IN_AOE_ZONE_STATUS>;

#[derive(Debug)]
pub struct DROWN_LEVEL {
    pub level: u8,
    pub times: Vec<f64>,
}

#[derive(Debug)]
pub struct BATTLE_EVENT {
    pub targetID: i32,
    pub eventType: u8,
    pub count: u16,
    pub details: u64,
}

#[derive(Debug)]
pub struct BATTLE_EVENTS_SUMMARY {
    pub lastDeathReasonID: u8,
    pub radioAssist: u32,
    pub lastKillerID: i32,
    pub damage: u32,
    pub trackAssist: u32,
    pub inspireAssist: u32,
    pub smokeAssist: u32,
    pub tankings: u32,
    pub stunAssist: u32,
}

#[derive(Debug)]
pub struct REMOTE_CAMERA_DATA {
    pub shotPoint: Vector3,
    pub time: f64,
    pub mode: u8,
    pub zoom: u8,
}

pub type STUN_INFO = f64;
pub type BOUNDS2 = Box<[Vector2; 2]>;

#[derive(Debug)]
pub struct FRONT_LINE_DATA {
    pub columnWidth: f32,
    pub frontDropPerColumn: f32,
    pub outlierFraction: f32,
    pub outlierVerticalDistance: f32,
    pub intrusionCheckExtendBounds: f32,
    pub intrusionVerticalTolerance: f32,
    pub frontLineIds: Vec<u8>,
    pub frontLineBounds: Vec<Box<[Vector2; 2]>>,
    pub frontLineMainDirVecs: Vec<Vector2>,
    pub frontEdgeExtendColumns: u8,
    pub defenderTeam: u8,
}

pub type LIST_OF_COMP_DESCRS = Vec<i32>;
pub type LIST_OF_IDX = Vec<i8>;

#[derive(Debug)]
pub struct AVATAR_AMMO_VIEWS {
    pub compDescrs: Vec<Vec<i32>>,
    pub vehTypeCompDescrs: Vec<i32>,
}

#[derive(Debug)]
pub struct AVATAR_AMMO_FOR_CELL {
    pub abilitiesIDs: Python,
    pub purchasedAbilities: Vec<i8>,
}

#[derive(Debug)]
pub struct POST_PROGRESSION_SETUPS {
    pub devicesSetups: Vec<Vec<u32>>,
    pub shellsSetups: Vec<Vec<u32>>,
    pub eqsSetups: Vec<Vec<u32>>,
    pub boostersSetups: Vec<Vec<u32>>,
}

#[derive(Debug)]
pub struct RESPAWN_AVAILABLE_VEHICLE {
    pub compDescr: String,
    pub vehDisabledSetupSwitches: Vec<u8>,
    pub vehPostProgression: Vec<i32>,
    pub settings: u16,
    pub vehSetupsIndexes: Vec<u8>,
    pub crewCompactDescrs: Vec<String>,
    pub vehSetups: POST_PROGRESSION_SETUPS,
    pub customRoleSlotTypeId: u8,
}

#[derive(Debug)]
pub struct RESPAWN_COOLDOWN_ITEM {
    pub endOfCooldownPiT: f32,
    pub vehTypeCompDescr: u32,
}

#[derive(Debug)]
pub struct RESPAWN_ZONE {
    pub isEnemyNear: u8,
    pub position: Vector3,
}

pub type LIST_OF_RESPAWN_ZONE = Vec<RESPAWN_ZONE>;

#[derive(Debug)]
pub struct RESPAWN_INFO {
    pub chosenRespawnZone: Vector3,
    pub vehSetupsIndexes: Vec<u8>,
    pub compDescr: String,
    pub respawnType: u8,
    pub manualRespawnPiT: f32,
    pub autoRespawnPiT: f32,
    pub respawnZones: Vec<RESPAWN_ZONE>,
}

#[derive(Debug)]
pub struct RESPAWN_LIMITED_VEHICLES {
    pub group: u8,
    pub vehTypeCompDescrs: Vec<u32>,
}

#[derive(Debug)]
pub struct RESPAWN_INFO_VEHICLE {
    pub vehSetups: Python,
    pub compDescr: String,
    pub prestigeGradeMarkID: i32,
    pub marksOnGun: u8,
    pub policyID: u8,
    pub ammo: Vec<i32>,
    pub onRespawnSettings: Python,
    pub vehPostProgression: Vec<i32>,
    pub crewCompactDescrs: Vec<String>,
    pub vehSetupsIndexes: Python,
    pub prestigeLevel: i32,
    pub vehPerks: Python,
    pub yaw: f32,
    pub position: Vector3,
    pub index: u16,
    pub prevGroup: u8,
    pub customRoleSlotTypeId: u8,
    pub group: u8,
    pub vehDisabledSetupSwitches: Vec<u8>,
    pub commanderSkinID: u16,
    pub outfit: String,
}

#[derive(Debug)]
pub struct BUFF_EFFECT {
    pub startTime: f64,
    pub inactivationDelay: f32,
    pub radius: f32,
    pub endTime: f64,
}

#[derive(Debug)]
pub struct DOT_EFFECT {
    pub endTime: f64,
    pub attackReasonID: u8,
    pub groupId: u8,
    pub period: f32,
}

#[derive(Debug)]
pub struct BUFF_EFFECT_INACTIVATION {
    pub senderKey: String,
    pub startTime: f64,
    pub endTime: f64,
    pub inactivationStartTime: f64,
    pub inactivationEndTime: f64,
}

#[derive(Debug)]
pub struct HOT_EFFECT {
    pub inactivationEndTime: f64,
    pub isInfluenceZone: u8,
    pub startTime: f64,
    pub senderKey: String,
    pub endTime: f64,
    pub inactivationStartTime: f64,
}

#[derive(Debug)]
pub struct INSPIRED_EFFECT {
    pub inactivationEndTime: f64,
    pub endTime: f64,
    pub startTime: f64,
    pub primary: u8,
    pub inactivationSource: u8,
    pub equipmentID: u16,
    pub inactivationStartTime: f64,
}

#[derive(Debug)]
pub struct SMOKE_INFO {
    pub equipmentID: u16,
    pub endTime: f32,
    pub team: u8,
    pub smokeID: f64,
    pub expiring: u8,
}

#[derive(Debug)]
pub struct VEHICLE_DETECTOR_INFO {
    pub point: Vector3,
    pub radius: f32,
    pub detectorID: u16,
    pub aliveOnly: u8,
}

#[derive(Debug)]
pub struct COOLDOWN_INFO {
    pub id: u8,
    pub baseTime: f32,
    pub leftTime: f32,
    pub endTime: f32,
}

#[derive(Debug)]
pub struct CLIENT_VEHICLE_PUBLIC_INFO {
    pub name: String,
    pub isTeamKiller: i8,
    pub clanDBID: i64,
    pub sessionID: String,
    pub overriddenBadge: u8,
    pub botDisplayStatus: u8,
    pub forbidInBattleInvitations: i8,
    pub isAlive: i8,
    pub customRoleSlotTypeId: u8,
    pub isPrebattleCreator: i8,
    pub team: i8,
    pub wtr: i16,
    pub accountDBID: i64,
    pub igrType: i8,
    pub potapovQuestIDs: Vec<u16>,
    pub realName: String,
    pub vehPostProgression: Vec<i32>,
    pub vehID: i32,
    pub isAvatarReady: i8,
    pub events: Python,
    pub ranked: Python,
    pub maxHealth: u16,
    pub prebattleID: i32,
    pub compDescr: String,
    pub potapovQuestInfo: Python,
    pub prestigeLevel: i32,
    pub clanAbbrev: String,
    pub anonymizedPrestigeLevel: i32,
    pub outfit: String,
    pub badges: Python,
    pub prestigeGradeMarkID: i32,
}

#[derive(Debug)]
pub struct PLAY_LIMITS {
    pub weeklyPlayLimit: i32,
    pub curfew: i32,
    pub sessionLimit: i32,
    pub dailyPlayLimit: i32,
}

#[derive(Debug)]
pub struct VEHICLE_IN_DEATHZONE {
    pub waveDuration: f32,
    pub nextStrikeTime: f32,
    pub vehicleId: i32,
}

#[derive(Debug)]
pub struct TIME_WITH_REASON {
    pub totalTime: i32,
    pub endTime: i32,
    pub reason: u8,
}

#[derive(Debug)]
pub struct PERK_INFO_HUD {
    pub state: u8,
    pub coolDown: f32,
    pub lifeTime: f64,
    pub perkID: i32,
}

#[derive(Debug)]
pub struct PERK_INFO_RIBBON {
    pub endTime: f64,
    pub perkID: i32,
}

#[derive(Debug)]
pub struct TOURNAMEMT_PARTICIPANT {
    pub databaseID: i64,
    pub teamID: u8,
    pub role: u8,
    pub typeCD: i32,
    pub name: String,
}

pub type LIST_OF_TOURNAMEMT_PARTICIPANTS = Vec<TOURNAMEMT_PARTICIPANT>;

#[derive(Debug)]
pub struct STEALTH_RADAR_INFO {
    pub duration: f32,
    pub equipmentID: u16,
    pub isActive: u8,
    pub endTime: f32,
}

#[derive(Debug)]
pub struct REGENERATION_KIT_INFO {
    pub endTime: f32,
    pub duration: f32,
    pub isActive: u8,
}

#[derive(Debug)]
pub struct VEHICLE_HEALTH_INFO {
    pub health: i16,
    pub isCrewActive: u8,
    pub isRespawnActive: u8,
    pub id: i32,
    pub deathReasonID: i8,
}

#[derive(Debug)]
pub struct GUN_MARKER {
    pub dispersion: f32,
    pub shotVector: Vector3,
    pub gunPosition: Vector3,
}

#[derive(Debug)]
pub struct KILL_CAM_ATTACKER_SPOTTED {
    pub velocity: Vector3,
    pub angVelocity: Vector3,
    pub rotation: Vector3,
    pub gunAngles: Vector2,
    pub position: Vector3,
}

#[derive(Debug)]
pub struct KILL_CAM_ATTACKER_UNSPOTTED {
    pub health: u16,
    pub vehicleType: String,
}

#[derive(Debug)]
pub struct KILL_CAM_ATTACKER {
    pub unspottedData: KILL_CAM_ATTACKER_UNSPOTTED,
    pub attackerID: i32,
    pub spottedData: KILL_CAM_ATTACKER_SPOTTED,
}

#[derive(Debug)]
pub struct KILL_CAM_VICTIM {
    pub causeOfDeath: String,
    pub relativeArmor: f32,
    pub health: u16,
    pub siegeState: u16,
    pub damageStickers: Vec<u64>,
    pub rotation: Vector3,
    pub victimIsNotSpotted: u8,
    pub position: Vector3,
}

#[derive(Debug)]
pub struct KILL_CAM_PROJECTILE_UNSPOTTED {
    pub angleGain: f32,
    pub ricochetAngleCos: f32,
    pub impactType: u16,
    pub nominalPiercingPower: f32,
    pub hasNonPiercedDamage: u8,
    pub gravity: f32,
    pub isExplosion: u8,
    pub shellDamageBurstHE: f32,
    pub is2CaliberRuleActive: u8,
    pub distanceLossHE: f32,
    pub hitAngleCos: f32,
    pub triNormal: Vector3,
    pub shellCompDescr: i32,
    pub shotID: i32,
    pub maxPenetrationAngle: f32,
    pub averageDamageOfShell: f32,
    pub nominalArmor: f32,
    pub is3CaliberRuleActive: u8,
    pub damageRandomizationFactor: f32,
    pub hasProjectilePierced: u8,
    pub trajectoryData: Vec<Box<[Vector3; 2]>>,
    pub explosionRadius: f32,
    pub impactPoint: Vector3,
    pub hasDistanceFalloff: u8,
    pub segments: Vec<u64>,
    pub piercingPower: f32,
    pub armorProtectionHE: f32,
    pub velocity: Vector3,
    pub ricochetCount: u16,
    pub effectiveShellDamage: f32,
    pub spallLinerProtectionHE: f32,
}

#[derive(Debug)]
pub struct KILL_CAM_PROJECTILE_SPOTTED {
    pub damageDistanceModifier: f32,
    pub distanceOfShot: f32,
}

#[derive(Debug)]
pub struct KILL_CAM_PROJECTILE {
    pub unspottedData: KILL_CAM_PROJECTILE_UNSPOTTED,
    pub spottedData: KILL_CAM_PROJECTILE_SPOTTED,
}

#[derive(Debug)]
pub struct KILL_CAM_DATA {
    pub attacker: KILL_CAM_ATTACKER,
    pub victim: KILL_CAM_VICTIM,
    pub statusCode: u8,
    pub projectile: KILL_CAM_PROJECTILE,
}

#[derive(Debug)]
pub struct TRACK_STATE {
    pub isBroken: u8,
    pub isDebris: u8,
    pub hitPoint: Vector3,
}

#[derive(Debug)]
pub struct VEHICLE_HIT_DIRECTION {
    pub isBlocked: u8,
    pub crits: u32,
    pub damagedID: i32,
    pub hitDirYaw: f32,
    pub attackerID: i32,
    pub attackReasonID: i8,
    pub isShellHE: u8,
    pub damage: u16,
}

#[derive(Debug)]
pub struct VEHICLE_CLIP_RELOAD_TIME {
    pub endTime: f32,
    pub firstTime: f32,
    pub isBoostApplicable: u8,
    pub baseTime: f32,
    pub timeLeft: f32,
    pub stunned: u8,
}

#[derive(Debug)]
pub struct VEHICLE_GUN_RELOAD_TIME {
    pub endTime: f32,
    pub baseTime: f32,
    pub timeLeft: f32,
}

#[derive(Debug)]
pub struct OWN_VEHICLE_POSITION {
    pub speed: f32,
    pub position: Vector3,
    pub rotationSpeed: f32,
    pub direction: Vector3,
}

#[derive(Debug)]
pub struct TARGET_VEHICLE_ID {
    pub targetID: i32,
}

#[derive(Debug)]
pub struct DUAL_GUN_STATE {
    pub gunStates: Vec<u8>,
    pub cooldowns: Vec<COOLDOWN_INFO>,
    pub activeGun: u8,
}

#[derive(Debug)]
pub struct VEHICLE_AMMO {
    pub quantityInClip: u16,
    pub endTime: f32,
    pub quantity: u16,
    pub compactDescr: i32,
    pub previousStage: u16,
    pub totalTime: i16,
    pub index: u8,
}

pub type VEHICLE_AMMO_LIST = Vec<VEHICLE_AMMO>;

#[derive(Debug)]
pub struct VEHICLE_DAMAGE_INFO {
    pub entityID: i32,
    pub damageIndex: u8,
    pub extraIndex: u8,
    pub equipmentID: u16,
}

pub type VEHICLE_DAMAGE_INFO_LIST = Vec<VEHICLE_DAMAGE_INFO>;

#[derive(Debug)]
pub struct VEHICLE_OPTIONAL_DEVICE_STATUS {
    pub isOn: u8,
    pub deviceID: u8,
}

pub type VEHICLE_OPTIONAL_DEVICE_STATUS_LIST = Vec<VEHICLE_OPTIONAL_DEVICE_STATUS>;

#[derive(Debug)]
pub struct VEHICLE_SETTING {
    pub vehicleID: i32,
    pub code: u8,
    pub value: i32,
}

pub type VEHICLE_SETTINGS = Vec<VEHICLE_SETTING>;

#[derive(Debug)]
pub struct TARGETING_INFO {
    pub chassisShotDispersionFactorsRotation: f32,
    pub turretYaw: f32,
    pub maxTurretRotationSpeed: f32,
    pub gunPitch: f32,
    pub maxGunRotationSpeed: f32,
    pub shotDispMultiplierFactor: f32,
    pub chassisShotDispersionFactorsMovement: f32,
    pub aimingTime: f32,
    pub gunShotDispersionFactorsTurretRotation: f32,
}

#[derive(Debug)]
pub struct BATTLE_EVENTS {
    pub endTime: f32,
    pub events: Vec<BATTLE_EVENT>,
}

#[derive(Debug)]
pub struct WELCOME_TO_SECTOR {
    pub actionDuration: f32,
    pub actionTime: f32,
    pub groupID: u8,
    pub groupState: u8,
    pub goodGroup: u8,
    pub sectorID: u8,
}

#[derive(Debug)]
pub struct SECTOR_SHOOTING {
    pub sectorID: u8,
}

#[derive(Debug)]
pub struct PLANE_TRAJECTORY {
    pub nextPos: Vector3,
    pub curPos: Vector3,
    pub isEndOfFlight: u8,
    pub nextDir: Vector2,
    pub nextTime: f64,
    pub curTime: f64,
    pub team: u8,
    pub curDir: Vector2,
    pub equipmentID: u16,
}

#[derive(Debug)]
pub struct FIRE_INFO {
    pub notificationIndex: u8,
    pub deviceExtraIndex: u8,
    pub equipmentID: u16,
    pub startTime: f64,
    pub attackerID: i32,
}

#[derive(Debug)]
pub struct DEBUFF_INFO {
    pub attackReasonID: u16,
    pub finishTime: f32,
}

pub type DEBUFF_INFO_LIST = Vec<DEBUFF_INFO>;

#[derive(Debug)]
pub struct ROCKET_ACCELERATION_STATE_STATUS {
    pub timeLeft: u16,
    pub status: u8,
    pub endTime: f32,
    pub reuseCount: i8,
}

#[derive(Debug)]
pub struct AUTO_SHOOT_GUN_STATE_STATUS {
    pub state: u8,
    pub stateActivationTime: f32,
    pub dispersionFactor: f32,
    pub updateTime: f32,
    pub maxShotDispersion: f32,
    pub shotDispersionPerSec: f32,
}

#[derive(Debug)]
pub struct GOODIE_RESOURCE {
    pub isPercentage: u8,
    pub r#type: u8,
    pub value: u16,
}

#[derive(Debug)]
pub struct GOODIE_STATE_INFO {
    pub expirations: Python,
    pub state: u8,
    pub count: u16,
    pub finishTime: f64,
}

#[derive(Debug)]
pub struct BATTLE_GOODIE_RECORD {
    pub lifetime: u16,
    pub resource: GOODIE_RESOURCE,
    pub goodieID: u32,
    pub stateInfo: GOODIE_STATE_INFO,
    pub useby: u64,
}

pub type BATTLE_GOODIES_LIST = Vec<BATTLE_GOODIE_RECORD>;

#[derive(Debug)]
pub struct ROLE_EQUIPMENT_STATE {
    pub level: u8,
    pub progress: f32,
}

#[derive(Debug)]
pub struct STATUS_WITH_TIME_INTERVAL {
    pub endTime: f32,
    pub startTime: f32,
    pub statusID: u8,
}

#[derive(Debug)]
pub struct TIME_INTERVAL {
    pub startTime: f32,
    pub endTime: f32,
}

#[derive(Debug)]
pub struct STATE_WITH_TIME_INTERVAL {
    pub stateID: u8,
    pub timeInterval: TIME_INTERVAL,
    pub isSourceVehicle: u8,
}

#[derive(Debug)]
pub struct VISUAL_SCRIPT_EQUIPMENT_STATE {
    pub endTime: f32,
    pub stage: u8,
    pub quantity: i32,
    pub prevStage: u8,
    pub totalTime: f32,
}

#[derive(Debug)]
pub struct VISUAL_SCRIPT_EQUIPMENT_PUBLIC_STATE {
    pub stage: u8,
}

#[derive(Debug)]
pub struct SPAWN_AVAILABLE_VEHICLE {
    pub isRent: u8,
    pub compDescr: String,
    pub isElite: u8,
    pub settings: u16,
}

#[derive(Debug)]
pub struct SPAWN_POINT_INFO {
    pub vehicleID: i32,
    pub number: i8,
}

#[derive(Debug)]
pub struct SPAWN_KEY_POINT {
    pub guid: String,
    pub position: Vector2,
}

#[derive(Debug)]
pub struct TEAM_SPAWN_KEY_POINT {
    pub vehID: i32,
    pub guid: String,
    pub placed: u8,
}

#[derive(Debug)]
pub struct GAME_OBJECT_STATE {
    pub id: String,
    pub state: u8,
}

#[derive(Debug)]
pub struct GAME_OBJECT_ACTIVATION {
    pub start: f32,
    pub end: f32,
    pub id: String,
}

#[derive(Debug)]
pub struct POLYGON_TRIGGER_PARAMS {
    pub dimensions: Vector3,
    pub verts: Vec<Vector3>,
    pub origin: Vector3,
    pub segments: Vec<Vec<u16>>,
    pub step: u16,
}

#[derive(Debug)]
pub struct SPAWNGROUP_INFO {
    pub position: Vector2,
    pub name: String,
}

#[derive(Debug)]
pub struct TEAM_LIVES {
    pub vehicleID: i32,
    pub usedLives: i8,
    pub lives: i8,
    pub lockedLives: i8,
}

#[derive(Debug)]
pub struct DEATH_INFO {
    pub equipmentID: i16,
    pub reasonID: i8,
    pub victimID: i32,
    pub numVehiclesAffected: i16,
    pub killerID: i32,
}

#[derive(Debug)]
pub struct VEHICLES_INFO {
    pub isAlive: u8,
    pub badges: Python,
    pub personalMissionIDs: Vec<i32>,
    pub accountDBID: u64,
    pub forbidInBattleInvitations: u8,
    pub events: Python,
    pub teamPanelMode: u8,
    pub team: i8,
    pub prebattleID: i32,
    pub vehicleID: i32,
    pub igrType: i8,
    pub avatarSessionID: String,
    pub isAvatarReady: u8,
    pub clanAbbrev: String,
    pub maxHealth: i16,
    pub isTeamKiller: u8,
    pub isPrebattleCreator: u8,
    pub name: String,
    pub deathInfo: DEATH_INFO,
    pub personalMissionInfo: Python,
    pub __generation: u32,
    pub prestigeGradeMarkID: i32,
    pub compDescr: String,
    pub ranked: Vec<u16>,
    pub clanDBID: i64,
    pub botDisplayStatus: i8,
    pub customRoleSlotTypeId: i8,
    pub prestigeLevel: i32,
    pub outfitCD: String,
    pub vehPostProgression: Vec<i32>,
    pub fakeName: String,
    pub overriddenBadge: i8,
}

#[derive(Debug)]
pub struct PVE_MINIMAP_DATA {
    pub minimapBorders: Box<[Vector2; 2]>,
    pub zoomLevel: f32,
}

#[derive(Debug)]
pub struct PVE_TARGET_MARKER {
    pub targetId: i32,
    pub lastVisiblePosition: Vector3,
    pub invisibleStyle: String,
    pub visibleStyle: String,
    pub settingId: i16,
}

#[derive(Debug)]
pub struct Anonymous134 {
    pub width: f32,
    pub points: Vec<Vector3>,
}

#[derive(Debug)]
pub struct Anonymous136 {
    pub position: Vector3,
    pub size: Vector3,
}

#[derive(Debug)]
pub struct Anonymous138 {
    pub radius: Vector3,
    pub position: Vector3,
}

#[derive(Debug)]
pub struct Anonymous140 {
    pub text: String,
    pub position: Vector3,
    pub color: Vector4,
    pub textSize: f32,
}

#[derive(Debug)]
pub struct Anonymous142 {
    pub version: u32,
    pub destroyTime: f32,
    pub lines: Vec<Anonymous134>,
    pub spheres: Vec<Anonymous138>,
    pub texts: Vec<Anonymous140>,
    pub cubes: Vec<Anonymous136>,
    pub name: String,
}

