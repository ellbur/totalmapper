
// vim: shiftwidth=2

use serde::{Deserialize, Serialize};
use enum_utils::FromStr;
use num_derive::FromPrimitive;
use std::fmt::Display;

#[derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, FromStr, FromPrimitive, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum KeyCode {
  ESC = 1,
  #[serde(rename = "1")]
  K1 = 2,
  #[serde(rename = "2")]
  K2 = 3,
  #[serde(rename = "3")]
  K3 = 4,
  #[serde(rename = "4")]
  K4 = 5,
  #[serde(rename = "5")]
  K5 = 6,
  #[serde(rename = "6")]
  K6 = 7,
  #[serde(rename = "7")]
  K7 = 8,
  #[serde(rename = "8")]
  K8 = 9,
  #[serde(rename = "9")]
  K9 = 10,
  #[serde(rename = "0")]
  K0 = 11,
  MINUS = 12,
  EQUAL = 13,
  BACKSPACE = 14,
  TAB = 15,
  Q = 16,
  W = 17,
  E = 18,
  R = 19,
  T = 20,
  Y = 21,
  U = 22,
  I = 23,
  O = 24,
  P = 25,
  LEFTBRACE = 26,
  RIGHTBRACE = 27,
  ENTER = 28,
  LEFTCTRL = 29,
  A = 30,
  S = 31,
  D = 32,
  F = 33,
  G = 34,
  H = 35,
  J = 36,
  K = 37,
  L = 38,
  SEMICOLON = 39,
  APOSTROPHE = 40,
  GRAVE = 41,
  LEFTSHIFT = 42,
  BACKSLASH = 43,
  Z = 44,
  X = 45,
  C = 46,
  V = 47,
  B = 48,
  N = 49,
  M = 50,
  COMMA = 51,
  DOT = 52,
  SLASH = 53,
  RIGHTSHIFT = 54,
  KPASTERISK = 55,
  LEFTALT = 56,
  SPACE = 57,
  CAPSLOCK = 58,
  F1 = 59,
  F2 = 60,
  F3 = 61,
  F4 = 62,
  F5 = 63,
  F6 = 64,
  F7 = 65,
  F8 = 66,
  F9 = 67,
  F10 = 68,
  NUMLOCK = 69,
  SCROLLLOCK = 70,
  KP7 = 71,
  KP8 = 72,
  KP9 = 73,
  KPMINUS = 74,
  KP4 = 75,
  KP5 = 76,
  KP6 = 77,
  KPPLUS = 78,
  KP1 = 79,
  KP2 = 80,
  KP3 = 81,
  KP0 = 82,
  KPDOT = 83,
  ZENKAKUHANKAKU = 85,
  K102ND = 86,
  F11 = 87,
  F12 = 88,
  RO = 89,
  KATAKANA = 90,
  HIRAGANA = 91,
  HENKAN = 92,
  KATAKANAHIRAGANA = 93,
  MUHENKAN = 94,
  KPJPCOMMA = 95,
  KPENTER = 96,
  RIGHTCTRL = 97,
  KPSLASH = 98,
  SYSRQ = 99,
  RIGHTALT = 100,
  LINEFEED = 101,
  HOME = 102,
  UP = 103,
  PAGEUP = 104,
  LEFT = 105,
  RIGHT = 106,
  END = 107,
  DOWN = 108,
  PAGEDOWN = 109,
  INSERT = 110,
  DELETE = 111,
  MACRO = 112,
  MUTE = 113,
  VOLUMEDOWN = 114,
  VOLUMEUP = 115,
  POWER = 116,
  KPEQUAL = 117,
  KPPLUSMINUS = 118,
  PAUSE = 119,
  SCALE = 120,
  KPCOMMA = 121,
  HANGEUL = 122,
  HANJA = 123,
  YEN = 124,
  LEFTMETA = 125,
  RIGHTMETA = 126,
  COMPOSE = 127,
  STOP = 128,
  AGAIN = 129,
  PROPS = 130,
  UNDO = 131,
  FRONT = 132,
  COPY = 133,
  OPEN = 134,
  PASTE = 135,
  FIND = 136,
  CUT = 137,
  HELP = 138,
  MENU = 139,
  CALC = 140,
  SETUP = 141,
  SLEEP = 142,
  WAKEUP = 143,
  FILE = 144,
  SENDFILE = 145,
  DELETEFILE = 146,
  XFER = 147,
  PROG1 = 148,
  PROG2 = 149,
  WWW = 150,
  MSDOS = 151,
  COFFEE = 152,
  ROTATE_DISPLAY = 153,
  CYCLEWINDOWS = 154,
  MAIL = 155,
  BOOKMARKS = 156,
  COMPUTER = 157,
  BACK = 158,
  FORWARD = 159,
  CLOSECD = 160,
  EJECTCD = 161,
  EJECTCLOSECD = 162,
  NEXTSONG = 163,
  PLAYPAUSE = 164,
  PREVIOUSSONG = 165,
  STOPCD = 166,
  RECORD = 167,
  REWIND = 168,
  PHONE = 169,
  ISO = 170,
  CONFIG = 171,
  HOMEPAGE = 172,
  REFRESH = 173,
  EXIT = 174,
  MOVE = 175,
  EDIT = 176,
  SCROLLUP = 177,
  SCROLLDOWN = 178,
  KPLEFTPAREN = 179,
  KPRIGHTPAREN = 180,
  NEW = 181,
  REDO = 182,
  F13 = 183,
  F14 = 184,
  F15 = 185,
  F16 = 186,
  F17 = 187,
  F18 = 188,
  F19 = 189,
  F20 = 190,
  F21 = 191,
  F22 = 192,
  F23 = 193,
  F24 = 194,
  PLAYCD = 200,
  PAUSECD = 201,
  PROG3 = 202,
  PROG4 = 203,
  DASHBOARD = 204,
  SUSPEND = 205,
  CLOSE = 206,
  PLAY = 207,
  FASTFORWARD = 208,
  BASSBOOST = 209,
  PRINT = 210,
  HP = 211,
  CAMERA = 212,
  SOUND = 213,
  QUESTION = 214,
  EMAIL = 215,
  CHAT = 216,
  SEARCH = 217,
  CONNECT = 218,
  FINANCE = 219,
  SPORT = 220,
  SHOP = 221,
  ALTERASE = 222,
  CANCEL = 223,
  BRIGHTNESSDOWN = 224,
  BRIGHTNESSUP = 225,
  MEDIA = 226,
  SWITCHVIDEOMODE = 227,
  KBDILLUMTOGGLE = 228,
  KBDILLUMDOWN = 229,
  KBDILLUMUP = 230,
  SEND = 231,
  REPLY = 232,
  FORWARDMAIL = 233,
  SAVE = 234,
  DOCUMENTS = 235,
  BATTERY = 236,
  BLUETOOTH = 237,
  WLAN = 238,
  UWB = 239,
  UNKNOWN = 240,
  VIDEO_NEXT = 241,
  VIDEO_PREV = 242,
  BRIGHTNESS_CYCLE = 243,
  BRIGHTNESS_AUTO = 244,
  DISPLAY_OFF = 245,
  WWAN = 246,
  RFKILL = 247,
  MICMUTE = 248,
  OK = 352,
  SELECT = 353,
  GOTO = 354,
  CLEAR = 355,
  POWER2 = 356,
  OPTION = 357,
  INFO = 358,
  TIME = 359,
  VENDOR = 360,
  ARCHIVE = 361,
  PROGRAM = 362,
  CHANNEL = 363,
  FAVORITES = 364,
  EPG = 365,
  PVR = 366,
  MHP = 367,
  LANGUAGE = 368,
  TITLE = 369,
  SUBTITLE = 370,
  ANGLE = 371,
  FULL_SCREEN = 372,
  MODE = 373,
  KEYBOARD = 374,
  ASPECT_RATIO = 375,
  PC = 376,
  TV = 377,
  TV2 = 378,
  VCR = 379,
  VCR2 = 380,
  SAT = 381,
  SAT2 = 382,
  CD = 383,
  TAPE = 384,
  RADIO = 385,
  TUNER = 386,
  PLAYER = 387,
  TEXT = 388,
  DVD = 389,
  AUX = 390,
  MP3 = 391,
  AUDIO = 392,
  VIDEO = 393,
  DIRECTORY = 394,
  LIST = 395,
  MEMO = 396,
  CALENDAR = 397,
  RED = 398,
  GREEN = 399,
  YELLOW = 400,
  BLUE = 401,
  CHANNELUP = 402,
  CHANNELDOWN = 403,
  FIRST = 404,
  LAST = 405,
  AB = 406,
  NEXT = 407,
  RESTART = 408,
  SLOW = 409,
  SHUFFLE = 410,
  BREAK = 411,
  PREVIOUS = 412,
  DIGITS = 413,
  TEEN = 414,
  TWEN = 415,
  VIDEOPHONE = 416,
  GAMES = 417,
  ZOOMIN = 418,
  ZOOMOUT = 419,
  ZOOMRESET = 420,
  WORDPROCESSOR = 421,
  EDITOR = 422,
  SPREADSHEET = 423,
  GRAPHICSEDITOR = 424,
  PRESENTATION = 425,
  DATABASE = 426,
  NEWS = 427,
  VOICEMAIL = 428,
  ADDRESSBOOK = 429,
  MESSENGER = 430,
  DISPLAYTOGGLE = 431,
  SPELLCHECK = 432,
  LOGOFF = 433,
  DOLLAR = 434,
  EURO = 435,
  FRAMEBACK = 436,
  FRAMEFORWARD = 437,
  CONTEXT_MENU = 438,
  MEDIA_REPEAT = 439,
  K10CHANNELSUP = 440,
  K10CHANNELSDOWN = 441,
  IMAGES = 442,
  NOTIFICATION_CENTER = 444,
  PICKUP_PHONE = 445,
  HANGUP_PHONE = 446,
  DEL_EOL = 448,
  DEL_EOS = 449,
  INS_LINE = 450,
  DEL_LINE = 451,
  FN = 464,
  FN_ESC = 465,
  FN_F1 = 466,
  FN_F2 = 467,
  FN_F3 = 468,
  FN_F4 = 469,
  FN_F5 = 470,
  FN_F6 = 471,
  FN_F7 = 472,
  FN_F8 = 473,
  FN_F9 = 474,
  FN_F10 = 475,
  FN_F11 = 476,
  FN_F12 = 477,
  FN_1 = 478,
  FN_2 = 479,
  FN_D = 480,
  FN_E = 481,
  FN_F = 482,
  FN_S = 483,
  FN_B = 484,
  FN_RIGHT_SHIFT = 485,
  BRL_DOT1 = 497,
  BRL_DOT2 = 498,
  BRL_DOT3 = 499,
  BRL_DOT4 = 500,
  BRL_DOT5 = 501,
  BRL_DOT6 = 502,
  BRL_DOT7 = 503,
  BRL_DOT8 = 504,
  BRL_DOT9 = 505,
  BRL_DOT10 = 506,
  NUMERIC_0 = 512,
  NUMERIC_1 = 513,
  NUMERIC_2 = 514,
  NUMERIC_3 = 515,
  NUMERIC_4 = 516,
  NUMERIC_5 = 517,
  NUMERIC_6 = 518,
  NUMERIC_7 = 519,
  NUMERIC_8 = 520,
  NUMERIC_9 = 521,
  NUMERIC_STAR = 522,
  NUMERIC_POUND = 523,
  NUMERIC_A = 524,
  NUMERIC_B = 525,
  NUMERIC_C = 526,
  NUMERIC_D = 527,
  CAMERA_FOCUS = 528,
  WPS_BUTTON = 529,
  TOUCHPAD_TOGGLE = 530,
  TOUCHPAD_ON = 531,
  TOUCHPAD_OFF = 532,
  CAMERA_ZOOMIN = 533,
  CAMERA_ZOOMOUT = 534,
  CAMERA_UP = 535,
  CAMERA_DOWN = 536,
  CAMERA_LEFT = 537,
  CAMERA_RIGHT = 538,
  ATTENDANT_ON = 539,
  ATTENDANT_OFF = 540,
  ATTENDANT_TOGGLE = 541,
  LIGHTS_TOGGLE = 542,
  ALS_TOGGLE = 560,
  ROTATE_LOCK_TOGGLE = 561,
  BUTTONCONFIG = 576,
  TASKMANAGER = 577,
  JOURNAL = 578,
  CONTROLPANEL = 579,
  APPSELECT = 580,
  SCREENSAVER = 581,
  VOICECOMMAND = 582,
  ASSISTANT = 583,
  KBD_LAYOUT_NEXT = 584,
  BRIGHTNESS_MIN = 592,
  BRIGHTNESS_MAX = 593,
  KBDINPUTASSIST_PREV = 608,
  KBDINPUTASSIST_NEXT = 609,
  KBDINPUTASSIST_PREVGROUP = 610,
  KBDINPUTASSIST_NEXTGROUP = 611,
  KBDINPUTASSIST_ACCEPT = 612,
  KBDINPUTASSIST_CANCEL = 613,
  RIGHT_UP = 614,
  RIGHT_DOWN = 615,
  LEFT_UP = 616,
  LEFT_DOWN = 617,
  ROOT_MENU = 618,
  MEDIA_TOP_MENU = 619,
  NUMERIC_11 = 620,
  NUMERIC_12 = 621,
  AUDIO_DESC = 622,
  K3D_MODE = 623,
  NEXT_FAVORITE = 624,
  STOP_RECORD = 625,
  PAUSE_RECORD = 626,
  VOD = 627,
  UNMUTE = 628,
  FASTREVERSE = 629,
  SLOWREVERSE = 630,
  DATA = 631,
  ONSCREEN_KEYBOARD = 632,
  PRIVACY_SCREEN_TOGGLE = 633,
  SELECTIVE_SCREENSHOT = 634,
  MACRO1 = 656,
  MACRO2 = 657,
  MACRO3 = 658,
  MACRO4 = 659,
  MACRO5 = 660,
  MACRO6 = 661,
  MACRO7 = 662,
  MACRO8 = 663,
  MACRO9 = 664,
  MACRO10 = 665,
  MACRO11 = 666,
  MACRO12 = 667,
  MACRO13 = 668,
  MACRO14 = 669,
  MACRO15 = 670,
  MACRO16 = 671,
  MACRO17 = 672,
  MACRO18 = 673,
  MACRO19 = 674,
  MACRO20 = 675,
  MACRO21 = 676,
  MACRO22 = 677,
  MACRO23 = 678,
  MACRO24 = 679,
  MACRO25 = 680,
  MACRO26 = 681,
  MACRO27 = 682,
  MACRO28 = 683,
  MACRO29 = 684,
  MACRO30 = 685,
  MACRO_RECORD_START = 688,
  MACRO_RECORD_STOP = 689,
  MACRO_PRESET_CYCLE = 690,
  MACRO_PRESET1 = 691,
  MACRO_PRESET2 = 692,
  MACRO_PRESET3 = 693,
  KBD_LCD_MENU1 = 696,
  KBD_LCD_MENU2 = 697,
  KBD_LCD_MENU3 = 698,
  KBD_LCD_MENU4 = 699,
  KBD_LCD_MENU5 = 700,
}

