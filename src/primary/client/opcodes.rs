#[non_exhaustive]
pub struct Opcode;

#[allow(dead_code)]
impl Opcode {
    pub const LOGIN_CHALLENGE: u8 = 0;
    pub const LOGIN_PROOF: u8 = 1;
    pub const REALM_LIST: u8 = 16;

    pub const CMSG_CHAR_CREATE: u32 = 54;
    pub const CMSG_CHAR_ENUM: u32 = 55;
    pub const CMSG_PLAYER_LOGIN: u32 = 61;
    pub const CMSG_LOGOUT_REQUEST: u32 = 75;
    pub const CMSG_NAME_QUERY: u32 = 80;
    pub const CMSG_ITEM_QUERY_SINGLE: u32 = 86;
    pub const CMSG_GAMEOBJECT_QUERY: u32 = 94;
    pub const CMSG_CREATURE_QUERY: u32 = 96;
    pub const CMSG_GROUP_INVITE: u32 = 110;
    pub const CMSG_GROUP_ACCEPT: u32 = 114;
    pub const CMSG_GROUP_DECLINE: u32 = 115;
    pub const CMSG_MESSAGECHAT: u32 = 149;
    pub const CMSG_JOIN_CHANNEL: u32 = 151;
    pub const CMSG_USE_ITEM: u32 = 171;
    pub const CMSG_OPEN_ITEM: u32 = 172;
    pub const CMSG_GAMEOBJ_USE: u32 = 177;
    pub const CMSG_EMOTE: u32 = 258;
    pub const CMSG_TEXT_EMOTE: u32 = 260;
    pub const CMSG_AUTOSTORE_LOOT_ITEM: u32 = 264;
    pub const CMSG_DESTROYITEM: u32 = 273;
    pub const CMSG_INITIATE_TRADE: u32 = 278;
    pub const CMSG_BEGIN_TRADE: u32 = 279;
    pub const CMSG_SET_TRADE_GOLD: u32 = 287;
    pub const CMSG_CAST_SPELL: u32 = 302;
    pub const CMSG_CANCEL_CAST: u32 = 303;
    pub const CMSG_SET_SELECTION: u32 = 317;
    pub const CMSG_LOOT_RELEASE: u32 = 351;
    pub const CMSG_SELL_ITEM: u32 = 416;
    pub const CMSG_BUY_ITEM: u32 = 418;
    pub const CMSG_PLAYED_TIME: u32 = 460;
    pub const CMSG_PING: u32 = 476;
    pub const CMSG_SETSHEATHED: u32 = 480;
    pub const CMSG_AUTH_SESSION: u32 = 493;
    pub const CMSG_SEND_MAIL: u32 = 568;
    pub const CMSG_GET_MAIL_LIST: u32 = 570;
    pub const CMSG_WARDEN_DATA: u32 = 743;
    pub const CMSG_REALM_SPLIT: u32 = 908;
    pub const CMSG_QUESTGIVER_STATUS_MULTIPLE_QUERY: u32 = 1047;
    pub const CMSG_GAMEOBJ_REPORT_USE: u32 = 1153;
    pub const CMSG_READY_FOR_ACCOUNT_DATA_TIMES: u32 = 1279;

