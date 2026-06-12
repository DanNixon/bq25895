use defmt::Format;
use num_enum::{IntoPrimitive, TryFromPrimitive};

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

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum InputCurrentLimit {
    _100mA = 0b000000,
    _150mA = 0b000001,
    _200mA = 0b000010,
    _250mA = 0b000011,
    _300mA = 0b000100,
    _350mA = 0b000101,
    _400mA = 0b000110,
    _450mA = 0b000111,
    _500mA = 0b001000,
    _550mA = 0b001001,
    _600mA = 0b001010,
    _650mA = 0b001011,
    _700mA = 0b001100,
    _750mA = 0b001101,
    _800mA = 0b001110,
    _850mA = 0b001111,
    _900mA = 0b010000,
    _950mA = 0b010001,
    _1000mA = 0b010010,
    _1050mA = 0b010011,
    _1100mA = 0b010100,
    _1150mA = 0b010101,
    _1200mA = 0b010110,
    _1250mA = 0b010111,
    _1300mA = 0b011000,
    _1350mA = 0b011001,
    _1400mA = 0b011010,
    _1450mA = 0b011011,
    _1500mA = 0b011100,
    _1550mA = 0b011101,
    _1600mA = 0b011110,
    _1650mA = 0b011111,
    _1700mA = 0b100000,
    _1750mA = 0b100001,
    _1800mA = 0b100010,
    _1850mA = 0b100011,
    _1900mA = 0b100100,
    _1950mA = 0b100101,
    _2000mA = 0b100110,
    _2050mA = 0b100111,
    _2100mA = 0b101000,
    _2150mA = 0b101001,
    _2200mA = 0b101010,
    _2250mA = 0b101011,
    _2300mA = 0b101100,
    _2350mA = 0b101101,
    _2400mA = 0b101110,
    _2450mA = 0b101111,
    _2500mA = 0b110000,
    _2550mA = 0b110001,
    _2600mA = 0b110010,
    _2650mA = 0b110011,
    _2700mA = 0b110100,
    _2750mA = 0b110101,
    _2800mA = 0b110110,
    _2850mA = 0b110111,
    _2900mA = 0b111000,
    _2950mA = 0b111001,
    _3000mA = 0b111010,
    _3050mA = 0b111011,
    _3100mA = 0b111100,
    _3150mA = 0b111101,
    _3200mA = 0b111110,
    _3250mA = 0b111111,
}

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

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum InputVoltageLimitOffset {
    _0mV = 0b00000,
    _100mV = 0b00001,
    _200mV = 0b00010,
    _300mV = 0b00011,
    _400mV = 0b00100,
    _500mV = 0b00101,
    _600mV = 0b00110,
    _700mV = 0b00111,
    _800mV = 0b01000,
    _900mV = 0b01001,
    _1000mV = 0b01010,
    _1100mV = 0b01011,
    _1200mV = 0b01100,
    _1300mV = 0b01101,
    _1400mV = 0b01110,
    _1500mV = 0b01111,
    _1600mV = 0b10000,
    _1700mV = 0b10001,
    _1800mV = 0b10010,
    _1900mV = 0b10011,
    _2000mV = 0b10100,
    _2100mV = 0b10101,
    _2200mV = 0b10110,
    _2300mV = 0b10111,
    _2400mV = 0b11000,
    _2500mV = 0b11001,
    _2600mV = 0b11010,
    _2700mV = 0b11011,
    _2800mV = 0b11100,
    _2900mV = 0b11101,
    _3000mV = 0b11110,
    _3100mV = 0b11111,
}

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

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum MinimumSystemVoltageLimit {
    _3000mV = 0b000,
    _3100mV = 0b001,
    _3200mV = 0b010,
    _3300mV = 0b011,
    _3400mV = 0b100,
    _3500mV = 0b101,
    _3600mV = 0b110,
    _3700mV = 0b111,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CurrentPulseControl {
    Disabled = 0b0,
    Enabled = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum FastChargeCurrentLimit {
    _0mA = 0b0000000,
    _64mA = 0b0000001,
    _128mA = 0b0000010,
    _192mA = 0b0000011,
    _256mA = 0b0000100,
    _320mA = 0b0000101,
    _384mA = 0b0000110,
    _448mA = 0b0000111,
    _512mA = 0b0001000,
    _576mA = 0b0001001,
    _640mA = 0b0001010,
    _704mA = 0b0001011,
    _768mA = 0b0001100,
    _832mA = 0b0001101,
    _896mA = 0b0001110,
    _960mA = 0b0001111,
    _1024mA = 0b0010000,
    _1088mA = 0b0010001,
    _1152mA = 0b0010010,
    _1216mA = 0b0010011,
    _1280mA = 0b0010100,
    _1344mA = 0b0010101,
    _1408mA = 0b0010110,
    _1472mA = 0b0010111,
    _1536mA = 0b0011000,
    _1600mA = 0b0011001,
    _1664mA = 0b0011010,
    _1728mA = 0b0011011,
    _1792mA = 0b0011100,
    _1856mA = 0b0011101,
    _1920mA = 0b0011110,
    _1948mA = 0b0011111,
    _2048mA = 0b0100000,
    _2112mA = 0b0100001,
    _2176mA = 0b0100010,
    _2240mA = 0b0100011,
    _2304mA = 0b0100100,
    _2368mA = 0b0100101,
    _2432mA = 0b0100110,
    _2496mA = 0b0100111,
    _2560mA = 0b0101000,
    _2624mA = 0b0101001,
    _2688mA = 0b0101010,
    _2752mA = 0b0101011,
    _2816mA = 0b0101100,
    _2880mA = 0b0101101,
    _2944mA = 0b0101110,
    _3008mA = 0b0101111,
    _3072mA = 0b0110000,
    _3136mA = 0b0110001,
    _3200mA = 0b0110010,
    _3264mA = 0b0110011,
    _3328mA = 0b0110100,
    _3392mA = 0b0110101,
    _3456mA = 0b0110110,
    _3520mA = 0b0110111,
    _3584mA = 0b0111000,
    _3648mA = 0b0111001,
    _3712mA = 0b0111010,
    _3776mA = 0b0111011,
    _3840mA = 0b0111100,
    _3904mA = 0b0111101,
    _3968mA = 0b0111110,
    _4032mA = 0b0111111,
    _4096mA = 0b1000000,
    _4160mA = 0b1000001,
    _4224mA = 0b1000010,
    _4288mA = 0b1000011,
    _4352mA = 0b1000100,
    _4416mA = 0b1000101,
    _4480mA = 0b1000110,
    _4544mA = 0b1000111,
    _4608mA = 0b1001000,
    _4672mA = 0b1001001,
    _4736mA = 0b1001010,
    _4800mA = 0b1001011,
    _4864mA = 0b1001100,
    _4928mA = 0b1001101,
    _4992mA = 0b1001110,
    _5056mA = 0b1001111,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum PrechargeCurrentLimit {
    _64mA = 0b0000,
    _128mA = 0b0001,
    _192mA = 0b0010,
    _256mA = 0b0011,
    _320mA = 0b0100,
    _384mA = 0b0101,
    _448mA = 0b0110,
    _512mA = 0b0111,
    _576mA = 0b1000,
    _640mA = 0b1001,
    _704mA = 0b1010,
    _768mA = 0b1011,
    _832mA = 0b1100,
    _896mA = 0b1101,
    _960mA = 0b1110,
    _1024mA = 0b1111,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum TerminationCurrentLimit {
    _64mA = 0b0000,
    _128mA = 0b0001,
    _192mA = 0b0010,
    _256mA = 0b0011,
    _320mA = 0b0100,
    _384mA = 0b0101,
    _448mA = 0b0110,
    _512mA = 0b0111,
    _576mA = 0b1000,
    _640mA = 0b1001,
    _704mA = 0b1010,
    _768mA = 0b1011,
    _832mA = 0b1100,
    _896mA = 0b1101,
    _960mA = 0b1110,
    _1024mA = 0b1111,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ChargeVoltageLimit {
    _3840mV = 0b000000,
    _3856mV = 0b000001,
    _3872mV = 0b000010,
    _3888mV = 0b000011,
    _3904mV = 0b000100,
    _3920mV = 0b000101,
    _3936mV = 0b000110,
    _3952mV = 0b000111,
    _3968mV = 0b001000,
    _3984mV = 0b001001,
    _4000mV = 0b001010,
    _4016mV = 0b001011,
    _4032mV = 0b001100,
    _4048mV = 0b001101,
    _4064mV = 0b001110,
    _4080mV = 0b001111,
    _4096mV = 0b010000,
    _4112mV = 0b010001,
    _4128mV = 0b010010,
    _4144mV = 0b010011,
    _4160mV = 0b010100,
    _4176mV = 0b010101,
    _4192mV = 0b010110,
    _4208mV = 0b010111,
    _4224mV = 0b011000,
    _4240mV = 0b011001,
    _4256mV = 0b011010,
    _4272mV = 0b011011,
    _4288mV = 0b011100,
    _4304mV = 0b011101,
    _4320mV = 0b011110,
    _4336mV = 0b011111,
    _4352mV = 0b100000,
    _4368mV = 0b100001,
    _4384mV = 0b100010,
    _4400mV = 0b100011,
    _4416mV = 0b100100,
    _4432mV = 0b100101,
    _4448mV = 0b100110,
    _4464mV = 0b100111,
    _4480mV = 0b101000,
    _4496mV = 0b101001,
    _4512mV = 0b101010,
    _4528mV = 0b101011,
    _4544mV = 0b101100,
    _4560mV = 0b101101,
    _4576mV = 0b101110,
    _4592mV = 0b101111,
    _4608mV = 0b110000,
}

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

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum IRCompensationVoltageClamp {
    _0mV = 0b000,
    _32mV = 0b001,
    _64mV = 0b010,
    _96mV = 0b011,
    _128mV = 0b100,
    _160mV = 0b101,
    _192mV = 0b110,
    _224mV = 0b111,
}

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

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum BoostModeVoltage {
    _4550mV = 0b0000,
    _4614mV = 0b0001,
    _4678mV = 0b0010,
    _4742mV = 0b0011,
    _4806mV = 0b0100,
    _4870mV = 0b0101,
    _4934mV = 0b0110,
    _4998mV = 0b0111,
    _5062mV = 0b1000,
    _5126mV = 0b1001,
    _5190mV = 0b1010,
    _5254mV = 0b1011,
    _5318mV = 0b1100,
    _5382mV = 0b1101,
    _5446mV = 0b1110,
    _5510mV = 0b1111,
}

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

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum VinDpmAbsoluteThreshold {
    _3900mV = 0b0001101,
    _4000mV = 0b0001110,
    _4100mV = 0b0001111,
    _4200mV = 0b0010000,
    _4300mV = 0b0010001,
    _4400mV = 0b0010010,
    _4500mV = 0b0010011,
    _4600mV = 0b0010100,
    _4700mV = 0b0010101,
    _4800mV = 0b0010110,
    _4900mV = 0b0010111,
    _5000mV = 0b0011000,
    _5100mV = 0b0011001,
    _5200mV = 0b0011010,
    _5300mV = 0b0011011,
    _5400mV = 0b0011100,
    _5500mV = 0b0011101,
    _5600mV = 0b0011110,
    _5700mV = 0b0011111,
    _5800mV = 0b0100000,
    _5900mV = 0b0100001,
    _6000mV = 0b0100010,
    _6100mV = 0b0100011,
    _6200mV = 0b0100100,
    _6300mV = 0b0100101,
    _6400mV = 0b0100110,
    _6500mV = 0b0100111,
    _6600mV = 0b0101000,
    _6700mV = 0b0101001,
    _6800mV = 0b0101010,
    _6900mV = 0b0101011,
    _7000mV = 0b0101100,
    _7100mV = 0b0101101,
    _7200mV = 0b0101110,
    _7300mV = 0b0101111,
    _7400mV = 0b0110000,
    _7500mV = 0b0110001,
    _7600mV = 0b0110010,
    _7700mV = 0b0110011,
    _7800mV = 0b0110100,
    _7900mV = 0b0110101,
    _8000mV = 0b0110110,
    _8100mV = 0b0110111,
    _8200mV = 0b0111000,
    _8300mV = 0b0111001,
    _8400mV = 0b0111010,
    _8500mV = 0b0111011,
    _8600mV = 0b0111100,
    _8700mV = 0b0111101,
    _8800mV = 0b0111110,
    _8900mV = 0b0111111,
    _9000mV = 0b1000000,
    _9100mV = 0b1000001,
    _9200mV = 0b1000010,
    _9300mV = 0b1000011,
    _9400mV = 0b1000100,
    _9500mV = 0b1000101,
    _9600mV = 0b1000110,
    _9700mV = 0b1000111,
    _9800mV = 0b1001000,
    _9900mV = 0b1001001,
    _10000mV = 0b1001010,
    _10100mV = 0b1001011,
    _10200mV = 0b1001100,
    _10300mV = 0b1001101,
    _10400mV = 0b1001110,
    _10500mV = 0b1001111,
    _10600mV = 0b1010000,
    _10700mV = 0b1010001,
    _10800mV = 0b1010010,
    _10900mV = 0b1010011,
    _11000mV = 0b1010100,
    _11100mV = 0b1010101,
    _11200mV = 0b1010110,
    _11300mV = 0b1010111,
    _11400mV = 0b1011000,
    _11500mV = 0b1011001,
    _11600mV = 0b1011010,
    _11700mV = 0b1011011,
    _11800mV = 0b1011100,
    _11900mV = 0b1011101,
    _12000mV = 0b1011110,
    _12100mV = 0b1011111,
    _12200mV = 0b1100000,
    _12300mV = 0b1100001,
    _12400mV = 0b1100010,
    _12500mV = 0b1100011,
    _12600mV = 0b1100100,
    _12700mV = 0b1100101,
    _12800mV = 0b1100110,
    _12900mV = 0b1100111,
    _13000mV = 0b1101000,
    _13100mV = 0b1101001,
    _13200mV = 0b1101010,
    _13300mV = 0b1101011,
    _13400mV = 0b1101100,
    _13500mV = 0b1101101,
    _13600mV = 0b1101110,
    _13700mV = 0b1101111,
    _13800mV = 0b1110000,
    _13900mV = 0b1110001,
    _14000mV = 0b1110010,
    _14100mV = 0b1110011,
    _14200mV = 0b1110100,
    _14300mV = 0b1110101,
    _14400mV = 0b1110110,
    _14500mV = 0b1110111,
    _14600mV = 0b1111000,
    _14700mV = 0b1111001,
    _14800mV = 0b1111010,
    _14900mV = 0b1111011,
    _15000mV = 0b1111100,
    _15100mV = 0b1111101,
    _15200mV = 0b1111110,
    _15300mV = 0b1111111,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ThermalRegulationStatus {
    Normal = 0b0,
    Active = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum BatteryVoltage {
    _2304mV = 0b0000000,
    _2324mV = 0b0000001,
    _2344mV = 0b0000010,
    _2364mV = 0b0000011,
    _2384mV = 0b0000100,
    _2404mV = 0b0000101,
    _2424mV = 0b0000110,
    _2444mV = 0b0000111,
    _2464mV = 0b0001000,
    _2484mV = 0b0001001,
    _2504mV = 0b0001010,
    _2524mV = 0b0001011,
    _2544mV = 0b0001100,
    _2564mV = 0b0001101,
    _2584mV = 0b0001110,
    _2604mV = 0b0001111,
    _2624mV = 0b0010000,
    _2644mV = 0b0010001,
    _2664mV = 0b0010010,
    _2684mV = 0b0010011,
    _2704mV = 0b0010100,
    _2724mV = 0b0010101,
    _2744mV = 0b0010110,
    _2764mV = 0b0010111,
    _2784mV = 0b0011000,
    _2804mV = 0b0011001,
    _2824mV = 0b0011010,
    _2844mV = 0b0011011,
    _2864mV = 0b0011100,
    _2884mV = 0b0011101,
    _2904mV = 0b0011110,
    _2924mV = 0b0011111,
    _2944mV = 0b0100000,
    _2964mV = 0b0100001,
    _2984mV = 0b0100010,
    _3004mV = 0b0100011,
    _3024mV = 0b0100100,
    _3044mV = 0b0100101,
    _3064mV = 0b0100110,
    _3084mV = 0b0100111,
    _3104mV = 0b0101000,
    _3124mV = 0b0101001,
    _3144mV = 0b0101010,
    _3164mV = 0b0101011,
    _3184mV = 0b0101100,
    _3204mV = 0b0101101,
    _3224mV = 0b0101110,
    _3244mV = 0b0101111,
    _3264mV = 0b0110000,
    _3284mV = 0b0110001,
    _3304mV = 0b0110010,
    _3324mV = 0b0110011,
    _3344mV = 0b0110100,
    _3364mV = 0b0110101,
    _3384mV = 0b0110110,
    _3404mV = 0b0110111,
    _3424mV = 0b0111000,
    _3444mV = 0b0111001,
    _3464mV = 0b0111010,
    _3484mV = 0b0111011,
    _3504mV = 0b0111100,
    _3524mV = 0b0111101,
    _3544mV = 0b0111110,
    _3564mV = 0b0111111,
    _3584mV = 0b1000000,
    _3604mV = 0b1000001,
    _3624mV = 0b1000010,
    _3644mV = 0b1000011,
    _3664mV = 0b1000100,
    _3684mV = 0b1000101,
    _3704mV = 0b1000110,
    _3724mV = 0b1000111,
    _3744mV = 0b1001000,
    _3764mV = 0b1001001,
    _3784mV = 0b1001010,
    _3804mV = 0b1001011,
    _3824mV = 0b1001100,
    _3844mV = 0b1001101,
    _3864mV = 0b1001110,
    _3884mV = 0b1001111,
    _3904mV = 0b1010000,
    _3924mV = 0b1010001,
    _3944mV = 0b1010010,
    _3964mV = 0b1010011,
    _3984mV = 0b1010100,
    _4004mV = 0b1010101,
    _4024mV = 0b1010110,
    _4044mV = 0b1010111,
    _4064mV = 0b1011000,
    _4084mV = 0b1011001,
    _4104mV = 0b1011010,
    _4124mV = 0b1011011,
    _4144mV = 0b1011100,
    _4164mV = 0b1011101,
    _4184mV = 0b1011110,
    _4204mV = 0b1011111,
    _4224mV = 0b1100000,
    _4244mV = 0b1100001,
    _4264mV = 0b1100010,
    _4284mV = 0b1100011,
    _4304mV = 0b1100100,
    _4324mV = 0b1100101,
    _4344mV = 0b1100110,
    _4364mV = 0b1100111,
    _4384mV = 0b1101000,
    _4404mV = 0b1101001,
    _4424mV = 0b1101010,
    _4444mV = 0b1101011,
    _4464mV = 0b1101100,
    _4484mV = 0b1101101,
    _4504mV = 0b1101110,
    _4524mV = 0b1101111,
    _4544mV = 0b1110000,
    _4564mV = 0b1110001,
    _4584mV = 0b1110010,
    _4604mV = 0b1110011,
    _4624mV = 0b1110100,
    _4644mV = 0b1110101,
    _4664mV = 0b1110110,
    _4684mV = 0b1110111,
    _4704mV = 0b1111000,
    _4724mV = 0b1111001,
    _4744mV = 0b1111010,
    _4764mV = 0b1111011,
    _4784mV = 0b1111100,
    _4804mV = 0b1111101,
    _4824mV = 0b1111110,
    _4848mV = 0b1111111,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum SystemVoltage {
    _2304mV = 0b0000000,
    _2324mV = 0b0000001,
    _2344mV = 0b0000010,
    _2364mV = 0b0000011,
    _2384mV = 0b0000100,
    _2404mV = 0b0000101,
    _2424mV = 0b0000110,
    _2444mV = 0b0000111,
    _2464mV = 0b0001000,
    _2484mV = 0b0001001,
    _2504mV = 0b0001010,
    _2524mV = 0b0001011,
    _2544mV = 0b0001100,
    _2564mV = 0b0001101,
    _2584mV = 0b0001110,
    _2604mV = 0b0001111,
    _2624mV = 0b0010000,
    _2644mV = 0b0010001,
    _2664mV = 0b0010010,
    _2684mV = 0b0010011,
    _2704mV = 0b0010100,
    _2724mV = 0b0010101,
    _2744mV = 0b0010110,
    _2764mV = 0b0010111,
    _2784mV = 0b0011000,
    _2804mV = 0b0011001,
    _2824mV = 0b0011010,
    _2844mV = 0b0011011,
    _2864mV = 0b0011100,
    _2884mV = 0b0011101,
    _2904mV = 0b0011110,
    _2924mV = 0b0011111,
    _2944mV = 0b0100000,
    _2964mV = 0b0100001,
    _2984mV = 0b0100010,
    _3004mV = 0b0100011,
    _3024mV = 0b0100100,
    _3044mV = 0b0100101,
    _3064mV = 0b0100110,
    _3084mV = 0b0100111,
    _3104mV = 0b0101000,
    _3124mV = 0b0101001,
    _3144mV = 0b0101010,
    _3164mV = 0b0101011,
    _3184mV = 0b0101100,
    _3204mV = 0b0101101,
    _3224mV = 0b0101110,
    _3244mV = 0b0101111,
    _3264mV = 0b0110000,
    _3284mV = 0b0110001,
    _3304mV = 0b0110010,
    _3324mV = 0b0110011,
    _3344mV = 0b0110100,
    _3364mV = 0b0110101,
    _3384mV = 0b0110110,
    _3404mV = 0b0110111,
    _3424mV = 0b0111000,
    _3444mV = 0b0111001,
    _3464mV = 0b0111010,
    _3484mV = 0b0111011,
    _3504mV = 0b0111100,
    _3524mV = 0b0111101,
    _3544mV = 0b0111110,
    _3564mV = 0b0111111,
    _3584mV = 0b1000000,
    _3604mV = 0b1000001,
    _3624mV = 0b1000010,
    _3644mV = 0b1000011,
    _3664mV = 0b1000100,
    _3684mV = 0b1000101,
    _3704mV = 0b1000110,
    _3724mV = 0b1000111,
    _3744mV = 0b1001000,
    _3764mV = 0b1001001,
    _3784mV = 0b1001010,
    _3804mV = 0b1001011,
    _3824mV = 0b1001100,
    _3844mV = 0b1001101,
    _3864mV = 0b1001110,
    _3884mV = 0b1001111,
    _3904mV = 0b1010000,
    _3924mV = 0b1010001,
    _3944mV = 0b1010010,
    _3964mV = 0b1010011,
    _3984mV = 0b1010100,
    _4004mV = 0b1010101,
    _4024mV = 0b1010110,
    _4044mV = 0b1010111,
    _4064mV = 0b1011000,
    _4084mV = 0b1011001,
    _4104mV = 0b1011010,
    _4124mV = 0b1011011,
    _4144mV = 0b1011100,
    _4164mV = 0b1011101,
    _4184mV = 0b1011110,
    _4204mV = 0b1011111,
    _4224mV = 0b1100000,
    _4244mV = 0b1100001,
    _4264mV = 0b1100010,
    _4284mV = 0b1100011,
    _4304mV = 0b1100100,
    _4324mV = 0b1100101,
    _4344mV = 0b1100110,
    _4364mV = 0b1100111,
    _4384mV = 0b1101000,
    _4404mV = 0b1101001,
    _4424mV = 0b1101010,
    _4444mV = 0b1101011,
    _4464mV = 0b1101100,
    _4484mV = 0b1101101,
    _4504mV = 0b1101110,
    _4524mV = 0b1101111,
    _4544mV = 0b1110000,
    _4564mV = 0b1110001,
    _4584mV = 0b1110010,
    _4604mV = 0b1110011,
    _4624mV = 0b1110100,
    _4644mV = 0b1110101,
    _4664mV = 0b1110110,
    _4684mV = 0b1110111,
    _4704mV = 0b1111000,
    _4724mV = 0b1111001,
    _4744mV = 0b1111010,
    _4764mV = 0b1111011,
    _4784mV = 0b1111100,
    _4804mV = 0b1111101,
    _4824mV = 0b1111110,
    _4848mV = 0b1111111,
}

/// TS voltage as a percentage of REGN
#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum TsVoltagePct {
    _21 = 0b0000000,
    _21_465 = 0b0000001,
    _21_93 = 0b0000010,
    _22_395 = 0b0000011,
    _22_86 = 0b0000100,
    _23_325 = 0b0000101,
    _23_79 = 0b0000110,
    _24_255 = 0b0000111,
    _24_72 = 0b0001000,
    _25_185 = 0b0001001,
    _25_65 = 0b0001010,
    _26_115 = 0b0001011,
    _26_58 = 0b0001100,
    _27_045 = 0b0001101,
    _27_51 = 0b0001110,
    _27_975 = 0b0001111,
    _28_44 = 0b0010000,
    _28_905 = 0b0010001,
    _29_37 = 0b0010010,
    _29_835 = 0b0010011,
    _30_3 = 0b0010100,
    _30_765 = 0b0010101,
    _31_23 = 0b0010110,
    _31_695 = 0b0010111,
    _32_16 = 0b0011000,
    _32_625 = 0b0011001,
    _33_09 = 0b0011010,
    _33_555 = 0b0011011,
    _34_02 = 0b0011100,
    _34_485 = 0b0011101,
    _34_95 = 0b0011110,
    _35_415 = 0b0011111,
    _35_88 = 0b0100000,
    _36_345 = 0b0100001,
    _36_81 = 0b0100010,
    _37_275 = 0b0100011,
    _37_74 = 0b0100100,
    _38_205 = 0b0100101,
    _38_67 = 0b0100110,
    _39_135 = 0b0100111,
    _39_6 = 0b0101000,
    _40_065 = 0b0101001,
    _40_53 = 0b0101010,
    _40_995 = 0b0101011,
    _41_46 = 0b0101100,
    _41_925 = 0b0101101,
    _42_39 = 0b0101110,
    _42_855 = 0b0101111,
    _43_32 = 0b0110000,
    _43_785 = 0b0110001,
    _44_25 = 0b0110010,
    _44_715 = 0b0110011,
    _45_18 = 0b0110100,
    _45_645 = 0b0110101,
    _46_11 = 0b0110110,
    _46_575 = 0b0110111,
    _47_04 = 0b0111000,
    _47_505 = 0b0111001,
    _47_97 = 0b0111010,
    _48_435 = 0b0111011,
    _48_9 = 0b0111100,
    _49_365 = 0b0111101,
    _49_83 = 0b0111110,
    _50_295 = 0b0111111,
    _50_76 = 0b1000000,
    _51_225 = 0b1000001,
    _51_69 = 0b1000010,
    _52_155 = 0b1000011,
    _52_62 = 0b1000100,
    _53_085 = 0b1000101,
    _53_55 = 0b1000110,
    _54_015 = 0b1000111,
    _54_48 = 0b1001000,
    _54_945 = 0b1001001,
    _55_41 = 0b1001010,
    _55_875 = 0b1001011,
    _56_34 = 0b1001100,
    _56_805 = 0b1001101,
    _57_27 = 0b1001110,
    _57_735 = 0b1001111,
    _58_2 = 0b1010000,
    _58_665 = 0b1010001,
    _59_13 = 0b1010010,
    _59_595 = 0b1010011,
    _60_06 = 0b1010100,
    _60_525 = 0b1010101,
    _60_99 = 0b1010110,
    _61_455 = 0b1010111,
    _61_92 = 0b1011000,
    _62_385 = 0b1011001,
    _62_85 = 0b1011010,
    _63_315 = 0b1011011,
    _63_78 = 0b1011100,
    _64_245 = 0b1011101,
    _64_71 = 0b1011110,
    _65_175 = 0b1011111,
    _65_64 = 0b1100000,
    _66_105 = 0b1100001,
    _66_57 = 0b1100010,
    _67_035 = 0b1100011,
    _67_5 = 0b1100100,
    _67_965 = 0b1100101,
    _68_43 = 0b1100110,
    _68_895 = 0b1100111,
    _69_36 = 0b1101000,
    _69_825 = 0b1101001,
    _70_29 = 0b1101010,
    _70_755 = 0b1101011,
    _71_22 = 0b1101100,
    _71_685 = 0b1101101,
    _72_15 = 0b1101110,
    _72_615 = 0b1101111,
    _73_08 = 0b1110000,
    _73_545 = 0b1110001,
    _74_01 = 0b1110010,
    _74_475 = 0b1110011,
    _74_94 = 0b1110100,
    _75_405 = 0b1110101,
    _75_87 = 0b1110110,
    _76_335 = 0b1110111,
    _76_8 = 0b1111000,
    _77_265 = 0b1111001,
    _77_73 = 0b1111010,
    _78_195 = 0b1111011,
    _78_66 = 0b1111100,
    _79_125 = 0b1111101,
    _80_055 = 0b1111110,
    _80_52 = 0b1111111,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum VbusGood {
    NotGood = 0b0,
    Good = 0b1,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum VbusVoltage {
    _2600mV = 0b0000000,
    _2700mV = 0b0000001,
    _2800mV = 0b0000010,
    _2900mV = 0b0000011,
    _3000mV = 0b0000100,
    _3100mV = 0b0000101,
    _3200mV = 0b0000110,
    _3300mV = 0b0000111,
    _3400mV = 0b0001000,
    _3500mV = 0b0001001,
    _3600mV = 0b0001010,
    _3700mV = 0b0001011,
    _3800mV = 0b0001100,
    _3900mV = 0b0001101,
    _4000mV = 0b0001110,
    _4100mV = 0b0001111,
    _4200mV = 0b0010000,
    _4300mV = 0b0010001,
    _4400mV = 0b0010010,
    _4500mV = 0b0010011,
    _4600mV = 0b0010100,
    _4700mV = 0b0010101,
    _4800mV = 0b0010110,
    _4900mV = 0b0010111,
    _5000mV = 0b0011000,
    _5100mV = 0b0011001,
    _5200mV = 0b0011010,
    _5300mV = 0b0011011,
    _5400mV = 0b0011100,
    _5500mV = 0b0011101,
    _5600mV = 0b0011110,
    _5700mV = 0b0011111,
    _5800mV = 0b0100000,
    _5900mV = 0b0100001,
    _6000mV = 0b0100010,
    _6100mV = 0b0100011,
    _6200mV = 0b0100100,
    _6300mV = 0b0100101,
    _6400mV = 0b0100110,
    _6500mV = 0b0100111,
    _6600mV = 0b0101000,
    _6700mV = 0b0101001,
    _6800mV = 0b0101010,
    _6900mV = 0b0101011,
    _7000mV = 0b0101100,
    _7100mV = 0b0101101,
    _7200mV = 0b0101110,
    _7300mV = 0b0101111,
    _7400mV = 0b0110000,
    _7500mV = 0b0110001,
    _7600mV = 0b0110010,
    _7700mV = 0b0110011,
    _7800mV = 0b0110100,
    _7900mV = 0b0110101,
    _8000mV = 0b0110110,
    _8100mV = 0b0110111,
    _8200mV = 0b0111000,
    _8300mV = 0b0111001,
    _8400mV = 0b0111010,
    _8500mV = 0b0111011,
    _8600mV = 0b0111100,
    _8700mV = 0b0111101,
    _8800mV = 0b0111110,
    _8900mV = 0b0111111,
    _9000mV = 0b1000000,
    _9100mV = 0b1000001,
    _9200mV = 0b1000010,
    _9300mV = 0b1000011,
    _9400mV = 0b1000100,
    _9500mV = 0b1000101,
    _9600mV = 0b1000110,
    _9700mV = 0b1000111,
    _9800mV = 0b1001000,
    _9900mV = 0b1001001,
    _10000mV = 0b1001010,
    _10100mV = 0b1001011,
    _10200mV = 0b1001100,
    _10300mV = 0b1001101,
    _10400mV = 0b1001110,
    _10500mV = 0b1001111,
    _10600mV = 0b1010000,
    _10700mV = 0b1010001,
    _10800mV = 0b1010010,
    _10900mV = 0b1010011,
    _11000mV = 0b1010100,
    _11100mV = 0b1010101,
    _11200mV = 0b1010110,
    _11300mV = 0b1010111,
    _11400mV = 0b1011000,
    _11500mV = 0b1011001,
    _11600mV = 0b1011010,
    _11700mV = 0b1011011,
    _11800mV = 0b1011100,
    _11900mV = 0b1011101,
    _12000mV = 0b1011110,
    _12100mV = 0b1011111,
    _12200mV = 0b1100000,
    _12300mV = 0b1100001,
    _12400mV = 0b1100010,
    _12500mV = 0b1100011,
    _12600mV = 0b1100100,
    _12700mV = 0b1100101,
    _12800mV = 0b1100110,
    _12900mV = 0b1100111,
    _13000mV = 0b1101000,
    _13100mV = 0b1101001,
    _13200mV = 0b1101010,
    _13300mV = 0b1101011,
    _13400mV = 0b1101100,
    _13500mV = 0b1101101,
    _13600mV = 0b1101110,
    _13700mV = 0b1101111,
    _13800mV = 0b1110000,
    _13900mV = 0b1110001,
    _14000mV = 0b1110010,
    _14100mV = 0b1110011,
    _14200mV = 0b1110100,
    _14300mV = 0b1110101,
    _14400mV = 0b1110110,
    _14500mV = 0b1110111,
    _14600mV = 0b1111000,
    _14700mV = 0b1111001,
    _14800mV = 0b1111010,
    _14900mV = 0b1111011,
    _15000mV = 0b1111100,
    _15100mV = 0b1111101,
    _15200mV = 0b1111110,
    _15300mV = 0b1111111,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ChargeCurrent {
    _0mA = 0b0000000,
    _50mA = 0b0000001,
    _100mA = 0b0000010,
    _150mA = 0b0000011,
    _200mA = 0b0000100,
    _250mA = 0b0000101,
    _300mA = 0b0000110,
    _350mA = 0b0000111,
    _400mA = 0b0001000,
    _450mA = 0b0001001,
    _500mA = 0b0001010,
    _550mA = 0b0001011,
    _600mA = 0b0001100,
    _650mA = 0b0001101,
    _700mA = 0b0001110,
    _750mA = 0b0001111,
    _800mA = 0b0010000,
    _850mA = 0b0010001,
    _900mA = 0b0010010,
    _950mA = 0b0010011,
    _1000mA = 0b0010100,
    _1050mA = 0b0010101,
    _1100mA = 0b0010110,
    _1150mA = 0b0010111,
    _1200mA = 0b0011000,
    _1250mA = 0b0011001,
    _1300mA = 0b0011010,
    _1350mA = 0b0011011,
    _1400mA = 0b0011100,
    _1450mA = 0b0011101,
    _1500mA = 0b0011110,
    _1550mA = 0b0011111,
    _1600mA = 0b0100000,
    _1650mA = 0b0100001,
    _1700mA = 0b0100010,
    _1750mA = 0b0100011,
    _1800mA = 0b0100100,
    _1850mA = 0b0100101,
    _1900mA = 0b0100110,
    _1950mA = 0b0100111,
    _2000mA = 0b0101000,
    _2050mA = 0b0101001,
    _2100mA = 0b0101010,
    _2150mA = 0b0101011,
    _2200mA = 0b0101100,
    _2250mA = 0b0101101,
    _2300mA = 0b0101110,
    _2350mA = 0b0101111,
    _2400mA = 0b0110000,
    _2450mA = 0b0110001,
    _2500mA = 0b0110010,
    _2550mA = 0b0110011,
    _2600mA = 0b0110100,
    _2650mA = 0b0110101,
    _2700mA = 0b0110110,
    _2750mA = 0b0110111,
    _2800mA = 0b0111000,
    _2850mA = 0b0111001,
    _2900mA = 0b0111010,
    _2950mA = 0b0111011,
    _3000mA = 0b0111100,
    _3050mA = 0b0111101,
    _3100mA = 0b0111110,
    _3150mA = 0b0111111,
    _3200mA = 0b1000000,
    _3250mA = 0b1000001,
    _3300mA = 0b1000010,
    _3350mA = 0b1000011,
    _3400mA = 0b1000100,
    _3450mA = 0b1000101,
    _3500mA = 0b1000110,
    _3550mA = 0b1000111,
    _3600mA = 0b1001000,
    _3650mA = 0b1001001,
    _3700mA = 0b1001010,
    _3750mA = 0b1001011,
    _3800mA = 0b1001100,
    _3850mA = 0b1001101,
    _3900mA = 0b1001110,
    _3950mA = 0b1001111,
    _4000mA = 0b1010000,
    _4050mA = 0b1010001,
    _4100mA = 0b1010010,
    _4150mA = 0b1010011,
    _4200mA = 0b1010100,
    _4250mA = 0b1010101,
    _4300mA = 0b1010110,
    _4350mA = 0b1010111,
    _4400mA = 0b1011000,
    _4450mA = 0b1011001,
    _4500mA = 0b1011010,
    _4550mA = 0b1011011,
    _4600mA = 0b1011100,
    _4650mA = 0b1011101,
    _4700mA = 0b1011110,
    _4750mA = 0b1011111,
    _4800mA = 0b1100000,
    _4850mA = 0b1100001,
    _4900mA = 0b1100010,
    _4950mA = 0b1100011,
    _5000mA = 0b1100100,
    _5050mA = 0b1100101,
    _5100mA = 0b1100110,
    _5150mA = 0b1100111,
    _5200mA = 0b1101000,
    _5250mA = 0b1101001,
    _5300mA = 0b1101010,
    _5350mA = 0b1101011,
    _5400mA = 0b1101100,
    _5450mA = 0b1101101,
    _5500mA = 0b1101110,
    _5550mA = 0b1101111,
    _5600mA = 0b1110000,
    _5650mA = 0b1110001,
    _5700mA = 0b1110010,
    _5750mA = 0b1110011,
    _5800mA = 0b1110100,
    _5850mA = 0b1110101,
    _5900mA = 0b1110110,
    _5950mA = 0b1110111,
    _6000mA = 0b1111000,
    _6050mA = 0b1111001,
    _6100mA = 0b1111010,
    _6150mA = 0b1111011,
    _6200mA = 0b1111100,
    _6250mA = 0b1111101,
    _6300mA = 0b1111110,
    _6350mA = 0b1111111,
}

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

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum IcoInputCurrentLimit {
    _100mA = 0b000000,
    _150mA = 0b000001,
    _200mA = 0b000010,
    _250mA = 0b000011,
    _300mA = 0b000100,
    _350mA = 0b000101,
    _400mA = 0b000110,
    _450mA = 0b000111,
    _500mA = 0b001000,
    _550mA = 0b001001,
    _600mA = 0b001010,
    _650mA = 0b001011,
    _700mA = 0b001100,
    _750mA = 0b001101,
    _800mA = 0b001110,
    _850mA = 0b001111,
    _900mA = 0b010000,
    _950mA = 0b010001,
    _1000mA = 0b010010,
    _1050mA = 0b010011,
    _1100mA = 0b010100,
    _1150mA = 0b010101,
    _1200mA = 0b010110,
    _1250mA = 0b010111,
    _1300mA = 0b011000,
    _1350mA = 0b011001,
    _1400mA = 0b011010,
    _1450mA = 0b011011,
    _1500mA = 0b011100,
    _1550mA = 0b011101,
    _1600mA = 0b011110,
    _1650mA = 0b011111,
    _1700mA = 0b100000,
    _1750mA = 0b100001,
    _1800mA = 0b100010,
    _1850mA = 0b100011,
    _1900mA = 0b100100,
    _1950mA = 0b100101,
    _2000mA = 0b100110,
    _2050mA = 0b100111,
    _2100mA = 0b101000,
    _2150mA = 0b101001,
    _2200mA = 0b101010,
    _2250mA = 0b101011,
    _2300mA = 0b101100,
    _2350mA = 0b101101,
    _2400mA = 0b101110,
    _2450mA = 0b101111,
    _2500mA = 0b110000,
    _2550mA = 0b110001,
    _2600mA = 0b110010,
    _2650mA = 0b110011,
    _2700mA = 0b110100,
    _2750mA = 0b110101,
    _2800mA = 0b110110,
    _2850mA = 0b110111,
    _2900mA = 0b111000,
    _2950mA = 0b111001,
    _3000mA = 0b111010,
    _3050mA = 0b111011,
    _3100mA = 0b111100,
    _3150mA = 0b111101,
    _3200mA = 0b111110,
    _3250mA = 0b111111,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum InputCurrentOptimizerStatus {
    InProgress = 0b0,
    /// Maximum input current detected
    Done = 0b1,
}
