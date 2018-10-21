//! Correction Factors

#[allow(dead_code)]
/// Correction factors, with (upper_bounds_in_cm, correction_factor_in_cm)
pub const CORRECTION_FACTORS: &[(u16, i16)] = &[
    (25, -23),
    (50, -22),
    (100, -21),
    (125, -20),
    (150, -19),
    (200, -18),
    (225, -17),
    (250, -16),
    (300, -15),
    (325, -14),
    (375, -13),
    (450, -12),
    (500, -11),
    (550, -10),
    (600, -9),
    (675, -8),
    (725, -7),
    (800, -6),
    (875, -5),
    (950, -4),
    (1025, -3),
    (1100, -2),
    (1175, -1),
    (1275, 0),
    (1375, 1),
    (1450, 2),
    (1550, 3),
    (1650, 4),
    (1775, 5),
    (1950, 6),
    (2125, 7),
    (2400, 8),
    (2775, 9),
    (3375, 10),
    (4850, 11),
    (6000, 12),
    (65535, 13),
];