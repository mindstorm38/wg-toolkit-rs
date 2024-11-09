
use super::super::alias::*;
use super::interface::*;

/// Entity 0x01
/// Methods for Account on cell component
pub enum AccountMethod { 
}


/// Entity 0x02
/// Methods for Avatar on cell component
pub enum AvatarMethod { 
    AvatarObserver(AvatarObserverMethod),
    autoAim(OBJECT_ID, BOOL), // idx(0)
    moveTo(Vec3), // idx(1)
    bindToVehicle(OBJECT_ID), // idx(2)
    monitorVehicleDamagedDevices(OBJECT_ID), // idx(3)
    activateEquipment(u16, i16), // idx(14)
    setEquipmentApplicationPoint(u16, Vec3, Vec2), // idx(15)
    switchViewPointOrBindToVehicle(BOOL, OBJECT_ID), // idx(16)
    setDualGunCharger(BOOL), // idx(21)
    reportClientStats(CLIENT_STATUS_STATISTICS), // idx(22)
    vehicle_moveWith(u8), // idx(24)
    vehicle_shoot(), // idx(25)
    vehicle_trackWorldPointWithGun(Vec3), // idx(26)
    vehicle_trackRelativePointWithGun(Vec3), // idx(27)
    vehicle_stopTrackingWithGun(f32, f32), // idx(28)
    setupAmmo(i64), // idx(29)
    vehicle_changeSetting(u8, i32), // idx(30)
    setServerMarker(BOOL), // idx(31)
    setSendKillCamSimulationData(BOOL), // idx(32)
}

// 0: vehicle_shoot [Fixed(0)] @ []
// 1: setDualGunCharger [Fixed(1)] @ []
// 2: vehicle_moveWith [Fixed(1)] @ []
// 3: setServerMarker [Fixed(1)] @ []
// 4: setSendKillCamSimulationData [Fixed(1)] @ []
// 5: switchObserverFPV [Fixed(1)] @ ["AvatarObserver"]
// 6: bindToVehicle [Fixed(4)] @ []
// 7: monitorVehicleDamagedDevices [Fixed(4)] @ []
// 8: activateEquipment [Fixed(4)] @ []
// 9: autoAim [Fixed(5)] @ []
// 10: switchViewPointOrBindToVehicle [Fixed(5)] @ []
// 11: vehicle_changeSetting [Fixed(5)] @ []
// 12: vehicle_stopTrackingWithGun [Fixed(8)] @ []
// 13: setupAmmo [Fixed(8)] @ []
// 14: moveTo [Fixed(12)] @ []
// 15: vehicle_trackWorldPointWithGun [Fixed(12)] @ []
// 16: vehicle_trackRelativePointWithGun [Fixed(12)] @ []
// 17: setEquipmentApplicationPoint [Fixed(22)] @ []
// 18: reportClientStats [Fixed(24)] @ []

/// Entity 0x03
/// Methods for ArenaInfo on cell component
pub enum ArenaInfoMethod { 
}


/// Entity 0x04
/// Methods for ClientSelectableObject on cell component
pub enum ClientSelectableObjectMethod { 
}


/// Entity 0x05
/// Methods for HangarVehicle on cell component
pub enum HangarVehicleMethod { 
}


/// Entity 0x06
/// Methods for Vehicle on cell component
pub enum VehicleMethod { 
    VehicleObserver(VehicleObserverMethod),
    RecoveryMechanic_Vehicle(RecoveryMechanic_VehicleMethod),
    moveWith(u8), // idx(1)
    trackWorldPointWithGun(Vec3), // idx(2)
    trackRelativePointWithGun(Vec3), // idx(3)
    stopTrackingWithGun(f32, f32), // idx(4)
    changeSetting(u8, i32), // idx(6)
    sendVisibilityDevelopmentInfo(OBJECT_ID, Vec3), // idx(7)
    sendStateToOwnClient(), // idx(16)
    switchSetup(u8, u8), // idx(40)
}

// 0: sendStateToOwnClient [Fixed(0)] @ []
// 1: recoveryMechanic_startRecovering [Fixed(0)] @ ["RecoveryMechanic_Vehicle"]
// 2: recoveryMechanic_stopRecovering [Fixed(0)] @ ["RecoveryMechanic_Vehicle"]
// 3: moveWith [Fixed(1)] @ []
// 4: switchSetup [Fixed(2)] @ []
// 5: changeSetting [Fixed(5)] @ []
// 6: stopTrackingWithGun [Fixed(8)] @ []
// 7: trackWorldPointWithGun [Fixed(12)] @ []
// 8: trackRelativePointWithGun [Fixed(12)] @ []
// 9: sendVisibilityDevelopmentInfo [Fixed(16)] @ []
// 10: setRemoteCamera [Fixed(22)] @ ["VehicleObserver"]