    pub const SMSG_CHAR_ENUM: u16 = 59;
    pub const SMSG_LOGIN_SETTIMESPEED: u16 = 66;
    pub const SMSG_NAME_QUERY_RESPONSE: u16 = 81;
    pub const SMSG_ITEM_QUERY_SINGLE_RESPONSE: u16 = 88;
    pub const SMSG_GAMEOBJECT_QUERY_RESPONSE: u16 = 95;
    pub const SMSG_CREATURE_QUERY_RESPONSE: u16 = 97;
    pub const SMSG_GROUP_INVITE: u16 = 111;
    pub const SMSG_GROUP_LIST: u16 = 125;
    pub const SMSG_MESSAGECHAT: u16 = 150;
    pub const SMSG_UPDATE_OBJECT: u16 = 169;
    pub const SMSG_DESTROY_OBJECT: u16 = 170;
    pub const SMSG_GAMEOBJECT_CUSTOM_ANIM: u16 = 179;
    pub const SMSG_MONSTER_MOVE: u16 = 221;
    pub const SMSG_TUTORIAL_FLAGS: u16 = 253;
    pub const SMSG_EMOTE: u16 = 259;
    pub const SMSG_TEXT_EMOTE: u16 = 261;
    pub const SMSG_INVENTORY_CHANGE_FAILURE: u16 = 274;
    pub const SMSG_SET_PROFICIENCY: u16 = 295;
    pub const SMSG_INITIAL_SPELLS: u16 = 298;
    pub const SMSG_CAST_RESULT: u16 = 304;
    pub const SMSG_SPELL_START: u16 = 305;
    pub const SMSG_SPELL_GO: u16 = 306;
    pub const SMSG_SPELL_FAILURE: u16 = 307;
    pub const SMSG_CANCEL_COMBAT: u16 = 334;
    pub const SMSG_SPELLHEALLOG: u16 = 336;
    pub const SMSG_BINDPOINTUPDATE: u16 = 341;
    pub const SMSG_LOOT_RESPONSE: u16 = 352;
    pub const SMSG_LOOT_RELEASE_RESPONSE: u16 = 353;
    pub const SMSG_LOOT_REMOVED: u16 = 354;
    pub const SMSG_ITEM_PUSH_RESULT: u16 = 358;
    pub const SMSG_BUY_ITEM: u16 = 420;
    pub const SMSG_FISH_NOT_HOOKED: u16 = 456;
    pub const SMSG_PONG: u16 = 477;
    pub const SMSG_SPELL_DELAYED: u16 = 482;
    pub const SMSG_AUTH_CHALLENGE: u16 = 492;
    pub const SMSG_AUTH_RESPONSE: u16 = 494;
    pub const SMSG_COMPRESSED_UPDATE_OBJECT: u16 = 502;
    pub const SMSG_ACCOUNT_DATA_TIMES: u16 = 521;
    pub const SMSG_GAMEOBJECT_DESPAWN_ANIM: u16 = 533;
    pub const SMSG_LOGIN_VERIFY_WORLD: u16 = 566;
    pub const SMSG_SEND_MAIL_RESULT: u16 = 569;
    pub const SMSG_MAIL_LIST_RESULT: u16 = 571;
    pub const SMSG_SPELLLOGEXECUTE: u16 = 588;
    pub const SMSG_SET_PCT_SPELL_MODIFIER: u16 = 615;
    pub const SMSG_SET_FORCED_REACTIONS: u16 = 677;
    pub const SMSG_SPELL_FAILED_OTHER: u16 = 678;
    pub const SMSG_WARDEN_DATA: u16 = 742;
    pub const SMSG_ADDON_INFO: u16 = 751;
    pub const SMSG_EXPECTED_SPAM_RECORDS: u16 = 818;
    pub const SMSG_MOTD: u16 = 829;
    pub const SMSG_REALM_SPLIT: u16 = 907;
    pub const SMSG_FEATURE_SYSTEM_STATUS: u16 = 969;
    pub const SMSG_QUESTGIVER_STATUS_MULTIPLE: u16 = 1048;
    pub const SMSG_ACHIEVEMENT_EARNED: u16 = 1128;
    pub const SMSG_CRITERIA_UPDATE: u16 = 1130;
    pub const SMSG_SET_PHASE_SHIFT: u16 = 1148;
    pub const SMSG_POWER_UPDATE: u16 = 1152;
    pub const SMSG_AURA_UPDATE_ALL: u16 = 1173;
    pub const SMSG_AURA_UPDATE: u16 = 1174;
    pub const SMSG_CLIENTCACHE_VERSION: u16 = 1195;
    pub const SMSG_TALENT_UPDATE: u16 = 1216;

