#[derive(Clone, Debug, Default)]
pub struct Message {
    pub message_type: u8,
    pub language: u32,
    pub sender_guid: u64,
    pub channel_name: String,
    pub target_guid: u64,
    pub message: String,
}

#[non_exhaustive]
pub struct MessageType;

#[allow(dead_code)]
impl MessageType {
    pub const SYSTEM: u8 = 0x00;
    pub const SAY: u8 = 0x01;
    pub const PARTY: u8 = 0x02;
    pub const RAID: u8 = 0x03;
    pub const GUILD: u8 = 0x04;
    pub const OFFICER: u8 = 0x05;
    pub const YELL: u8 = 0x06;
    pub const WHISPER: u8 = 0x07;
    pub const WHISPER_FOREIGN: u8 = 0x08;
    pub const WHISPER_INFORM: u8 = 0x09;
    pub const EMOTE: u8 = 0x0A;
    pub const TEXT_EMOTE: u8 = 0x0B;
    pub const MONSTER_SAY: u8 = 0x0C;
    pub const MONSTER_PARTY: u8 = 0x0D;
    pub const MONSTER_YELL: u8 = 0x0E;
    pub const MONSTER_WHISPER: u8 = 0x0F;
    pub const MONSTER_EMOTE: u8 = 0x10;
    pub const CHANNEL: u8 = 0x11;
    pub const CHANNEL_JOIN: u8 = 0x12;
    pub const CHANNEL_LEAVE: u8 = 0x13;
    pub const CHANNEL_LIST: u8 = 0x14;
    pub const CHANNEL_NOTICE: u8 = 0x15;
    pub const CHANNEL_NOTICE_USER: u8 = 0x16;
    pub const AFK: u8 = 0x17;
    pub const DND: u8 = 0x18;
    pub const IGNORED: u8 = 0x19;
    pub const SKILL: u8 = 0x1A;
    pub const LOOT: u8 = 0x1B;
    pub const MONEY: u8 = 0x1C;
    pub const OPENING: u8 = 0x1D;
}

#[non_exhaustive]
pub struct EmoteType;