/// Entity 0x07
/// Methods for AreaDestructibles on cell component
pub enum AreaDestructiblesMethod { 
}


/// Entity 0x08
/// Methods for OfflineEntity on cell component
pub enum OfflineEntityMethod { 
}


/// Entity 0x09
/// Methods for Flock on cell component
pub enum FlockMethod { 
}


/// Entity 0x0A
/// Methods for FlockExotic on cell component
pub enum FlockExoticMethod { 
}


/// Entity 0x0B
/// Methods for Login on cell component
pub enum LoginMethod { 
}


/// Entity 0x0C
/// Methods for DetachedTurret on cell component
pub enum DetachedTurretMethod { 
}


/// Entity 0x0D
/// Methods for DebugDrawEntity on cell component
pub enum DebugDrawEntityMethod { 
}


/// Entity 0x0E
/// Methods for ClientSelectableCameraObject on cell component
pub enum ClientSelectableCameraObjectMethod { 
}


/// Entity 0x0F
/// Methods for ClientSelectableCameraVehicle on cell component
pub enum ClientSelectableCameraVehicleMethod { 
}


/// Entity 0x10
/// Methods for ClientSelectableWebLinksOpener on cell component
pub enum ClientSelectableWebLinksOpenerMethod { 
}


/// Entity 0x11
/// Methods for ClientSelectableEasterEgg on cell component
pub enum ClientSelectableEasterEggMethod { 
}


/// Entity 0x12
/// Methods for EmptyEntity on cell component
pub enum EmptyEntityMethod { 
}


/// Entity 0x13
/// Methods for LimitedVisibilityEntity on cell component
pub enum LimitedVisibilityEntityMethod { 
}


/// Entity 0x14
/// Methods for HeroTank on cell component
pub enum HeroTankMethod { 
}


/// Entity 0x15
/// Methods for PlatoonTank on cell component
pub enum PlatoonTankMethod { 
}


/// Entity 0x16
/// Methods for PlatoonLighting on cell component
pub enum PlatoonLightingMethod { 
}


/// Entity 0x17
/// Methods for SectorBase on cell component
pub enum SectorBaseMethod { 
}


/// Entity 0x18
/// Methods for Sector on cell component
pub enum SectorMethod { 
}


/// Entity 0x19
/// Methods for DestructibleEntity on cell component
pub enum DestructibleEntityMethod { 
}


/// Entity 0x1A
/// Methods for StepRepairPoint on cell component
pub enum StepRepairPointMethod { 
}


/// Entity 0x1B
/// Methods for ProtectionZone on cell component
pub enum ProtectionZoneMethod { 
}


/// Entity 0x1C
/// Methods for HangarPoster on cell component
pub enum HangarPosterMethod { 
}


/// Entity 0x1D
/// Methods for TeamInfo on cell component
pub enum TeamInfoMethod { 
}


/// Entity 0x1E
/// Methods for AvatarInfo on cell component
pub enum AvatarInfoMethod { 
}


/// Entity 0x1F
/// Methods for ArenaObserverInfo on cell component
pub enum ArenaObserverInfoMethod { 
}


/// Entity 0x20
/// Methods for AreaOfEffect on cell component
pub enum AreaOfEffectMethod { 
}


/// Entity 0x21
/// Methods for AttackBomber on cell component
pub enum AttackBomberMethod { 
}


/// Entity 0x22
/// Methods for AttackArtilleryFort on cell component
pub enum AttackArtilleryFortMethod { 
}


/// Entity 0x23
/// Methods for PersonalDeathZone on cell component
pub enum PersonalDeathZoneMethod { 
}


/// Entity 0x24
/// Methods for ClientSelectableRankedObject on cell component
pub enum ClientSelectableRankedObjectMethod { 
}


/// Entity 0x25
/// Methods for SimulatedVehicle on cell component
pub enum SimulatedVehicleMethod { 
}


/// Entity 0x26
/// Methods for ClientSelectableHangarsSwitcher on cell component
pub enum ClientSelectableHangarsSwitcherMethod { 
}


/// Entity 0x27
/// Methods for StaticDeathZone on cell component
pub enum StaticDeathZoneMethod { 
}


/// Entity 0x28
/// Methods for BasicMine on cell component
pub enum BasicMineMethod { 
}


/// Entity 0x29
/// Methods for ApplicationPoint on cell component
pub enum ApplicationPointMethod { 
}


/// Entity 0x2A
/// Methods for NetworkEntity on cell component
pub enum NetworkEntityMethod { 
}


/// Entity 0x2B
/// Methods for Comp7Lighting on cell component
pub enum Comp7LightingMethod { 
}