#[allow(dead_code)]
pub fn get_all_keyboard_key_codes() -> Vec<KeyCode> {
  use KeyCode::*;
  
  vec![
    ESC, K1, K2, K3, K4, K5, K6, K7, K8, K9, K0, MINUS, EQUAL, BACKSPACE, TAB, Q, W,
    E, R, T, Y, U, I, O, P, LEFTBRACE, RIGHTBRACE, ENTER, LEFTCTRL, A, S, D, F, G,
    H, J, K, L, SEMICOLON, APOSTROPHE, GRAVE, LEFTSHIFT, BACKSLASH, Z, X, C, V, B,
    N, M, COMMA, DOT, SLASH, RIGHTSHIFT, KPASTERISK, LEFTALT, SPACE, CAPSLOCK, F1, F2,
    F3, F4, F5, F6, F7, F8, F9, F10, NUMLOCK, SCROLLLOCK, KP7, KP8, KP9, KPMINUS, KP4,
    KP5, KP6, KPPLUS, KP1, KP2, KP3, KP0, KPDOT, ZENKAKUHANKAKU, K102ND, F11, F12, RO,
    KATAKANA, HIRAGANA, HENKAN, KATAKANAHIRAGANA, MUHENKAN, KPJPCOMMA, KPENTER, RIGHTCTRL,
    KPSLASH, SYSRQ, RIGHTALT, LINEFEED, HOME, UP, PAGEUP, LEFT, RIGHT, END, DOWN, PAGEDOWN,
    INSERT, DELETE, MACRO, MUTE, VOLUMEDOWN, VOLUMEUP, POWER, KPEQUAL, KPPLUSMINUS, PAUSE,
    SCALE, KPCOMMA, HANGEUL, HANJA, YEN, LEFTMETA, RIGHTMETA, COMPOSE, STOP, AGAIN, PROPS,
    UNDO, FRONT, COPY, OPEN, PASTE, FIND, CUT, HELP, MENU, CALC, SETUP, SLEEP, WAKEUP,
    FILE, SENDFILE, DELETEFILE, XFER, PROG1, PROG2, WWW, MSDOS, COFFEE, ROTATE_DISPLAY,
    CYCLEWINDOWS, MAIL, BOOKMARKS, COMPUTER, BACK, FORWARD, CLOSECD, EJECTCD, EJECTCLOSECD,
    NEXTSONG, PLAYPAUSE, PREVIOUSSONG, STOPCD, RECORD, REWIND, PHONE, ISO, CONFIG,
    HOMEPAGE, REFRESH, EXIT, MOVE, EDIT, SCROLLUP, SCROLLDOWN, KPLEFTPAREN, KPRIGHTPAREN,
    NEW, REDO, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, PLAYCD, PAUSECD,
    PROG3, PROG4, DASHBOARD, SUSPEND, CLOSE, PLAY, FASTFORWARD, BASSBOOST, PRINT,
    HP, CAMERA, SOUND, QUESTION, EMAIL, CHAT, SEARCH, CONNECT, FINANCE, SPORT, SHOP,
    ALTERASE, CANCEL, BRIGHTNESSDOWN, BRIGHTNESSUP, MEDIA, SWITCHVIDEOMODE, KBDILLUMTOGGLE,
    KBDILLUMDOWN, KBDILLUMUP, SEND, REPLY, FORWARDMAIL, SAVE, DOCUMENTS, BATTERY,
    BLUETOOTH, WLAN, UWB, UNKNOWN, VIDEO_NEXT, VIDEO_PREV, BRIGHTNESS_CYCLE, BRIGHTNESS_AUTO,
    DISPLAY_OFF, WWAN, RFKILL, MICMUTE, OK, SELECT, GOTO, CLEAR, POWER2, OPTION,
    INFO, TIME, VENDOR, ARCHIVE, PROGRAM, CHANNEL, FAVORITES, EPG, PVR, MHP, LANGUAGE,
    TITLE, SUBTITLE, ANGLE, FULL_SCREEN, MODE, KEYBOARD, ASPECT_RATIO, PC, TV, TV2,
    VCR, VCR2, SAT, SAT2, CD, TAPE, RADIO, TUNER, PLAYER, TEXT, DVD, AUX, MP3, AUDIO, VIDEO,
    DIRECTORY, LIST, MEMO, CALENDAR, RED, GREEN, YELLOW, BLUE, CHANNELUP, CHANNELDOWN, FIRST,
    LAST, AB, NEXT, RESTART, SLOW, SHUFFLE, BREAK, PREVIOUS, DIGITS, TEEN, TWEN, VIDEOPHONE,
    GAMES, ZOOMIN, ZOOMOUT, ZOOMRESET, WORDPROCESSOR, EDITOR, SPREADSHEET, GRAPHICSEDITOR,
    PRESENTATION, DATABASE, NEWS, VOICEMAIL, ADDRESSBOOK, MESSENGER, DISPLAYTOGGLE,
    SPELLCHECK, LOGOFF, DOLLAR, EURO, FRAMEBACK, FRAMEFORWARD, CONTEXT_MENU, MEDIA_REPEAT,
    K10CHANNELSUP, K10CHANNELSDOWN, IMAGES, NOTIFICATION_CENTER, PICKUP_PHONE, HANGUP_PHONE,
    DEL_EOL, DEL_EOS, INS_LINE, DEL_LINE, FN, FN_ESC, FN_F1, FN_F2, FN_F3, FN_F4, FN_F5,
    FN_F6, FN_F7, FN_F8, FN_F9, FN_F10, FN_F11, FN_F12, FN_1, FN_2, FN_D, FN_E, FN_F, FN_S,
    FN_B, FN_RIGHT_SHIFT, BRL_DOT1, BRL_DOT2, BRL_DOT3, BRL_DOT4, BRL_DOT5, BRL_DOT6,
    BRL_DOT7, BRL_DOT8, BRL_DOT9, BRL_DOT10, NUMERIC_0, NUMERIC_1, NUMERIC_2, NUMERIC_3,
    NUMERIC_4, NUMERIC_5, NUMERIC_6, NUMERIC_7, NUMERIC_8, NUMERIC_9, NUMERIC_STAR,
    NUMERIC_POUND, NUMERIC_A, NUMERIC_B, NUMERIC_C, NUMERIC_D, CAMERA_FOCUS, WPS_BUTTON,
    TOUCHPAD_TOGGLE, TOUCHPAD_ON, TOUCHPAD_OFF, CAMERA_ZOOMIN, CAMERA_ZOOMOUT, CAMERA_UP,
    CAMERA_DOWN, CAMERA_LEFT, CAMERA_RIGHT, ATTENDANT_ON, ATTENDANT_OFF, ATTENDANT_TOGGLE,
    LIGHTS_TOGGLE, ALS_TOGGLE, ROTATE_LOCK_TOGGLE,
  ]  
}