#[allow(dead_code)]
impl EmoteType {
    pub const ONESHOT_NONE: u32 = 0;
    pub const ONESHOT_TALK: u32 = 1;
    pub const ONESHOT_BOW: u32 = 2;
    pub const ONESHOT_WAVE: u32 = 3;
    pub const ONESHOT_CHEER: u32 = 4;
    pub const ONESHOT_EXCLAMATION: u32 = 5;
    pub const ONESHOT_QUESTION: u32 = 6;
    pub const ONESHOT_EAT: u32 = 7;
    pub const STATE_DANCE: u32 = 10;
    pub const ONESHOT_LAUGH: u32 = 11;
    pub const STATE_SLEEP: u32 = 12;
    pub const STATE_SIT: u32 = 13;
    pub const ONESHOT_RUDE: u32 = 14;
    pub const ONESHOT_ROAR: u32 = 15;
    pub const ONESHOT_KNEEL: u32 = 16;
    pub const ONESHOT_KISS: u32 = 17;
    pub const ONESHOT_CRY: u32 = 18;
    pub const ONESHOT_CHICKEN: u32 = 19;
    pub const ONESHOT_BEG: u32 = 20;
    pub const ONESHOT_APPLAUD: u32 = 21;
    pub const ONESHOT_SHOUT: u32 = 22;
    pub const ONESHOT_FLEX: u32 = 23;
    pub const ONESHOT_SHY: u32 = 24;
    pub const ONESHOT_POINT: u32 = 25;
    pub const STATE_STAND: u32 = 26;
    pub const STATE_READYUNARMED: u32 = 27;
    pub const STATE_WORK_SHEATHED: u32 = 28;
    pub const STATE_POINT: u32 = 29;
    pub const STATE_NONE: u32 = 30;
    pub const ONESHOT_WOUND: u32 = 33;
    pub const ONESHOT_WOUNDCRITICAL: u32 = 34;
    pub const ONESHOT_ATTACKUNARMED: u32 = 35;
    pub const ONESHOT_ATTACK1H: u32 = 36;
    pub const ONESHOT_ATTACK2HTIGHT: u32 = 37;
    pub const ONESHOT_ATTACK2HLOOSE: u32 = 38;
    pub const ONESHOT_PARRYUNARMED: u32 = 39;
    pub const ONESHOT_PARRYSHIELD: u32 = 43;
    pub const ONESHOT_READYUNARMED: u32 = 44;
    pub const ONESHOT_READY1H: u32 = 45;
    pub const ONESHOT_READYBOW: u32 = 48;
    pub const ONESHOT_SPELLPRECAST: u32 = 50;
    pub const ONESHOT_SPELLCAST: u32 = 51;
    pub const ONESHOT_BATTLEROAR: u32 = 53;
    pub const ONESHOT_SPECIALATTACK1H: u32 = 54;
    pub const ONESHOT_KICK: u32 = 60;
    pub const ONESHOT_ATTACKTHROWN: u32 = 61;
    pub const STATE_STUN: u32 = 64;
    pub const STATE_DEAD: u32 = 65;
    pub const ONESHOT_SALUTE: u32 = 66;
    pub const STATE_KNEEL: u32 = 68;
    pub const STATE_USESTANDING: u32 = 69;
    pub const ONESHOT_WAVE_NOSHEATHE: u32 = 70;
    pub const ONESHOT_CHEER_NOSHEATHE: u32 = 71;
    pub const ONESHOT_EAT_NOSHEATHE: u32 = 92;
    pub const STATE_STUN_NOSHEATHE: u32 = 93;
    pub const ONESHOT_DANCE: u32 = 94;
    pub const ONESHOT_SALUTE_NOSHEATH: u32 = 113;
    pub const STATE_USESTANDING_NOSHEATHE: u32 = 133;
    pub const ONESHOT_LAUGH_NOSHEATHE: u32 = 153;
    pub const STATE_WORK: u32 = 173;
    pub const STATE_SPELLPRECAST: u32 = 193;
    pub const ONESHOT_READYRIFLE: u32 = 213;
    pub const STATE_READYRIFLE: u32 = 214;
    pub const STATE_WORK_MINING: u32 = 233;
    pub const STATE_WORK_CHOPWOOD: u32 = 234;
    pub const STATE_APPLAUD: u32 = 253;
    pub const ONESHOT_LIFTOFF: u32 = 254;
    pub const ONESHOT_YES: u32 = 273;
    pub const ONESHOT_NO: u32 = 274;
    pub const ONESHOT_TRAIN: u32 = 275;
    pub const ONESHOT_LAND: u32 = 293;
    pub const STATE_AT_EASE: u32 = 313;
    pub const STATE_READY1H: u32 = 333;
    pub const STATE_SPELLKNEELSTART: u32 = 353;
    pub const STATE_SUBMERGED: u32 = 373;
    pub const ONESHOT_SUBMERGE: u32 = 374;
    pub const STATE_READY2H: u32 = 375;
    pub const STATE_READYBOW: u32 = 376;
    pub const ONESHOT_MOUNTSPECIAL: u32 = 377;
    pub const STATE_TALK: u32 = 378;
    pub const STATE_FISHING: u32 = 379;
    pub const ONESHOT_FISHING: u32 = 380;
    pub const ONESHOT_LOOT: u32 = 381;
    pub const STATE_WHIRLWIND: u32 = 382;
    pub const STATE_DROWNED: u32 = 383;
    pub const STATE_HOLD_BOW: u32 = 384;
    pub const STATE_HOLD_RIFLE: u32 = 385;
    pub const STATE_HOLD_THROWN: u32 = 386;
    pub const ONESHOT_DROWN: u32 = 387;
    pub const ONESHOT_STOMP: u32 = 388;
    pub const ONESHOT_ATTACKOFF: u32 = 389;
    pub const ONESHOT_ATTACKOFFPIERCE: u32 = 390;
    pub const STATE_ROAR: u32 = 391;
    pub const STATE_LAUGH: u32 = 392;
    pub const ONESHOT_CREATURE_SPECIAL: u32 = 393;
    pub const ONESHOT_JUMPLANDRUN: u32 = 394;
    pub const ONESHOT_JUMPEND: u32 = 395;
    pub const ONESHOT_TALK_NOSHEATHE: u32 = 396;
    pub const ONESHOT_POINT_NOSHEATHE: u32 = 397;
    pub const STATE_CANNIBALIZE: u32 = 398;
    pub const ONESHOT_JUMPSTART: u32 = 399;
    pub const STATE_DANCESPECIAL: u32 = 400;
    pub const ONESHOT_DANCESPECIAL: u32 = 401;
    pub const ONESHOT_CUSTOMSPELL01: u32 = 402;
    pub const ONESHOT_CUSTOMSPELL02: u32 = 403;
    pub const ONESHOT_CUSTOMSPELL03: u32 = 404;
    pub const ONESHOT_CUSTOMSPELL04: u32 = 405;
    pub const ONESHOT_CUSTOMSPELL05: u32 = 406;
    pub const ONESHOT_CUSTOMSPELL06: u32 = 407;
    pub const ONESHOT_CUSTOMSPELL07: u32 = 408;
    pub const ONESHOT_CUSTOMSPELL08: u32 = 409;
    pub const ONESHOT_CUSTOMSPELL09: u32 = 410;
    pub const ONESHOT_CUSTOMSPELL10: u32 = 411;
    pub const STATE_EXCLAIM: u32 = 412;
    pub const STATE_DANCE_CUSTOM: u32 = 413;
    pub const STATE_SIT_CHAIR_MED: u32 = 415;
    pub const STATE_CUSTOM_SPELL_01: u32 = 416;
    pub const STATE_CUSTOM_SPELL_02: u32 = 417;
    pub const STATE_EAT: u32 = 418;
    pub const STATE_CUSTOM_SPELL_04: u32 = 419;
    pub const STATE_CUSTOM_SPELL_03: u32 = 420;
    pub const STATE_CUSTOM_SPELL_05: u32 = 421;
    pub const STATE_SPELLEFFECT_HOLD: u32 = 422;
    pub const STATE_EAT_NO_SHEATHE: u32 = 423;
    pub const STATE_MOUNT: u32 = 424;
    pub const STATE_READY2HL: u32 = 425;
    pub const STATE_SIT_CHAIR_HIGH: u32 = 426;
    pub const STATE_FALL: u32 = 427;
    pub const STATE_LOOT: u32 = 428;
    pub const STATE_SUBMERGED_NEW: u32 = 429;
    pub const ONESHOT_COWER: u32 = 430;
    pub const STATE_COWER: u32 = 431;
    pub const ONESHOT_USESTANDING: u32 = 432;
    pub const STATE_STEALTH_STAND: u32 = 433;
    pub const ONESHOT_OMNICAST_GHOUL: u32 = 434;
    pub const ONESHOT_ATTACKBOW: u32 = 435;
    pub const ONESHOT_ATTACKRIFLE: u32 = 436;
    pub const STATE_SWIM_IDLE: u32 = 437;
    pub const STATE_ATTACK_UNARMED: u32 = 438;
    pub const ONESHOT_SPELLCAST_W_SOUND: u32 = 439;
    pub const ONESHOT_DODGE: u32 = 440;
    pub const ONESHOT_PARRY1H: u32 = 441;
    pub const ONESHOT_PARRY2H: u32 = 442;
    pub const ONESHOT_PARRY2HL: u32 = 443;
    pub const STATE_FLYFALL: u32 = 444;
    pub const ONESHOT_FLYDEATH: u32 = 445;
    pub const STATE_FLY_FALL: u32 = 446;
    pub const ONESHOT_FLY_SIT_GROUND_DOWN: u32 = 447;
    pub const ONESHOT_FLY_SIT_GROUND_UP: u32 = 448;
    pub const ONESHOT_EMERGE: u32 = 449;
    pub const ONESHOT_DRAGONSPIT: u32 = 450;
    pub const STATE_SPECIALUNARMED: u32 = 451;
    pub const ONESHOT_FLYGRAB: u32 = 452;
    pub const STATE_FLYGRABCLOSED: u32 = 453;
    pub const ONESHOT_FLYGRABTHROWN: u32 = 454;
    pub const STATE_FLY_SIT_GROUND: u32 = 455;
    pub const STATE_WALKBACKWARDS: u32 = 456;
    pub const ONESHOT_FLYTALK: u32 = 457;
    pub const ONESHOT_FLYATTACK1H: u32 = 458;
    pub const STATE_CUSTOMSPELL08: u32 = 459;
    pub const ONESHOT_FLY_DRAGONSPIT: u32 = 460;
    pub const STATE_SIT_CHAIR_LOW: u32 = 461;
    pub const ONE_SHOT_STUN: u32 = 462;
    pub const ONESHOT_SPELLCAST_OMNI: u32 = 463;
    pub const STATE_READYTHROWN: u32 = 465;
    pub const ONESHOT_WORK_CHOPWOOD: u32 = 466;
    pub const ONESHOT_WORK_MINING: u32 = 467;
    pub const STATE_SPELL_CHANNEL_OMNI: u32 = 468;
    pub const STATE_SPELL_CHANNEL_DIRECTED: u32 = 469;
    pub const STAND_STATE_NONE: u32 = 470;
    pub const STATE_READYJOUST: u32 = 471;
    pub const STATE_STRANGULATE: u32 = 473;
    pub const STATE_READYSPELLOMNI: u32 = 474;
    pub const STATE_HOLD_JOUST: u32 = 475;
    pub const ONESHOT_CRY_JAINA: u32 = 476;
}

