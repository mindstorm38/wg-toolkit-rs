//! This module stores entity-specific codecs. These codecs represent 
//! entities as sent by the server to clients. The opposite is currently
//! not planned. Entities are implemented as regular bundle's element,
//! but they are only useful when wrapped into base/cell elements, such
//! as `CreateBasePlayer`.

use std::io::{self, Read, Write};
use std::sync::Arc;

use crate::util::io::*;
use super::SimpleElement;


/// The Login entity.
/// 
/// ID: 11
#[derive(Debug)]
pub struct Login {
    /// The database identifier of the account. It's the same identifier
    /// has the one publicly available through the Wargaming API. 
    /// 
    /// Such as '518858105' for player 'Mindstorm38_'.
    pub account_db_id: String,
}

impl SimpleElement for Login {

    fn encode<W: Write>(&self, mut write: W) -> io::Result<()> {
        write.write_string_variable(&self.account_db_id)?;
        write.write_u8(0) // I don't know why there is a terminating zero.
    }

    fn decode<R: Read>(mut read: R, _len: usize) -> io::Result<Self> {
        Ok(Self {
            account_db_id: read.read_string_variable()?,
        })
    }

}


/// The account entity.
/// 
/// ID: 1
#[derive(Debug)]
pub struct Account {
    /// Part of the `AccountVersion.def` interface, just used by the 
    /// python app to check that game version is coherent.
    /// 
    /// For example `eu_1.19.1_4` as of this writing.
    pub required_version: String,
    /// The name of the account.
    pub name: String,

    pub initial_server_settings: Arc<server_settings::Settings>,
}

impl SimpleElement for Account {
    
    fn encode<W: Write>(&self, mut write: W) -> io::Result<()> {
        use serde_pickle::SerOptions;
        write.write_string_variable(&self.required_version)?;
        write.write_string_variable(&self.name)?;
        let pickle_data = serde_pickle::to_vec(&*self.initial_server_settings, SerOptions::new().proto_v2()).unwrap();
        write.write_blob_variable(&pickle_data)
    }

    fn decode<R: Read>(mut read: R, _len: usize) -> io::Result<Self> {
        use serde_pickle::DeOptions;
        Ok(Self {
            required_version: read.read_string_variable()?,
            name: read.read_string_variable()?,
            initial_server_settings: {
                let pickle_data = read.read_blob_variable()?;
                Arc::new(serde_pickle::from_slice(&pickle_data, DeOptions::new().decode_strings()).unwrap())
            },
        })
    }

}


/// This module contains the server settings structures, that implements serde 
/// serialize/deserialize, which is mainly used for python pickle serialization.
pub mod server_settings {

    use std::collections::{BTreeSet, BTreeMap, HashSet};
    use serde::{Serialize, Deserialize};

