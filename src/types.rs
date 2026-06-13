use defmt::Format;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use nutype::nutype;

macro_rules! integer_type {
    ($name:ident, $min:expr, $max:expr, $step:expr) => {
        #[nutype(
            derive(Debug, Copy, Clone, PartialEq, Eq),
            derive_unchecked(Format),
            validate(
                greater_or_equal = $min,
                less_or_equal = $max,
                predicate = |n| n.checked_sub($min).is_some_and(|n| n.is_multiple_of($step)),
            ),
        )]
        pub struct $name(u32);

        impl TryFrom<u8> for $name {
            type Error = paste::paste! { [<$name Error>] };

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                Self::try_new($min + ((value as u32) * $step))
            }
        }

        impl From<$name> for u8 {
            fn from(value: $name) -> Self {
                ((value.into_inner() - $min) / $step).try_into().unwrap()
            }
        }
    };
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum HizMode {
    Disabled = 0b0,
    Enabled = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum IlimPin {
    Disabled = 0b0,
    Enabled = 0b1,
}

integer_type!(InputCurrentLimit, 100, 3250, 50);

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum BoostModeHotTemperatureThreshold {
    /// V_bhot1 (typically 34.75%)
    Vbhot1 = 0b00,
    /// V_bhot0 (typically 37.75%)
    Vbhot0 = 0b01,
    /// V_bhot2 (typically 31.25%)
    Vbhot2 = 0b10,
    /// Disable boost mode thermal protection
    Disabled = 0b11,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum BoostModeColdTemperatureThreshold {
    /// V_bcold0 (typically 77%)
    Vbcold0 = 0b0,
    /// V_bcold1 (typically 80%)
    Vbcold1 = 0b1,
}

integer_type!(InputVoltageLimitOffset, 0, 3100, 100);

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum AdcConversionRate {
    Oneshot = 0b0,
    Continuout = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum BoostModeFrequency {
    _1_5MHz = 0b0,
    _500KHz = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum InputCurrentOptimizer {
    Disabled = 0b0,
    Enabled = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum HighVoltageDcp {
    Disabled = 0b0,
    Enabled = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum MaxChargeAdapter {
    Disabled = 0b0,
    Enabled = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ForceDpDmDetection {
    No = 0b0,
    Yes = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum AutomaticDpDmDetection {
    Disabled = 0b0,
    Enabled = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum BatteryLoadEnable {
    Disabled = 0b0,
    Enabled = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum I2cWatchdogReset {
    Normal = 0b0,
    Reset = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum BoostMode {
    Disabled = 0b0,
    Enabled = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ChargeEnable {
    Disabled = 0b0,
    Enabled = 0b1,
}

integer_type!(MinimumSystemVoltageLimit, 3000, 3700, 100);

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CurrentPulseControl {
    Disabled = 0b0,
    Enabled = 0b1,
}

integer_type!(FastChargeCurrentLimit, 0, 5056, 64);

integer_type!(PrechargeCurrentLimit, 64, 1024, 64);

integer_type!(TerminationCurrentLimit, 64, 1024, 64);

integer_type!(ChargeVoltageLimit, 3840, 4608, 16);

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum BatteryPrechargeFastChargeThreshold {
    _2800mV = 0b0,
    _3000mV = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum BatteryRechargeThresholdOffset {
    _100mV = 0b0,
    _200mV = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ChargingTermination {
    Disabled = 0b0,
    Enabled = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum StatPin {
    Disabled = 0b1,
    Enabled = 0b0,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum WatchdogTimer {
    Disabled = 0b00,
    _40s = 0b01,
    _80s = 0b10,
    _160s = 0b11,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ChargingSafetyTimer {
    Disabled = 0b0,
    Enabled = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum FastChargeTimer {
    _5h = 0b00,
    _8h = 0b01,
    _12h = 0b10,
    _20h = 0b11,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum IRCompensationResistor {
    Disable = 0b000,
    _20mOhm = 0b001,
    _40mOhm = 0b010,
    _60mOhm = 0b011,
    _80mOhm = 0b100,
    _100mOhm = 0b101,
    _120mOhm = 0b110,
    _140mOhm = 0b111,
}

integer_type!(IRCompensationVoltageClamp, 0, 224, 32);

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ThermalRegulationThreshold {
    _60C = 0b00,
    _80C = 0b01,
    _100C = 0b10,
    _120C = 0b11,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum SafetyTimerSpeedDuringDpmOrThermalRegulation {
    Normal = 0b0,
    Half = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum BatteryFetMode {
    Normal = 0b0,
    ForceOff = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum BatteryFetOffDelay {
    /// Turns off immediately
    Immediate = 0b0,
    /// Delayed by t_sm_dly
    Delayed = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum BatteryFetFullSystemReset {
    Disable = 0b0,
    Enable = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CurrentPulseControlVoltageUp {
    Disable = 0b0,
    Enable = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CurrentPulseControlVoltageDown {
    Disable = 0b0,
    Enable = 0b1,
}

integer_type!(BoostModeVoltage, 4550, 5510, 64);

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum VbusStatus {
    NoInput = 0b000,
    UsbHostSDP = 0b001,
    UsbCdp = 0b010,
    UsbDcp = 0b011,
    AdjustableHighVoltageDcp = 0b100,
    UnknownAdapter = 0b101,
    NonStandardAdapter = 0b110,
    Otg = 0b111,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ChargingStatus {
    NotCharging = 0b00,
    PreCharging = 0b01,
    FastCharging = 0b10,
    ChargeTerminationDone = 0b11,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum PowerGood {
    NotGood = 0b0,
    Good = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum UsbInput {
    Usb100 = 0b0,
    Usb500 = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum VsysRegulation {
    /// V_bat > V_sysmin
    Inactive = 0b0,
    /// V_bat < V_sysmin
    Active = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum WatchdogStatus {
    Normal = 0b0,
    TimerExpired = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum BoostModeStatus {
    Normal = 0b0,
    /// One of: V_bus overloaded in OTG, V_bus over-voltage, battery too low in boost mode
    Fault = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ChargeStatus {
    Normal = 0b00,
    /// One of: V_bus > V_acov, V_bat < V_bus < V_busmin
    InputFault = 0b01,
    ThermalShutdown = 0b10,
    SafetyTimerExpired = 0b11,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum BatteryStatus {
    Normal = 0b0,
    /// V_bat > V_batovp
    OvervoltageProtection = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum NtcStatus {
    Normal = 0b000,

    BuckCold = 0b001,
    BuckHot = 0b010,

    BoostCold = 0b101,
    BoostHot = 0b110,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum VinDpmThresholdType {
    Relative = 0b0,
    Absolute = 0b1,
}

#[nutype(
    derive(Debug, Copy, Clone, PartialEq, Eq),
    derive_unchecked(Format),
    validate(
        greater_or_equal = 3900,
        less_or_equal = 15300,
        predicate = |n| n.checked_sub(3900).is_some_and(|n| n.is_multiple_of(100)),
    ),
)]
pub struct VinDpmAbsoluteThreshold(u32);

impl TryFrom<u8> for VinDpmAbsoluteThreshold {
    type Error = VinDpmAbsoluteThresholdError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let value = (value as u32)
            .checked_sub(0b0001101)
            .ok_or(VinDpmAbsoluteThresholdError::PredicateViolated)?;
        Self::try_new(3900 + (value * 100))
    }
}

impl From<VinDpmAbsoluteThreshold> for u8 {
    fn from(value: VinDpmAbsoluteThreshold) -> Self {
        let value: u8 = ((value.into_inner() - 3900) / 100).try_into().unwrap();
        value + 0b0001101
    }
}

#[cfg(test)]
mod vin_dpm_absolute_threshold_tests {
    use super::VinDpmAbsoluteThreshold;

    // Raw register offset for the minimum voltage (3900 mV).
    const RAW_MIN: u8 = 0b0001101;

    #[test]
    fn try_new_minimum() {
        assert!(VinDpmAbsoluteThreshold::try_new(3900).is_ok());
    }

    #[test]
    fn try_new_maximum() {
        assert!(VinDpmAbsoluteThreshold::try_new(15300).is_ok());
    }

    #[test]
    fn try_new_mid_range() {
        assert!(VinDpmAbsoluteThreshold::try_new(4800).is_ok());
    }

    #[test]
    fn try_new_below_minimum_rejected() {
        assert!(VinDpmAbsoluteThreshold::try_new(3899).is_err());
    }

    #[test]
    fn try_new_above_maximum_rejected() {
        assert!(VinDpmAbsoluteThreshold::try_new(15301).is_err());
    }

    #[test]
    fn try_new_non_multiple_of_100_rejected() {
        // 3950 is in range but not a 100 mV step above 3900.
        assert!(VinDpmAbsoluteThreshold::try_new(3950).is_err());
    }

    #[test]
    fn try_from_u8_minimum_raw() {
        let threshold = VinDpmAbsoluteThreshold::try_from(RAW_MIN).unwrap();
        assert_eq!(threshold.into_inner(), 3900);
    }

    #[test]
    fn try_from_u8_one_step_above_minimum() {
        let threshold = VinDpmAbsoluteThreshold::try_from(RAW_MIN + 1).unwrap();
        assert_eq!(threshold.into_inner(), 4000);
    }

    #[test]
    fn try_from_u8_maximum_raw() {
        // (15300 - 3900) / 100 + 13 = 127
        let threshold = VinDpmAbsoluteThreshold::try_from(127).unwrap();
        assert_eq!(threshold.into_inner(), 15300);
    }

    #[test]
    fn try_from_u8_below_offset_rejected() {
        // Any raw value below 13 underflows the offset subtraction.
        assert!(VinDpmAbsoluteThreshold::try_from(RAW_MIN - 1).is_err());
        assert!(VinDpmAbsoluteThreshold::try_from(0).is_err());
    }

    #[test]
    fn try_from_u8_above_maximum_raw_rejected() {
        // Raw 128 would decode to 15400 mV, which exceeds 15300 mV.
        assert!(VinDpmAbsoluteThreshold::try_from(128).is_err());
    }

    #[test]
    fn into_u8_minimum() {
        let threshold = VinDpmAbsoluteThreshold::try_new(3900).unwrap();
        assert_eq!(u8::from(threshold), RAW_MIN);
    }

    #[test]
    fn into_u8_maximum() {
        let threshold = VinDpmAbsoluteThreshold::try_new(15300).unwrap();
        assert_eq!(u8::from(threshold), 127);
    }

    #[test]
    fn into_u8_mid_range() {
        let threshold = VinDpmAbsoluteThreshold::try_new(4800).unwrap();
        // (4800 - 3900) / 100 + 13 = 9 + 13 = 22
        assert_eq!(u8::from(threshold), 22);
    }

    #[test]
    fn roundtrip_raw_to_threshold_to_raw() {
        for raw in RAW_MIN..=127 {
            let threshold = VinDpmAbsoluteThreshold::try_from(raw).unwrap();
            assert_eq!(u8::from(threshold), raw);
        }
    }

    #[test]
    fn roundtrip_threshold_to_raw_to_threshold() {
        for mv in (3900u32..=15300).step_by(100) {
            let threshold = VinDpmAbsoluteThreshold::try_new(mv).unwrap();
            let raw = u8::from(threshold);
            let recovered = VinDpmAbsoluteThreshold::try_from(raw).unwrap();
            assert_eq!(threshold, recovered);
        }
    }
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ThermalRegulationStatus {
    Normal = 0b0,
    Active = 0b1,
}

integer_type!(BatteryVoltage, 2304, 4844, 20);

#[cfg(test)]
mod battery_voltage_tests {
    use super::BatteryVoltage;

    #[test]
    fn try_new_minimum() {
        assert!(BatteryVoltage::try_new(2304).is_ok());
    }

    #[test]
    fn try_new_one_step_above_minimum() {
        assert!(BatteryVoltage::try_new(2324).is_ok());
    }

    #[test]
    fn try_new_practical_maximum() {
        // Highest value reachable in 20 mV steps from 2304.
        assert!(BatteryVoltage::try_new(4844).is_ok());
    }

    #[test]
    fn try_new_below_minimum_rejected() {
        assert!(BatteryVoltage::try_new(2303).is_err());
    }

    #[test]
    fn try_new_above_nutype_bound_rejected() {
        assert!(BatteryVoltage::try_new(4849).is_err());
    }

    #[test]
    fn try_new_non_multiple_of_step_rejected() {
        // 2310 is in range but (2310-2304)=6 is not a multiple of 20.
        assert!(BatteryVoltage::try_new(2310).is_err());
    }

    #[test]
    fn try_from_u8_zero_raw() {
        let v = BatteryVoltage::try_from(0u8).unwrap();
        assert_eq!(v.into_inner(), 2304);
    }

    #[test]
    fn try_from_u8_one_raw() {
        let v = BatteryVoltage::try_from(1u8).unwrap();
        assert_eq!(v.into_inner(), 2324);
    }

    #[test]
    fn try_from_u8_maximum_raw() {
        // 2304 + 127*20 = 4844
        let v = BatteryVoltage::try_from(127u8).unwrap();
        assert_eq!(v.into_inner(), 4844);
    }

    #[test]
    fn try_from_u8_above_maximum_raw_rejected() {
        // 2304 + 128*20 = 4864 > 4848
        assert!(BatteryVoltage::try_from(128u8).is_err());
    }

    #[test]
    fn into_u8_minimum() {
        let v = BatteryVoltage::try_new(2304).unwrap();
        assert_eq!(u8::from(v), 0);
    }

    #[test]
    fn into_u8_one_step() {
        let v = BatteryVoltage::try_new(2324).unwrap();
        assert_eq!(u8::from(v), 1);
    }

    #[test]
    fn into_u8_practical_maximum() {
        let v = BatteryVoltage::try_new(4844).unwrap();
        assert_eq!(u8::from(v), 127);
    }

    #[test]
    fn roundtrip_raw_to_voltage_to_raw() {
        for raw in 0u8..=127 {
            let v = BatteryVoltage::try_from(raw).unwrap();
            assert_eq!(u8::from(v), raw);
        }
    }

    #[test]
    fn roundtrip_voltage_to_raw_to_voltage() {
        for mv in (2304u32..=4844).step_by(20) {
            let v = BatteryVoltage::try_new(mv).unwrap();
            let raw = u8::from(v);
            let recovered = BatteryVoltage::try_from(raw).unwrap();
            assert_eq!(v, recovered);
        }
    }
}

integer_type!(SystemVoltage, 2304, 4848, 20);

/// TS voltage as a percentage of REGN
#[nutype(
    derive(Debug, Copy, Clone, PartialEq),
    derive_unchecked(Format),
    validate(greater_or_equal = 21, less_or_equal = 81)
)]
pub struct TsVoltagePct(f32);

impl TryFrom<u8> for TsVoltagePct {
    type Error = TsVoltagePctError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_new(21.0 + ((value as f32) * 0.465))
    }
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum VbusGood {
    NotGood = 0b0,
    Good = 0b1,
}

integer_type!(VbusVoltage, 2600, 15300, 100);

integer_type!(ChargeCurrent, 0, 6350, 50);

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum VinDpm {
    Idle = 0b0,
    Active = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum IinDpm {
    Idle = 0b0,
    Active = 0b1,
}

integer_type!(IcoInputCurrentLimit, 100, 3250, 50);

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum InputCurrentOptimizerStatus {
    InProgress = 0b0,
    /// Maximum input current detected
    Done = 0b1,
}