#[non_exhaustive]
pub struct TextEmoteType;

#[allow(dead_code)]
impl TextEmoteType {
    pub const AGREE: u32 = 1;
    pub const AMAZE: u32 = 2;
    pub const ANGRY: u32 = 3;
    pub const APOLOGIZE: u32 = 4;
    pub const APPLAUD: u32 = 5;
    pub const BASHFUL: u32 = 6;
    pub const BECKON: u32 = 7;
    pub const BEG: u32 = 8;
    pub const BITE: u32 = 9;
    pub const BLEED: u32 = 10;
    pub const BLINK: u32 = 11;
    pub const BLUSH: u32 = 12;
    pub const BONK: u32 = 13;
    pub const BORED: u32 = 14;
    pub const BOUNCE: u32 = 15;
    pub const BRB: u32 = 16;
    pub const BOW: u32 = 17;
    pub const BURP: u32 = 18;
    pub const BYE: u32 = 19;
    pub const CACKLE: u32 = 20;
    pub const CHEER: u32 = 21;
    pub const CHICKEN: u32 = 22;
    pub const CHUCKLE: u32 = 23;
    pub const CLAP: u32 = 24;
    pub const CONFUSED: u32 = 25;
    pub const CONGRATULATE: u32 = 26;
    pub const COUGH: u32 = 27;
    pub const COWER: u32 = 28;
    pub const CRACK: u32 = 29;
    pub const CRINGE: u32 = 30;
    pub const CRY: u32 = 31;
    pub const CURIOUS: u32 = 32;
    pub const CURTSEY: u32 = 33;
    pub const DANCE: u32 = 34;
    pub const DRINK: u32 = 35;
    pub const DROOL: u32 = 36;
    pub const EAT: u32 = 37;
    pub const EYE: u32 = 38;
    pub const FART: u32 = 39;
    pub const FIDGET: u32 = 40;
    pub const FLEX: u32 = 41;
    pub const FROWN: u32 = 42;
    pub const GASP: u32 = 43;
    pub const GAZE: u32 = 44;
    pub const GIGGLE: u32 = 45;
    pub const GLARE: u32 = 46;
    pub const GLOAT: u32 = 47;
    pub const GREET: u32 = 48;
    pub const GRIN: u32 = 49;
    pub const GROAN: u32 = 50;
    pub const GROVEL: u32 = 51;
    pub const GUFFAW: u32 = 52;
    pub const HAIL: u32 = 53;
    pub const HAPPY: u32 = 54;
    pub const HELLO: u32 = 55;
    pub const HUG: u32 = 56;
    pub const HUNGRY: u32 = 57;
    pub const KISS: u32 = 58;
    pub const KNEEL: u32 = 59;
    pub const LAUGH: u32 = 60;
    pub const LAYDOWN: u32 = 61;
    pub const MESSAGE: u32 = 62;
    pub const MOAN: u32 = 63;
    pub const MOON: u32 = 64;
    pub const MOURN: u32 = 65;
    pub const NO: u32 = 66;
    pub const NOD: u32 = 67;
    pub const NOSEPICK: u32 = 68;
    pub const PANIC: u32 = 69;
    pub const PEER: u32 = 70;
    pub const PLEAD: u32 = 71;
    pub const POINT: u32 = 72;
    pub const POKE: u32 = 73;
    pub const PRAY: u32 = 74;
    pub const ROAR: u32 = 75;
    pub const ROFL: u32 = 76;
    pub const RUDE: u32 = 77;
    pub const SALUTE: u32 = 78;
    pub const SCRATCH: u32 = 79;
    pub const SEXY: u32 = 80;
    pub const SHAKE: u32 = 81;
    pub const SHOUT: u32 = 82;
    pub const SHRUG: u32 = 83;
    pub const SHY: u32 = 84;
    pub const SIGH: u32 = 85;
    pub const SIT: u32 = 86;
    pub const SLEEP: u32 = 87;
    pub const SNARL: u32 = 88;
    pub const SPIT: u32 = 89;
    pub const STARE: u32 = 90;
    pub const SURPRISED: u32 = 91;
    pub const SURRENDER: u32 = 92;
    pub const TALK: u32 = 93;
    pub const TALKEX: u32 = 94;
    pub const TALKQ: u32 = 95;
    pub const TAP: u32 = 96;
    pub const THANK: u32 = 97;
    pub const THREATEN: u32 = 98;
    pub const TIRED: u32 = 99;
    pub const VICTORY: u32 = 100;
    pub const WAVE: u32 = 101;
    pub const WELCOME: u32 = 102;
    pub const WHINE: u32 = 103;
    pub const WHISTLE: u32 = 104;
    pub const WORK: u32 = 105;
    pub const YAWN: u32 = 106;
    pub const BOGGLE: u32 = 107;
    pub const CALM: u32 = 108;
    pub const COLD: u32 = 109;
    pub const COMFORT: u32 = 110;
    pub const CUDDLE: u32 = 111;
    pub const DUCK: u32 = 112;
    pub const INSULT: u32 = 113;
    pub const INTRODUCE: u32 = 114;
    pub const JK: u32 = 115;
    pub const LICK: u32 = 116;
    pub const LISTEN: u32 = 117;
    pub const LOST: u32 = 118;
    pub const MOCK: u32 = 119;
    pub const PONDER: u32 = 120;
    pub const POUNCE: u32 = 121;
    pub const PRAISE: u32 = 122;
    pub const PURR: u32 = 123;
    pub const PUZZLE: u32 = 124;
    pub const RAISE: u32 = 125;
    pub const READY: u32 = 126;
    pub const SHIMMY: u32 = 127;
    pub const SHIVER: u32 = 128;
    pub const SHOO: u32 = 129;
    pub const SLAP: u32 = 130;
    pub const SMIRK: u32 = 131;
    pub const SNIFF: u32 = 132;
    pub const SNUB: u32 = 133;
    pub const SOOTHE: u32 = 134;
    pub const STINK: u32 = 135;
    pub const TAUNT: u32 = 136;
    pub const TEASE: u32 = 137;
    pub const THIRSTY: u32 = 138;
    pub const VETO: u32 = 139;
    pub const SNICKER: u32 = 140;
    pub const STAND: u32 = 141;
    pub const TICKLE: u32 = 142;
    pub const VIOLIN: u32 = 143;
    pub const SMILE: u32 = 163;
    pub const RASP: u32 = 183;
    pub const PITY: u32 = 203;
    pub const GROWL: u32 = 204;
    pub const BARK: u32 = 205;
    pub const SCARED: u32 = 223;
    pub const FLOP: u32 = 224;
    pub const LOVE: u32 = 225;
    pub const MOO: u32 = 226;
    pub const TEXT_HELP_ME: u32 = 303;
    pub const TEXT_HEAL_ME: u32 = 326;
    pub const OPENFIRE: u32 = 327;
    pub const FLIRT: u32 = 328;
    pub const JOKE: u32 = 329;
    pub const COMMEND: u32 = 243;
    pub const WINK: u32 = 363;
    pub const PAT: u32 = 364;
    pub const SERIOUS: u32 = 365;
    pub const MOUNTSPECIAL: u32 = 366;
    pub const GOODLUCK: u32 = 367;
    pub const BLAME: u32 = 368;
    pub const BLANK: u32 = 369;
    pub const BRANDISH: u32 = 370;
    pub const BREATH: u32 = 371;
    pub const DISAGREE: u32 = 372;
    pub const DOUBT: u32 = 373;
    pub const EMBARRASS: u32 = 374;
    pub const ENCOURAGE: u32 = 375;
    pub const ENEMY: u32 = 376;
    pub const EYEBROW: u32 = 377;
    pub const TOAST: u32 = 378;
}


#[non_exhaustive]
pub struct Language;

#[allow(dead_code)]
impl Language {
    pub const UNIVERSAL: u32 = 0;
    pub const ORCISH: u32 = 1;
    pub const DARNASSIAN: u32 = 2;
    pub const TAURAHE: u32 = 3;
    pub const DWARVISH: u32 = 6;
    pub const COMMON: u32 = 7;
    pub const DEMONIC: u32 = 8;
    pub const TITAN: u32 = 9;
    pub const THALASSIAN: u32 = 10;
    pub const DRACONIC: u32 = 11;
    pub const KALIMAG: u32 = 12;
    pub const GNOMISH: u32 = 13;
    pub const TROLL: u32 = 14;
    pub const GUTTERSPEAK: u32 = 33;
    pub const DRAENEI: u32 = 35;
    pub const ZOMBIE: u32 = 36;
    pub const GNOMISH_BINARY: u32 = 37;
    pub const GOBLIN_BINARY: u32 = 38;
}