    /// Server settings as used by the client's python code.
    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct Settings {
        pub shop: Shop,
        #[serde(rename = "sessionStats")]
        pub session_stats: SessionStats,
        // prem_battle_bonuses: not used by the client
        // xmpp_muc_enabled: not used by the client
        #[serde(rename = "disabledPMOperations")]
        pub disabled_personal_mission_operations: BTreeMap<String, bool>,
        pub marathon_config: MarathonConfig,
        #[serde(rename = "frontlineSettings")]
        pub frontline_settings: FrontlineSettings,
        #[serde(rename = "isVehicleRestoreEnabled")]
        pub is_vehicle_restore_enabled: bool,
        pub epic_config: EpicConfig,
        pub enhancements_config: EnhancementsConfig,
        /// A mapping from vehicle names to min/max battle level.
        /// This currently doesn't support the case where vehicle type & level are used.
        #[serde(rename = "randomBattleLevelsForDemonstrator")]
        pub random_battle_levels_for_demonstrator: BTreeMap<String, (u8, u8)>,
        // pub crystal_rewards_config: BTreeMap<String, bool>, LAZY
        pub eula_config: EulaConfig,
        pub player_subscriptions_config: PlayerSubscriptionsConfig,
        #[serde(rename = "reactiveCommunicationConfig")]
        pub reactive_communication_config: ReactiveCommunicationConfig,
        pub ranked_config: RankedConfig,
        pub fun_random_config: FunRandomConfig,
        #[serde(rename = "tournamentSettings")]
        pub tournament_settings: TournamentSettings,
        /// TODO: Special serialization from/to lists.
        #[serde(rename = "roaming")]
        pub roaming_settings: RoamingSettings,
        pub wgm_offline_emergency_config: WgmOfflineEmergencyConfig,
        /// XMPP enable.
        pub xmpp_enabled: bool,
        /// XMPP main connections (host, port).
        pub xmpp_connections: Vec<(String, u16)>,
        /// XMPP alternative connections (host, port).
        pub xmpp_alt_connections: Vec<(String, u16)>,
        /// XMPP bosh connections (host, port).
        pub xmpp_bosh_connections: Vec<(String, u16)>,
        /// Main XMPP host.
        pub xmpp_host: String,
        /// Main XMPP port.
        pub xmpp_port: u16,
        /// Main XMPP resource.
        pub xmpp_resource: String,
        /// XMPP muc services.
        pub xmpp_muc_services: Vec<XmppMucService>,
        /// Periphery routing configurations.
        pub periphery_routing_config: PeripheryRoutingConfig,
        // serial_number_generator_url: not used by the client
        pub progressive_reward_config: ProgressiveRewardConfig,
        #[serde(rename = "isAnonymizerEnabled")]
        pub is_anonymizer_enabled: bool,
        #[serde(rename = "isCrewSkinsEnabled")]
        pub is_crew_skins_enabled: bool,
        #[serde(rename = "isOffersEnabled")]
        pub is_offers_enabled: bool,
        pub collector_vehicle_config: CollectorVehicleConfig,
        #[serde(rename = "isPromoLoggingEnabled")]
        pub is_promo_logging_enabled: bool,
        #[serde(rename = "igbWhitelist")]
        pub igb_whitelist: IgbWhitelist,
        #[serde(rename = "isVehiclesCompareEnabled")]
        pub is_vehicles_compare_enabled: u8,
        #[serde(rename = "isPremiumInPostBattleEnabled")]
        pub is_premium_in_post_battle_enabled: bool,
        pub trade_in_config: TradeInConfig,
        pub comp7_prestige_ranks_config: Comp7PrestigeRanksConfig,
        #[serde(rename = "additionalBonus_config")]
        pub additional_bonus_config: AdditionalBonusConfig,
        #[serde(rename = "mapbox_config")]
        pub map_box_config: MapBoxConfig,
        #[serde(rename = "maxScoutInSquads")]
        pub max_scout_in_squads: u8,
        #[serde(rename = "customizationQuests")]
        pub customization_quests: BTreeMap<String, Vec<CustomizationQuest>>,
        pub blueprints_config: BlueprintsConfig,
        pub blueprints_convert_sale_config: BlueprintsConvertSaleConfig,
        #[serde(rename = "bootcampBonuses")]
        pub boot_camp_bonuses: BootCampBonuses,
        #[serde(rename = "premSquad_config")]
        pub prem_squad_config: PremSquadConfig,
        #[serde(rename = "preferredMaps_config")]
        pub preferred_maps_config: PreferredMapsConfig,
        #[serde(rename = "shortBootcampPercent")]
        pub short_boot_camp_percent: u8,
        #[serde(rename = "isRegularQuestEnabled")]
        pub is_regular_quest_enabled: bool,
        #[serde(rename = "isOnly10ModeEnabled")]
        pub is_only10_mode_enabled: bool,
        // disabledPersonalMissions: lazy
        #[serde(rename = "isManualEnabled")]
        pub is_manual_enabled: bool,
        pub battle_matters_config: BattleMattersConfig,
        #[serde(rename = "voipUserDomain")]
        pub voip_user_domain: String,
        #[serde(rename = "voipDomain")]
        pub voip_domain: String,
        #[serde(rename = "piggyBank_config")]
        pub piggy_bank_config: PiggyBankConfig,
        pub battle_royale_config: BattleRoyaleConfig,
        pub regional_settings: RegionalSettings,
        #[serde(rename = "isTutorialEnabled")]
        pub is_tutorial_enabled: bool,
        pub event_battles_config: EventBattlesConfig,
        pub misc_gui_settings: MiscGuiSettings,
        pub file_server: BTreeMap<String, FileServer>,
        pub seniority_awards_config: SeniorityAwardsConfig,
        #[serde(rename = "productsCatalog")]
        pub products_catalog: ProductsCatalog,
        #[serde(rename = "isBuyPotapovQuestTileEnabled")]
        pub is_buy_potapov_quest_tile_enabled: bool,
        pub comp7_config: Comp7Config,
        pub wgnp: Wgnp,
        #[serde(rename = "isMapsTrainingEnabled")]
        pub is_maps_training_enabled: bool,
        pub telecom_rentals_config: TelecomRentalsConfig,
        #[serde(rename = "isGoldFishEnabled")]
        pub is_gold_fish_enabled: bool,
        #[serde(rename = "isBootcampEnabled")]
        pub is_bootcamp_enabled: bool,
        #[serde(rename = "spgRedesignFeatures")]
        pub spg_redesign_features: SpgRedesignFeatures,
        #[serde(rename = "isEpicRandomAchievementsEnabled")]
        pub is_epic_random_achievements_enabled: bool,
        #[serde(rename = "isLegacyModeSelectorEnabled")]
        pub is_legacy_mode_selector_enabled: bool,
        pub renewable_subscription_config: RenewableSubscriptionConfig,
        pub ui_logging_config: UiLoggingConfig,
        #[serde(rename = "isCustomizationEnabled")]
        pub is_customization_enabled: bool,
        #[serde(rename = "isDeluxeDevicesEnabled")]
        pub is_deluxe_devices_enabled: bool,
        #[serde(rename = "isCrewBooksSaleEnabled")]
        pub is_crew_books_sale_enabled: bool,
        #[serde(rename = "maxScoutInSquadsLevels")]
        pub max_scout_in_squads_levels: Vec<u8>,
        /// TODO: Check if right type.
        #[serde(rename = "disabledCustomizations")]
        pub disabled_customizations: Vec<String>,
        #[serde(rename = "isPMBattleProgressEnabled")]
        pub is_pm_battle_progress_enabled: bool,
        #[serde(rename = "isSandboxEnabled")]
        pub is_sandbox_enabled: bool,
        #[serde(rename = "isLootBoxesEnabled")]
        pub is_loot_boxes_enabled: bool,
        pub wallet: Vec<bool>,
        #[serde(rename = "isBattleBoostersEnabled")]
        pub is_battle_boosters_enabled: bool,
        #[serde(rename = "isReferralProgramEnabled")]
        pub is_referral_program_enabled: bool,
        pub personal_reserves_config: PersonalReservesConfig,
        pub referral_program_config: BTreeMap<u8, ReferralProgramConfig>,
        #[serde(rename = "clanProfile")]
        pub clan_profile: ClanProfile,
        pub magnetic_auto_aim_config: MagneticAutoAimConfig,
        #[serde(rename = "isTrophyDevicesEnabled")]
        pub is_trophy_devices_enabled: bool,
        #[serde(rename = "isEpicRandomEnabled")]
        pub is_epic_random_enabled: bool,
        pub hero_vehicles: HeroVehicles,
        pub dog_tags_config: DogTagsConfig,
        #[serde(rename = "isFieldPostEnabled")]
        pub is_field_post_enabled: bool,
        pub resource_well_config: ResourceWellConfig,
        #[serde(rename = "isGlobalMapEnabled")]
        pub is_global_map_enabled: bool,
        #[serde(rename = "battlePass_config")]
        pub battle_pass_config: BattlePassConfig,
        #[serde(rename = "isBuyPotapovQuestSlotEnabled")]
        pub is_buy_potapov_quest_slot_enabled: bool,
        #[serde(rename = "lootBoxes_config")]
        pub loot_boxes_config: BTreeMap<u32, LootBoxesConfig>,
        #[serde(rename = "promoCutouts")]
        pub promo_cutouts: u32,
        #[serde(rename = "recertificationFormState")]
        pub recertification_form_state: Option<String>,
        #[serde(rename = "isCrewBooksPurchaseEnabled")]
        pub is_crew_books_purchase_enabled: bool,
        pub active_test_confirmation_config: ActiveTestConfirmationConfig,
        pub wgcg: Wgcg,
        pub daily_quests_config: DailyQuestsConfig,
        #[serde(rename = "hallOfFame")]
        pub hall_of_fame: HallOfFame,
        #[serde(rename = "isCrewBooksEnabled")]
        pub is_crew_books_enabled: bool,
        #[serde(rename = "isTrainingBattleEnabled")]
        pub is_training_battle_enabled: bool,
        // gifts_config: i'm lazy
        #[serde(rename = "premQuests_config")]
        pub prem_quests_config: PremQuestsConfig,
        // elen_settings: i'm lazy
        pub battle_notifier_config: BattleNotifierConfig,
        // advent_calendar_config: i'm lazy
        #[serde(rename = "isMultinationalVehiclesEnabled")]
        pub is_multinational_vehicles_enabled: bool,
        // unit_assembler_config: i'm lazy
        // telecom_config: i'm lazy
        #[serde(rename = "maxSPGinSquads")]
        pub max_sp_gin_squads: u8,
        #[serde(rename = "isEpicRandomMarkOfMasteryEnabled")]
        pub is_epic_random_mark_of_mastery_enabled: bool,
        // bonus_caps_override_config: i'm lazy
        // strongholdSettings: i'm lazy
        #[serde(rename = "isSpecBattleMgrEnabled")]
        pub is_spec_battle_mgr_enabled: bool,
        // clans_config: lazy
        #[serde(rename = "isCommandBattleEnabled")]
        pub is_command_battle_enabled: bool,
        // randomMapsForDemonstrator: lazy
        #[serde(rename = "isPreferredMapsEnabled")]
        pub is_preferred_maps_enabled: bool,
        #[serde(rename = "isNationChangeEnabled")]
        pub is_nation_change_enabled: bool,
        #[serde(rename = "isEpicRandomMarksOnGunEnabled")]
        pub is_epic_random_marks_on_gun_enabled: bool,
        #[serde(rename = "isPM2QuestEnabled")]
        pub is_pm2_quest_enabled: bool,
        #[serde(rename = "isTankmanRestoreEnabled")]
        pub is_tankman_restore_enabled: bool,
        // vehicle_post_progression_config: lazy
        
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct Shop {
        pub host_url: String,
        pub is_storage_enabled: bool,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct SessionStats {
        #[serde(rename = "isSessionStatsEnabled")]
        pub session_stats_enabled: bool,
        #[serde(rename = "isLinkWithHoFEnabled")]
        pub link_with_hall_of_fame_enable: bool,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct MarathonConfig {
        pub marathon_url: String,
        pub finish_sale_time: u64,
        pub reward_vehicle_url: String,
        pub reward_style_url_igb: String,
        pub reward_vehicle_url_igb: String,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct FrontlineSettings {
        pub is_epic_training_enabled: bool,
    }
    
    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct EpicConfig {
        #[serde(flatten)]
        pub base: BaseConfig,
        pub auto_start: bool,
        pub kick_locked_queue_size: u32,
        pub defeated_bonus_credits: u32,
        pub credits_max: u32,
        pub winner_bonus_credits: u32,
        pub maps: Vec<String>,
        pub url: String,
        pub credits_modifier: f32,
        pub kick_locked_fraction: f32,
        #[serde(rename = "unlockableInBattleVehLevels")]
        pub unlockable_in_battle_vehicle_levels: Vec<u8>,
        pub max_time_in_queue: u32,
        pub valid_vehicle_levels: Vec<u8>,
        pub max_battles_for_same_team: f32,
        pub max_queue_size: u32,
        pub rent_vehicles: Vec<String>,
        pub credits_min_threshold: u32,
        pub seasons: BTreeMap<String, Season>,
        pub epic_meta_game: EpicMetaGame,
        pub battle_pass_data_enabled: bool,
        #[serde(rename = "arenaTypeIDs")]
        pub arena_type_ids: Vec<u32>,
    }
    
    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct EpicMetaGame {
        pub slots: TankClassMap<(u8, u8, u8)>,
        pub max_combat_reserve_level: u8,
        pub in_battle_reserves_by_rank: EpicMetaGameReserves,
        pub max_battle_duration: u32,
        pub default_slots: TankClassMap<u8>,
        pub rewards: EpicMetaGameRewards,
        pub meta_level: EpicMetaGameMetaLevel,
        pub skip_params_validation: u8,
        pub default_reserves: TankClassMap<(u8, u8, u8)>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct EpicMetaGameReserves {
        pub ammo_levels: TankClassMap<Vec<(u8, u8, u8)>>,
        pub slot_actions: TankClassMap<Vec<(u8,)>>,
        pub slots_by_rank: TankClassMap<Vec<Vec<u8>>>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct EpicMetaGameRewards {
        pub combat_reserves: BTreeMap<u8, EpicMetaGameReward>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct EpicMetaGameReward {
        pub tags: Vec<String>,
        pub price: u8,
        pub levels: Vec<String>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct EpicMetaGameMetaLevel {
        pub max_level: u8,
        pub fame_pts_to_progress: Vec<u16>,
        pub ability_points_for_level: Vec<u8>,
        pub fame_pts_by_rank: BTreeMap<u8, u16>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct EnhancementsConfig {
        pub enabled: bool,
        pub version: u8,
        pub slots: u32,
        pub host: String,
        pub url: String,
        #[serde(rename = "forbiddenVehTypes")]
        pub forbidden_vehicle_types: BTreeSet<String>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct EulaConfig {
        pub enabled: bool,
        pub steam_acc_enabled: bool,
        pub demo_acc_enabled: bool,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct PlayerSubscriptionsConfig {
        pub enabled: bool,
        pub hide_entrypoint: bool,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct ReactiveCommunicationConfig {
        pub is_enabled: bool,
        pub url: String,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct RankedConfig {
        #[serde(flatten)]
        pub base: BaseSeasonConfig,
        pub year_rating_page_url: String,
        pub shop_state: String,
        pub arena_time: u32,
        pub unburnable_ranks: BTreeSet<u16>,
        pub bonus_battles_multiplier: u16,
        pub divisions: BTreeMap<u8, RankedConfigDivision>,
        pub year_reward_state: String,
        #[serde(rename = "minXP")]
        pub min_xp: u16,
        pub shop_page_url: String,
        pub num_players: u16,
        pub unburnable_step_ranks: BTreeSet<u16>,
        pub min_level: u16,
        pub winner_rank_changes: Vec<u16>,
        pub forbidden_class_tags: BTreeSet<String>,
        pub acc_ranks: u16,
        pub intro_page_url: String,
        pub season_gap_page_url: String,
        pub year_l_b_size: u16,
        pub year_l_b_state: String,
        pub expire_seasons: u16,
        pub efficiency_groups: BTreeMap<u16, Vec<f32>>,
        pub loser_rank_changes: Vec<i8>,
        pub max_vehicles: TankRankedClassMap<u8>,
        pub mm_fail_times: Vec<u16>,
        pub cycle_finish_seconds: u16,
        pub expected_seasons: u16,
        pub max_players_in_queue: u16,
        pub qualification_battles: u16,
        pub archivate_after: u64,
        pub rank_groups: Vec<u8>,
        #[serde(rename = "forbiddenVehTypes")]
        pub forbidden_vehicle_types: BTreeSet<u32>,
        pub info_page_url: String,
        pub qualification_bonus_battles: Vec<RankedConfigBonusBattle>,
        pub max_time_in_queue: u16,
        pub acc_steps: Vec<u8>,
        pub maps: BTreeSet<(u8, u8)>,
        pub season_rating_page_url: String,
        pub template: TankRankedClassMap<u8>,
        pub leagues_bonus_battles: Vec<RankedConfigBonusBattle>,
        pub has_special_season: bool,
        pub year_awards_marks: Vec<u8>,
        pub imbalance: TankRankedClassMap<u16>,
        pub max_level: u8,
        pub shields: BTreeMap<u8, (u8, u8)>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct RankedConfigDivision {
        pub start_rank: u8,
        pub bonus_battles: RankedConfigBonusBattles,
        pub is_league: bool,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum RankedConfigBonusBattles {
        Some(Vec<RankedConfigBonusBattlesEntry>),
        None(BTreeMap<(), ()>),
    }

    impl Default for RankedConfigBonusBattles {
        fn default() -> Self {
            Self::None(BTreeMap::new())
        }
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct RankedConfigBonusBattlesEntry {
        pub efficiency: f32,
        pub battles_count: u8,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct RankedConfigBonusBattle {
        pub steps: u8,
        pub battles_count: u8,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct FunRandomConfig {
        pub is_enabled: bool,
        pub meta_progression: FunRandomConfigMetaProgression,
        pub info_page_url: String,
        /// TODO: Check if types are valid.
        pub events: BTreeMap<String, String>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct FunRandomConfigMetaProgression {
        pub is_enabled: bool,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct TournamentSettings {
        pub is_tournament_enabled: bool,
        pub is_external_battle_enabled: bool,
        pub tms_host_url: String,
    }
    
    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct RoamingSettings {
        pub home_center_id: u8,
        pub cur_center_id: u8,
        pub servers: Vec<ServerInfo>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct WgmOfflineEmergencyConfig {
        pub enabled: bool,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct PeripheryRoutingConfig {
        pub is_enabled: bool,
        pub periphery_routing_groups: BTreeMap<String, Vec<u16>>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct ProgressiveRewardConfig {
        pub is_enabled: bool,
        pub max_level: u8,
        #[serde(rename = "probabilityTokenID")]
        pub probability_token_id: String,
        #[serde(rename = "levelTokenID")]
        pub level_token_id: String,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct CollectorVehicleConfig {
        pub enabled: bool,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct IgbWhitelist {
        pub is_enabled: bool,
        pub whitelist_url: String,
        pub default_whitelist: BTreeSet<String>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct TradeInConfig {
        pub vehicle_groups: BTreeMap<String, String>,
        pub conversion_rules: BTreeMap<String, String>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct Comp7PrestigeRanksConfig {
        pub ranks: BTreeSet<String>,
        pub divisions: Vec<Comp7Division>,
        pub divisions_by_rank: BTreeMap<u8, Vec<Comp7Division>>,
        pub rank_kinds: BTreeSet<String>,
        pub ranks_order: Vec<String>,
        pub elite_rank_percent: u8,
    }
    
    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct Comp7Division {
        pub index: u8,
        pub rank: u8,
        pub id: u16,
        pub name: String,
        pub tags: BTreeSet<String>,
        pub range: (u32, u32),
        #[serde(default)]
        pub reward_tokens: Option<BTreeMap<String, String>>,
        #[serde(default)]
        pub rank_inactivity: Option<BTreeMap<String, u16>>,
        #[serde(default)]
        pub has_rank_inactivity: bool,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct AdditionalBonusConfig {
        pub enabled: bool,
        pub apply_count: u16,
        pub bonus_factor: f32,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct MapBoxConfig {
        #[serde(flatten)]
        pub base: BaseSeasonConfig,
        pub vehicle_soft_fail_timeouts: TankClassMap<Vec<u8>>,
        #[serde(rename = "geometryIDs")]
        pub geometry_ids: BTreeMap<StringOrInteger, BTreeSet<u16>>,
        #[serde(rename = "gameplayIDs")]
        pub gameplay_ids: BTreeMap<StringOrInteger, BTreeSet<u16>>,
        pub progression_update_interval: u16,
        pub squad_timeouts: Vec<u16>,
        #[serde(rename = "forbiddenVehTypes")]
        pub forbidden_vehicle_types: BTreeSet<String>,
        pub forbidden_class_tags: BTreeSet<String>,
        pub battle_type_timeout: u16,
        pub levels: Vec<u8>,
        pub template: TankClassMap<u8>,
        #[serde(rename = "balanceVehTypes")]
        pub balance_vehicle_types: Vec<String>,
        pub level_type_weights: Vec<(u8, u8, u8)>,
        pub max_players_in_team: u16,
        pub vehicle_timeouts: Vec<u16>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct CustomizationQuest {
        pub finish_time: u64,
        pub quest_ids: BTreeMap<u8, Vec<String>>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct XmppMucService {
        pub enabled: bool,
        pub r#type: u8,
        pub hostname: String,
        pub user_string: String,
        pub format: String,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct BlueprintsConfig {
        pub is_enabled: bool,
        pub use_blueprints_for_unlock: bool,
        pub allow_blueprints_conversion: bool,
        pub levels: BTreeMap<u8, BlueprintsConfigLevel>
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct BlueprintsConfigLevel(
        u8,
        f32,
        (u8, u8), 
        (f32, f32),
        BTreeMap<u8, BTreeMap<u8, f32>>,
    );

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct BootCampBonuses {
        pub dossier: BTreeMap<u8, BTreeMap<(String, String), BootCampBonus>>,
        pub premium_plus: u8,
        pub gold: u16,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct BootCampBonus {
        pub r#type: String,
        pub value: u8,
        pub unique: bool,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct PremSquadConfig {
        pub enabled: bool,
        pub credits_factor: BTreeMap<String, f32>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct PreferredMapsConfig {
        pub slot_cooldown: u64,
        pub premium_slots: u8,
        pub default_slots: u8,
        #[serde(rename = "mapIDs")]
        pub map_ids: BTreeSet<u16>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct BattleMattersConfig {
        pub is_enabled: bool,
        pub is_paused: bool,
        pub delayed_reward_offer_visibility_token: String,
        pub delayed_reward_offer_currency_token: String,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct PiggyBankConfig {
        pub enabled: bool,
        pub cycle_length: u32,
        pub cycle_start_time: u64,
        pub open_soon_threshold: u32,
        pub credits_threshold: u32,
        pub multiplier: f32,
        pub arena: Vec<u8>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct BlueprintsConvertSaleConfig {
        pub enabled: u8,
        /// TODO: Check in the future if this value type is sufficient.
        pub options: BTreeMap<String, String>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct BattleRoyaleConfig {
        #[serde(flatten)]
        pub base: BaseSeasonConfig,
        pub spawn: BTreeMap<u16, BattleRoyaleSpawn>,
        pub arena: BattleRoyaleArena,
        // coneVisibility: Seems unused in client.
        pub default_ammo: Vec<(String, (f32,))>,
        pub url: String,
        pub economics: BTreeMap<u16, BattleRoyaleEconomic>,
        pub vehicles_slots_config: BTreeMap<String, BattleRoyaleSlotsConfig>,
        pub in_battle_upgrades: BattleRoyaleUpgrades,
        #[serde(rename = "battleXP")]
        pub battle_xp: BattleRoyaleXp,
    }
    
    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct BattleRoyaleSpawn {
        pub spawn_key_points_per_sector: u16,
        pub spawn_sectors_amount: u16,
        pub placement_strategies: Vec<String>,
        pub spawn_key_points_choose_time: u16,
    }
    
    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct BattleRoyaleArena {
        #[serde(rename = "typeIDs")]
        pub type_ids: Vec<u16>,
        pub max_battle_duration: u16,
        #[serde(flatten)]
        pub entries: BTreeMap<u16, BattleRoyaleArenaEntry>,
    }
    
    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct BattleRoyaleArenaEntry {
        pub max_teams_in_arena: u16,
        pub max_players_in_team: u8,
        pub enable_advanced123_protection: bool,
        pub max_time_in_queue: u16,
        pub min_queue_size: Vec<(u16, u16)>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct BattleRoyaleEconomic {
        pub rent: BattleRoyaleEconomicRent,
        pub test_drive: f32,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct BattleRoyaleEconomicRent {
        pub currency: String,
        pub amount: u16,
        pub time: f32,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct BattleRoyaleSlotsConfig {
        pub charge1: u16,
        pub charge2: u16,
        pub charge3: u16,
        pub charge4: u16,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct BattleRoyaleUpgrades {
        pub combating_cooldown: f32,
        pub settling_cooldown: f32,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct BattleRoyaleXp {
        pub xp_to_lvl: Vec<u16>,
        pub player: BattleRoyaleXpPlayer,
        /// TODO: Check if K/V types are correct.
        pub loot: BTreeMap<String, String>,
        pub bot: BattleRoyaleXpPlayer,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct BattleRoyaleXpPlayer {
        pub xp_for_hit: u16,
        pub xp_for_kill: u16,
        pub xp_for_damage_coef: f32,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct RegionalSettings {
        pub starting_time_of_a_new_game_day: u32,
        pub starting_time_of_a_new_day: u32,
        pub starting_day_of_a_new_week: u32,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct EventBattlesConfig {
        #[serde(flatten)]
        pub base: BaseSeasonConfig,
        pub maps: Vec<u16>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct MiscGuiSettings {
        pub buy_module_dialog: MiscGuiBuyModuleDialog,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct MiscGuiBuyModuleDialog {
        pub enable_auto_sell_check_box: bool,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct FileServer {
        pub url_template: String,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct SeniorityAwardsConfig {
        pub enabled: bool,
        pub reward_eligibility_token: String,
        pub received_rewards_token: String,
        pub claim_reward_token: String,
        pub reward_quests_prefix: String,
        pub show_reward_notification: bool,
        pub clock_on_notification: u16,
        pub end_time: u64,
        pub reminders: Vec<u64>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct ProductsCatalog {
        pub url: String,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct Comp7Config {
        pub is_enabled: bool,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct Wgnp {
        pub enabled: bool,
        pub rename_api_enabled: bool,
        pub url: String,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct TelecomRentalsConfig {
        pub enabled: bool,
        pub allowed_bundles: Vec<u8>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct SpgRedesignFeatures {
        pub stun_enabled: bool,
        pub mark_target_area_enabled: bool,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct RenewableSubscriptionConfig {
        pub enabled: bool,
        pub enable_gold_reserve: bool,
        pub enable_free_directives: bool,
        #[serde(rename = "crewXPPerMinute")]
        pub crew_xp_per_minute: f32,
        #[serde(rename = "enablePassiveCrewXP")]
        pub enable_passive_crew_xp: bool,
        pub enable_new_subscriptions: bool,
        pub enable_tank_rental: bool,
        pub max_gold_reserve_capacity: u16,
        pub gold_reserve_gains_per_battle_type: BTreeMap<u16, RenewableSubscriptionReserveGains>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct RenewableSubscriptionReserveGains {
        pub win: u16,
        pub draw: u16,
        pub loss: u16,
        pub min_top: u16,
        pub min_level: u16,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct UiLoggingConfig {
        pub enabled: bool,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct PersonalReservesConfig {
        pub display_conversion_notification: bool,
        pub is_reserves_in_battle_activation_enabled: bool,
        pub supported_queue_types: BTreeMap<String, BTreeSet<u16>>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct ReferralProgramConfig {
        pub vehicle_levels: BTreeSet<u8>,
        pub progression_level: u8,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct ClanProfile {
        pub is_enabled: bool,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct MagneticAutoAimConfig {
        pub enable_for_tags: BTreeSet<String>,
        // pub enabledForTags: ... deprecated
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct HeroVehicles {
        pub is_enabled: bool,
        pub vehicles: BTreeMap<u32, HeroVehiclesEntry>
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct HeroVehiclesEntry {
        pub url: String,
        pub shop_url: String,
        pub name: String,
        pub crew: String,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct DogTagsConfig {
        pub enabled: bool,
        pub enable_dog_tags_in_battle: bool,
        pub enable_dog_tags_customization_screen: bool,
        pub enable_dog_tags_in_post_battle: bool,
        pub enable_skill_components: bool,
        pub enable_component_unlocking: bool,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct ResourceWellConfig {
        pub is_enabled: bool,
        pub start_time: u64,
        pub finish_time: u64,
        pub remind_time: u64,
        pub season: u8,
        pub points: u16,
        /// TODO: Check K/V types.
        pub resources: BTreeMap<String, String>,
    }

    /// TODO: Check 'BattlePassConfig' in python's code.
    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct BattlePassConfig {
        /// None or Some("disabled")
        pub mode: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct LootBoxesConfig {
        pub category: String,
        pub history_name: StringOrInteger,
        pub r#type: String,
        pub enabled: bool,
        pub id: u32,
        pub bonus: BTreeMap<String, String>,
        pub guaranteed_frequency: LootBoxesGuaranteedFrequency,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct LootBoxesGuaranteedFrequency {
        pub guaranteed_reward: Option<LootBoxesGuaranteedReward>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct LootBoxesGuaranteedReward {
        pub is_for_players: bool,
        pub guaranteed_frequency: u8,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct ActiveTestConfirmationConfig {
        pub enabled: bool,
        pub link: String,
        pub start_time: u64,
        pub finish_time: u64,
        #[serde(rename = "peripheryIDs")]
        pub periphery_ids: HashSet<u16>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct Wgcg {
        pub is_enabled: bool,
        pub gate_url: String,
        pub is_jwt_authorization_enabled: bool,
        pub login_on_start: bool,
        pub is_white_list_enabled: bool,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct DailyQuestsConfig {
        pub enabled: bool,
        pub reroll_enabled: bool,
        pub reroll_timeout: u32,
        pub epic_reward_enabled: bool,
        pub epic_reward_needs_token: u16,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct HallOfFame {
        pub hof_host_url: String,
        pub is_status_enabled: bool,
        pub is_hof_enabled: bool,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct PremQuestsConfig {
        pub enabled: bool,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct BattleNotifierConfig {
        pub enabled: bool,
    }

    /// Intended for generic usage.
    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct TankClassMap<V> {
        pub heavy_tank: V,
        pub medium_tank: V,
        pub light_tank: V,
        #[serde(rename = "AT-SPG")]
        pub at_spg: V,
        #[serde(rename = "SPG")]
        pub spg: V,
        /// Used sometimes for wheeled vehicles, instead of putting them 
        /// in `light_tank`.
        pub wheeled_vehicle: Option<V>,
    }

    /// Intended for generic usage.
    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct TankRankedClassMap<V> {
        pub heavy_tank: V,
        pub medium_tank: V,
        pub wheeled_vehicle: V,
        pub scout: V,
        #[serde(rename = "AT-SPG")]
        pub at_spg: V,
        #[serde(rename = "SPG")]
        pub spg: V,
    }

    /// Intended for generic usage.
    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct ServerInfo {
        pub center_id: u8,
        pub db_id_min: u32,
        pub db_id_max: u32,
        pub region_code: String,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct BaseConfig {
        pub is_enabled: bool,
        #[serde(rename = "peripheryIDs")]
        pub periphery_ids: BTreeSet<u16>,
    }

    /// Base configuration for season-based configurations.
    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct BaseSeasonConfig {
        #[serde(flatten)]
        pub base: BaseConfig,
        pub seasons: BTreeMap<u32, Season>,
        pub prime_times: BTreeMap<u16, PrimeTime>,
        /// TODO:
        pub cycle_times: Vec<bool>,
    }

    /// Intended for generic usage.
    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct PrimeTime {
        pub start: (u8, u8),
        pub end: (u8, u8),
        pub weekdays: BTreeSet<u8>,
        #[serde(rename = "peripheryIDs")]
        pub periphery_ids: BTreeSet<u16>,
    }

    /// Intended for generic usage.
    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct Season {
        pub start_season: u64,
        pub end_season: u64,
        pub number: u16,
        pub cycles: BTreeMap<u32, SeasonCycle>
    }

    /// Intended for generic usage.
    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct SeasonCycle {
        pub end: u64,
        pub start: u64,
    }
    
    /// Intended for generic usage as untagged map key.
    #[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
    #[serde(untagged)]
    pub enum StringOrInteger {
        String(String),
        Int(u32),
    }

    impl Default for StringOrInteger {
        fn default() -> Self {
            Self::Int(0)
        }
    }

}