    pub const MSG_MOVE_START_FORWARD: u16 = 181;
    pub const MSG_MOVE_START_BACKWARD: u16 = 182;
    pub const MSG_MOVE_STOP: u16 = 183;
    pub const MSG_MOVE_START_STRAFE_LEFT: u16 = 184;
    pub const MSG_MOVE_START_STRAFE_RIGHT: u16 = 185;
    pub const MSG_MOVE_STOP_STRAFE: u16 = 186;
    pub const MSG_MOVE_JUMP: u16 = 187;
    pub const MSG_MOVE_START_TURN_LEFT: u16 = 188;
    pub const MSG_MOVE_START_TURN_RIGHT: u16 = 189;
    pub const MSG_MOVE_STOP_TURN: u16 = 190;
    pub const MSG_MOVE_START_PITCH_UP: u16 = 191;
    pub const MSG_MOVE_START_PITCH_DOWN: u16 = 192;
    pub const MSG_MOVE_STOP_PITCH: u16 = 193;
    pub const MSG_MOVE_FALL_LAND: u16 = 201;
    pub const MSG_MOVE_START_SWIM: u16 = 202;
    pub const MSG_MOVE_STOP_SWIM: u16 = 203;
    pub const MSG_MOVE_SET_FACING: u16 = 218;
    pub const MSG_MOVE_SET_PITCH: u16 = 219;
    pub const MSG_MOVE_WORLDPORT_ACK: u16 = 220;
    pub const MSG_MOVE_HEARTBEAT: u16 = 238;
    pub const MSG_SET_DUNGEON_DIFFICULTY: u16 = 809;

    pub fn get_client_opcode_name(opcode: u32) -> String {
        if (opcode as u8) == Opcode::LOGIN_CHALLENGE {
            return String::from("LOGIN_CHALLENGE");
        } else if (opcode as u8) == Opcode::LOGIN_PROOF {
            return String::from("LOGIN_PROOF");
        } else if (opcode as u8) == Opcode::REALM_LIST {
            return String::from("REALM_LIST");
        }

        match opcode {
            Opcode::CMSG_CHAR_ENUM => String::from("CMSG_CHAR_ENUM"),
            Opcode::CMSG_PLAYER_LOGIN => String::from("CMSG_PLAYER_LOGIN"),
            Opcode::CMSG_LOGOUT_REQUEST => String::from("CMSG_LOGOUT_REQUEST"),
            Opcode::CMSG_NAME_QUERY => String::from("CMSG_NAME_QUERY"),
            Opcode::CMSG_ITEM_QUERY_SINGLE => String::from("CMSG_ITEM_QUERY_SINGLE"),
            Opcode::CMSG_GAMEOBJECT_QUERY => String::from("CMSG_GAMEOBJECT_QUERY"),
            Opcode::CMSG_CREATURE_QUERY => String::from("CMSG_CREATURE_QUERY"),
            Opcode::CMSG_GROUP_INVITE => String::from("CMSG_GROUP_INVITE"),
            Opcode::CMSG_GROUP_ACCEPT => String::from("CMSG_GROUP_ACCEPT"),
            Opcode::CMSG_GROUP_DECLINE => String::from("CMSG_GROUP_DECLINE"),
            Opcode::CMSG_MESSAGECHAT => String::from("CMSG_MESSAGECHAT"),
            Opcode::CMSG_JOIN_CHANNEL => String::from("CMSG_JOIN_CHANNEL"),
            Opcode::CMSG_USE_ITEM => String::from("CMSG_USE_ITEM"),
            Opcode::CMSG_OPEN_ITEM => String::from("CMSG_OPEN_ITEM"),
            Opcode::CMSG_GAMEOBJ_USE => String::from("CMSG_GAMEOBJ_USE"),
            Opcode::CMSG_EMOTE => String::from("CMSG_EMOTE"),
            Opcode::CMSG_TEXT_EMOTE => String::from("CMSG_TEXT_EMOTE"),
            Opcode::CMSG_AUTOSTORE_LOOT_ITEM => String::from("CMSG_AUTOSTORE_LOOT_ITEM"),
            Opcode::CMSG_DESTROYITEM => String::from("CMSG_DESTROYITEM"),
            Opcode::CMSG_INITIATE_TRADE => String::from("CMSG_INITIATE_TRADE"),
            Opcode::CMSG_BEGIN_TRADE => String::from("CMSG_BEGIN_TRADE"),
            Opcode::CMSG_SET_TRADE_GOLD => String::from("CMSG_SET_TRADE_GOLD"),
            Opcode::CMSG_CAST_SPELL => String::from("CMSG_CAST_SPELL"),
            Opcode::CMSG_CANCEL_CAST => String::from("CMSG_CANCEL_CAST"),
            Opcode::CMSG_SET_SELECTION => String::from("CMSG_SET_SELECTION"),
            Opcode::CMSG_LOOT_RELEASE => String::from("CMSG_LOOT_RELEASE"),
            Opcode::CMSG_SELL_ITEM => String::from("CMSG_SELL_ITEM"),
            Opcode::CMSG_BUY_ITEM => String::from("CMSG_BUY_ITEM"),
            Opcode::CMSG_PLAYED_TIME => String::from("CMSG_PLAYED_TIME"),
            Opcode::CMSG_PING => String::from("CMSG_PING"),
            Opcode::CMSG_SETSHEATHED => String::from("CMSG_SETSHEATHED"),
            Opcode::CMSG_AUTH_SESSION => String::from("CMSG_AUTH_SESSION"),
            Opcode::CMSG_SEND_MAIL => String::from("CMSG_SEND_MAIL"),
            Opcode::CMSG_GET_MAIL_LIST => String::from("CMSG_GET_MAIL_LIST"),
            Opcode::CMSG_WARDEN_DATA => String::from("CMSG_WARDEN_DATA"),
            Opcode::CMSG_REALM_SPLIT => String::from("CMSG_REALM_SPLIT"),
            Opcode::CMSG_QUESTGIVER_STATUS_MULTIPLE_QUERY => String::from("CMSG_QUESTGIVER_STATUS_MULTIPLE_QUERY"),
            Opcode::CMSG_GAMEOBJ_REPORT_USE => String::from("CMSG_GAMEOBJ_REPORT_USE"),
            Opcode::CMSG_READY_FOR_ACCOUNT_DATA_TIMES => String::from("CMSG_READY_FOR_ACCOUNT_DATA_TIMES"),
            _ => format!("{}", opcode),
        }
    }

