//! Sprite bitmap data for Space Invaders.
//!
//! All sprites are stored as `const` arrays in flash (zero RAM cost).
//! Format: one byte per row, MSB = leftmost pixel, 1 = black pixel.
//! The render module converts to LCD bit ordering (LSB-first, 0 = black).

/// Player ship: 16 pixels wide, 8 rows tall.
/// Stored as 2 bytes per row (big-endian: first byte = left half).
pub const PLAYER: [[u8; 2]; 8] = [
    [0x01, 0x80], //        ##
    [0x01, 0x80], //        ##
    [0x03, 0xC0], //       ####
    [0x07, 0xE0], //      ######
    [0x1F, 0xF8], //    ##########
    [0x3F, 0xFC], //   ############
    [0x7F, 0xFE], //  ##############
    [0x7F, 0xFE], //  ##############
];

/// Invader type A (rows 0-1): 8px wide, 8 rows - "crab" shape, frame 1.
pub const INVADER_A1: [u8; 8] = [
    0x18, //    ##
    0x3C, //   ####
    0x7E, //  ######
    0xDB, // ## ## ##
    0xFF, // ########
    0x24, //   #  #
    0x5A, //  # ## #
    0xA5, // #  ##  #
];

/// Invader type A, frame 2.
pub const INVADER_A2: [u8; 8] = [
    0x18, //    ##
    0x3C, //   ####
    0x7E, //  ######
    0xDB, // ## ## ##
    0xFF, // ########
    0x24, //   #  #
    0x42, //  #    #
    0x24, //   #  #
];

/// Invader type B (rows 2-3): 8px wide - "squid" shape, frame 1.
pub const INVADER_B1: [u8; 8] = [
    0x24, //   #  #
    0x24, //   #  #
    0x7E, //  ######
    0xDB, // ## ## ##
    0xFF, // ########
    0xFF, // ########
    0xA5, // #  ##  #
    0x18, //    ##
];

/// Invader type B, frame 2.
pub const INVADER_B2: [u8; 8] = [
    0x24, //   #  #
    0x24, //   #  #
    0x7E, //  ######
    0xDB, // ## ## ##
    0xFF, // ########
    0xFF, // ########
    0x24, //   #  #
    0x42, //  #    #
];

/// Invader type C (row 4): 8px wide - "octopus" shape, frame 1.
pub const INVADER_C1: [u8; 8] = [
    0x3C, //   ####
    0x7E, //  ######
    0xFF, // ########
    0xDB, // ## ## ##
    0xFF, // ########
    0x3C, //   ####
    0x66, //  ##  ##
    0xC3, // ##    ##
];

/// Invader type C, frame 2.
pub const INVADER_C2: [u8; 8] = [
    0x3C, //   ####
    0x7E, //  ######
    0xFF, // ########
    0xDB, // ## ## ##
    0xFF, // ########
    0x3C, //   ####
    0x66, //  ##  ##
    0x24, //   #  #
];

/// Explosion sprite: 8x8.
pub const EXPLOSION: [u8; 8] = [
    0x24, //   #  #
    0x42, //  #    #
    0x18, //    ##
    0xBD, // # #### #
    0x18, //    ##
    0x42, //  #    #
    0x24, //   #  #
    0x00, //
];

/// Player bullet: 2px wide, 4 rows (stored in upper bits of byte).
pub const BULLET_PLAYER: [u8; 4] = [
    0xC0, // ##
    0xC0, // ##
    0xC0, // ##
    0xC0, // ##
];

/// Invader bullet: 2px wide, 4 rows.
pub const BULLET_INVADER: [u8; 4] = [
    0xC0, // ##
    0x60, //  ##
    0xC0, // ##
    0x60, //  ##
];
