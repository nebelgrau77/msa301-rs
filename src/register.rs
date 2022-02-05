//! Register mapping and bitmasks
//!

/// MSA301 Registers
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum Registers {
    /// Soft reset (r)
    SOFT_RESET = 0x00,
    /// Part ID (r)
    PART_ID = 0x01,
    /// X axis LSB (r)
    XAXIS_L = 0x02,
    /// X axis MSB (r)
    XAXIS_H = 0x03,
    /// Y axis LSB (r)
    YAXIS_L = 0x04,
    /// Y axis MSB (r)
    YAXIS_H = 0x05,
    /// Z axis LSB (r)
    ZAXIS_L = 0x06,
    /// Z axis MSB (r)
    ZAXIS_H = 0x07,
    /// Motion interrupt (r)
    MOTION_INT = 0x09,
    /// Data interrupt (r)
    DATA_INT = 0x0A,
    /// Tap active status (r)
    TAP_ACTIVE = 0x0B,
    /// Orientation status (r)
    ORIENTATION = 0x0C,
    /// Resolution/range (r/w)
    RES_RANGE = 0x0F,
    /// Axis enabling and output data rate (r/w)
    CFG_ODR = 0x10,
    /// Power mode/bandwidth (r/w)
    PWR_BW = 0x11,
    /// Swap polarity (r/w)
    POLAR_SWP = 0x12,
    /// Interrupt settings (r/w)
    INT_SET0 = 0x16,
    /// Interrupt settings (r/w)
    INT_SET1 = 0x17,
    /// Interrupt pin mapping (r/w)
    INT_MAP0 = 0x19,
    /// Interrupt pin mapping (r/w)
    INT_MAP1 = 0x1A,
    /// Interrupt configuration (r/w)
    INT_CFG = 0x20,
    /// Interrupt latching (r/w)
    INT_LATCH = 0x21,
    /// Freefall duration (r/w)
    FRFL_DUR = 0x22,
    /// Freefall threshold (r/w)
    FRFL_THS = 0x23,
    /// Freefall hysteresis setting (r/w)
    FRFL_HYS = 0x24,
    /// Active duration (r/w)
    ACTIVE_DUR = 0x27,
    /// Active threshold (r/w)
    ACTIVE_THS = 0x28,
    /// Tap duration (r/w)
    TAP_DUR = 0x2A,
    /// Tap threshold (r/w)
    TAP_THS = 0x2B,
    /// Orientation configuration (hysteresis, blocking mode, thresholds) (r/w)
    ORIENT_CFG = 0x2C,
    /// Z-compensation (r/w)
    Z_COMP = 0x2D,
    /// Offset compensation for the X-axis (r/w)
    OFFSET_X = 0x38,
    /// Offset compensation for the Y-axis (r/w)
    OFFSET_Y = 0x39,
    /// Offset compensation for the Z-axis (r/w)
    OFFSET_Z = 0x3A,
}

impl Registers {
    pub fn addr(self) -> u8 {
        self as u8
    }
}

/// MSA301 Bit masks
#[allow(non_camel_case_types)]
pub struct Bitmasks;
#[allow(dead_code)]
impl Bitmasks {
    // === INTERRUPT_CFG (0x0B) ===
    pub (crate) const NEW_DATA_INT: u8 = 0b0000_0001;
}