    pub fn get_server_opcode_name(opcode: u16) -> String {
        if (opcode as u8) == Opcode::LOGIN_CHALLENGE {
            return String::from("LOGIN_CHALLENGE");
        } else if (opcode as u8) == Opcode::LOGIN_PROOF {
            return String::from("LOGIN_PROOF");
        } else if (opcode as u8) == Opcode::REALM_LIST {
            return String::from("REALM_LIST");
        }

        match opcode {
            Opcode::SMSG_CHAR_ENUM => String::from("SMSG_CHAR_ENUM"),
            Opcode::SMSG_LOGIN_SETTIMESPEED => String::from("SMSG_LOGIN_SETTIMESPEED"),
            Opcode::SMSG_NAME_QUERY_RESPONSE => String::from("SMSG_NAME_QUERY_RESPONSE"),
            Opcode::SMSG_ITEM_QUERY_SINGLE_RESPONSE => String::from("SMSG_ITEM_QUERY_SINGLE_RESPONSE"),
            Opcode::SMSG_GAMEOBJECT_QUERY_RESPONSE => String::from("SMSG_GAMEOBJECT_QUERY_RESPONSE"),
            Opcode::SMSG_CREATURE_QUERY_RESPONSE => String::from("SMSG_CREATURE_QUERY_RESPONSE"),
            Opcode::SMSG_GROUP_INVITE => String::from("SMSG_GROUP_INVITE"),
            Opcode::SMSG_GROUP_LIST => String::from("SMSG_GROUP_LIST"),
            Opcode::SMSG_MESSAGECHAT => String::from("SMSG_MESSAGECHAT"),
            Opcode::SMSG_UPDATE_OBJECT => String::from("SMSG_UPDATE_OBJECT"),
            Opcode::SMSG_DESTROY_OBJECT => String::from("SMSG_DESTROY_OBJECT"),
            Opcode::SMSG_GAMEOBJECT_CUSTOM_ANIM => String::from("SMSG_GAMEOBJECT_CUSTOM_ANIM"),
            Opcode::SMSG_MONSTER_MOVE => String::from("SMSG_MONSTER_MOVE"),
            Opcode::SMSG_TUTORIAL_FLAGS => String::from("SMSG_TUTORIAL_FLAGS"),
            Opcode::SMSG_EMOTE => String::from("SMSG_EMOTE"),
            Opcode::SMSG_TEXT_EMOTE => String::from("SMSG_TEXT_EMOTE"),
            Opcode::SMSG_INVENTORY_CHANGE_FAILURE => String::from("SMSG_INVENTORY_CHANGE_FAILURE"),
            Opcode::SMSG_SET_PROFICIENCY => String::from("SMSG_SET_PROFICIENCY"),
            Opcode::SMSG_INITIAL_SPELLS => String::from("SMSG_INITIAL_SPELLS"),
            Opcode::SMSG_CAST_RESULT => String::from("SMSG_CAST_RESULT"),
            Opcode::SMSG_SPELL_START => String::from("SMSG_SPELL_START"),
            Opcode::SMSG_SPELL_GO => String::from("SMSG_SPELL_GO"),
            Opcode::SMSG_SPELL_FAILURE => String::from("SMSG_SPELL_FAILURE"),
            Opcode::SMSG_CANCEL_COMBAT => String::from("SMSG_CANCEL_COMBAT"),
            Opcode::SMSG_SPELLHEALLOG => String::from("SMSG_SPELLHEALLOG"),
            Opcode::SMSG_BINDPOINTUPDATE => String::from("SMSG_BINDPOINTUPDATE"),
            Opcode::SMSG_LOOT_RESPONSE => String::from("SMSG_LOOT_RESPONSE"),
            Opcode::SMSG_LOOT_RELEASE_RESPONSE => String::from("SMSG_LOOT_RELEASE_RESPONSE"),
            Opcode::SMSG_LOOT_REMOVED => String::from("SMSG_LOOT_REMOVED"),
            Opcode::SMSG_ITEM_PUSH_RESULT => String::from("SMSG_ITEM_PUSH_RESULT"),
            Opcode::SMSG_BUY_ITEM => String::from("SMSG_BUY_ITEM"),
            Opcode::SMSG_FISH_NOT_HOOKED => String::from("SMSG_FISH_NOT_HOOKED"),
            Opcode::SMSG_PONG => String::from("SMSG_PONG"),
            Opcode::SMSG_SPELL_DELAYED => String::from("SMSG_SPELL_DELAYED"),
            Opcode::SMSG_AUTH_CHALLENGE => String::from("SMSG_AUTH_CHALLENGE"),
            Opcode::SMSG_AUTH_RESPONSE => String::from("SMSG_AUTH_RESPONSE"),
            Opcode::SMSG_COMPRESSED_UPDATE_OBJECT => String::from("SMSG_COMPRESSED_UPDATE_OBJECT"),
            Opcode::SMSG_ACCOUNT_DATA_TIMES => String::from("SMSG_ACCOUNT_DATA_TIMES"),
            Opcode::SMSG_GAMEOBJECT_DESPAWN_ANIM => String::from("SMSG_GAMEOBJECT_DESPAWN_ANIM"),
            Opcode::SMSG_LOGIN_VERIFY_WORLD => String::from("SMSG_LOGIN_VERIFY_WORLD"),
            Opcode::SMSG_SEND_MAIL_RESULT => String::from("SMSG_SEND_MAIL_RESULT"),
            Opcode::SMSG_MAIL_LIST_RESULT => String::from("SMSG_MAIL_LIST_RESULT"),
            Opcode::SMSG_SPELLLOGEXECUTE => String::from("SMSG_SPELLLOGEXECUTE"),
            Opcode::SMSG_SET_PCT_SPELL_MODIFIER => String::from("SMSG_SET_PCT_SPELL_MODIFIER"),
            Opcode::SMSG_SET_FORCED_REACTIONS => String::from("SMSG_SET_FORCED_REACTIONS"),
            Opcode::SMSG_SPELL_FAILED_OTHER => String::from("SMSG_SPELL_FAILED_OTHER"),
            Opcode::SMSG_WARDEN_DATA => String::from("SMSG_WARDEN_DATA"),
            Opcode::SMSG_ADDON_INFO => String::from("SMSG_ADDON_INFO"),
            Opcode::SMSG_EXPECTED_SPAM_RECORDS => String::from("SMSG_EXPECTED_SPAM_RECORDS"),
            Opcode::SMSG_MOTD => String::from("SMSG_MOTD"),
            Opcode::SMSG_REALM_SPLIT => String::from("SMSG_REALM_SPLIT"),
            Opcode::SMSG_FEATURE_SYSTEM_STATUS => String::from("SMSG_FEATURE_SYSTEM_STATUS"),
            Opcode::SMSG_QUESTGIVER_STATUS_MULTIPLE => String::from("SMSG_QUESTGIVER_STATUS_MULTIPLE"),
            Opcode::SMSG_ACHIEVEMENT_EARNED => String::from("SMSG_ACHIEVEMENT_EARNED"),
            Opcode::SMSG_CRITERIA_UPDATE => String::from("SMSG_CRITERIA_UPDATE"),
            Opcode::SMSG_SET_PHASE_SHIFT => String::from("SMSG_SET_PHASE_SHIFT"),
            Opcode::SMSG_POWER_UPDATE => String::from("SMSG_POWER_UPDATE"),
            Opcode::SMSG_AURA_UPDATE_ALL => String::from("SMSG_AURA_UPDATE_ALL"),
            Opcode::SMSG_AURA_UPDATE => String::from("SMSG_AURA_UPDATE"),
            Opcode::SMSG_CLIENTCACHE_VERSION => String::from("SMSG_CLIENTCACHE_VERSION"),
            Opcode::SMSG_TALENT_UPDATE => String::from("SMSG_TALENT_UPDATE"),
            Opcode::MSG_MOVE_START_FORWARD => String::from("MSG_MOVE_START_FORWARD"),
            Opcode::MSG_MOVE_START_BACKWARD => String::from("MSG_MOVE_START_BACKWARD"),
            Opcode::MSG_MOVE_STOP => String::from("MSG_MOVE_STOP"),
            Opcode::MSG_MOVE_START_STRAFE_LEFT => String::from("MSG_MOVE_START_STRAFE_LEFT"),
            Opcode::MSG_MOVE_START_STRAFE_RIGHT => String::from("MSG_MOVE_START_STRAFE_RIGHT"),
            Opcode::MSG_MOVE_STOP_STRAFE => String::from("MSG_MOVE_STOP_STRAFE"),
            Opcode::MSG_MOVE_JUMP => String::from("MSG_MOVE_JUMP"),
            Opcode::MSG_MOVE_START_TURN_LEFT => String::from("MSG_MOVE_START_TURN_LEFT"),
            Opcode::MSG_MOVE_START_TURN_RIGHT => String::from("MSG_MOVE_START_TURN_RIGHT"),
            Opcode::MSG_MOVE_STOP_TURN => String::from("MSG_MOVE_STOP_TURN"),
            Opcode::MSG_MOVE_START_PITCH_UP => String::from("MSG_MOVE_START_PITCH_UP"),
            Opcode::MSG_MOVE_START_PITCH_DOWN => String::from("MSG_MOVE_START_PITCH_DOWN"),
            Opcode::MSG_MOVE_STOP_PITCH => String::from("MSG_MOVE_STOP_PITCH"),
            Opcode::MSG_MOVE_FALL_LAND => String::from("MSG_MOVE_FALL_LAND"),
            Opcode::MSG_MOVE_START_SWIM => String::from("MSG_MOVE_START_SWIM"),
            Opcode::MSG_MOVE_STOP_SWIM => String::from("MSG_MOVE_STOP_SWIM"),
            Opcode::MSG_MOVE_SET_FACING => String::from("MSG_MOVE_SET_FACING"),
            Opcode::MSG_MOVE_SET_PITCH => String::from("MSG_MOVE_SET_PITCH"),
            Opcode::MSG_MOVE_WORLDPORT_ACK => String::from("MSG_MOVE_WORLDPORT_ACK"),
            Opcode::MSG_MOVE_HEARTBEAT => String::from("MSG_MOVE_HEARTBEAT"),
            Opcode::MSG_SET_DUNGEON_DIFFICULTY => String::from("MSG_SET_DUNGEON_DIFFICULTY"),
            _ => format!("{}", opcode),
        }
    }
}