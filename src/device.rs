device_driver::create_device!(
    device_name: Bq25895,

    dsl: {
        config {
            type RegisterAddressType = u8;
            type DefaultByteOrder = LE;
        }

        register REG00 {
            type Access = RW;
            const ADDRESS = 0x00;
            const SIZE_BITS = 8;

            en_hiz: uint as try crate::HizMode = 7..=7,
            en_ilim: uint as try crate::IlimPin = 6..=6,
            iinlim: uint as try crate::InputCurrentLimit = 0..=5,
        },

        register REG01 {
            type Access = RW;
            const ADDRESS = 0x01;
            const SIZE_BITS = 8;

            bhot: uint as try crate::BoostModeHotTemperatureThreshold = 6..=7,
            bcold: uint as try crate::BoostModeColdTemperatureThreshold = 5..=5,
            vindpm_os: uint as try crate::InputVoltageLimitOffset = 0..=4,
        },

        register REG02 {
            type Access = RW;
            const ADDRESS = 0x02;
            const SIZE_BITS = 8;

            conv_rate: uint as try crate::AdcConversionRate = 6..=6,
            boost_freq: uint as try crate::BoostModeFrequency = 5..=5,
            ico_en: uint as try crate::InputCurrentOptimizer = 4..=4,
            hvdcp_en: uint as try crate::HighVoltageDcp = 3..=3,
            maxc_en: uint as try crate::MaxChargeAdapter = 2..=2,
            force_dpdm: uint as try crate::ForceDpDmDetection = 1..=1,
            auto_dpdm_en: uint as try crate::AutomaticDpDmDetection = 0..=0,
        },

        register REG03 {
            type Access = RW;
            const ADDRESS = 0x03;
            const SIZE_BITS = 8;

            bat_loaden: uint as try crate::BatteryLoadEnable = 7..=7,
            wd_rst: uint as try crate::I2cWatchdogReset = 6..=6,
            otc_config: uint as try crate::BoostMode = 5..=5,
            chg_config: uint as try crate::ChargeEnable = 4..=4,
            sys_min: uint as try crate::MinimumSystemVoltageLimit = 1..=3,
        },

        register REG04 {
            type Access = RW;
            const ADDRESS = 0x04;
            const SIZE_BITS = 8;

            en_pumpx: uint as try crate::CurrentPulseControl = 7..=7,
            ichg: uint as try crate::FastChargeCurrentLimit = 0..=6,
        },

        register REG05 {
            type Access = RW;
            const ADDRESS = 0x05;
            const SIZE_BITS = 8;

            iprechg: uint as try crate::PrechargeCurrentLimit = 4..=7,
            iterm: uint as try crate::TerminationCurrentLimit = 0..=3,
        },

        register REG06 {
            type Access = RW;
            const ADDRESS = 0x06;
            const SIZE_BITS = 8;

            vreg: uint as try crate::ChargeVoltageLimit = 2..=7,
            batlowv: uint as try crate::BatteryPrechargeFastChargeThreshold = 1..=1,
            vrechg: uint as try crate::BatteryRechargeThresholdOffset = 0..=0,
        },

        register REG07 {
            type Access = RW;
            const ADDRESS = 0x07;
            const SIZE_BITS = 8;

            en_term: uint as try crate::ChargingTermination = 7..=7,
            stat_dis: uint as try crate::StatPin = 6..=6,
            watchdog: uint as try crate::WatchdogTimer = 4..=5,
            en_timer: uint as try crate::ChargingSafetyTimer = 3..=3,
            chg_timer: uint as try crate::FastChargeTimer = 1..=2,
        },

        register REG08 {
            type Access = RW;
            const ADDRESS = 0x08;
            const SIZE_BITS = 8;

            bat_comp: uint as try crate::IRCompensationResistor = 5..=7,
            vclamp: uint as try crate::IRCompensationVoltageClamp = 2..=4,
            treg: uint as try crate::ThermalRegulationThreshold = 0..=1,
        },

        register REG09 {
            type Access = RW;
            const ADDRESS = 0x09;
            const SIZE_BITS = 8;

            tmr2x_en: uint as try crate::SafetyTimerSpeedDuringDpmOrThermalRegulation = 6..=6,
            batfet_dis: uint as try crate::BatteryFetMode = 5..=5,
            batfet_dly: uint as try crate::BatteryFetOffDelay = 3..=3,
            batfet_rst_en: uint as try crate::BatteryFetFullSystemReset = 2..=2,
            pumpx_up: uint as try crate::CurrentPulseControlVoltageUp = 1..=1,
            pumpx_dn: uint as try crate::CurrentPulseControlVoltageDown = 0..=0,
        },

        register REG0A {
            type Access = RW;
            const ADDRESS = 0x0a;
            const SIZE_BITS = 8;

            boostv: uint as try crate::BoostModeVoltage = 4..=7,
        },

        register REG0B {
            type Access = RO;
            const ADDRESS = 0x0b;
            const SIZE_BITS = 8;

            vbus_stat: uint as try crate::VbusStatus = 5..=7,
            chrg_stat: uint as try crate::ChargingStatus = 3..=4,
            pg_stat: uint as try crate::PowerGood = 2..=2,
            sdp_stat: uint as try crate::UsbInput = 1..=1,
            vsys_stat: uint as try crate::VsysRegulation = 0..=0,
        },

        register REG0C {
            type Access = RO;
            const ADDRESS = 0x0c;
            const SIZE_BITS = 8;

            watchdog_fault: uint as try crate::WatchdogStatus = 7..=7,
            boost_fault: uint as try crate::BoostModeStatus = 6..=6,
            charge_fault: uint as try crate::ChargeStatus = 4..=5,
            bat_fault: uint as try crate::BatteryStatus = 3..=3,
            ntc_fault: uint as try crate::NtcStatus = 0..=2,
        },

        register REG0D {
            type Access = RW;
            const ADDRESS = 0x0d;
            const SIZE_BITS = 8;

            force_vindpm: uint as try crate::VinDpmThresholdType = 7..=7,
            vindpm: uint as try crate::VinDpmAbsoluteThreshold = 0..=6,
        },

        register REG0E {
            type Access = RO;
            const ADDRESS = 0x0e;
            const SIZE_BITS = 8;

            therm_stat: uint as try crate::ThermalRegulationStatus = 7..=7,
            batv: uint as try crate::BatteryVoltage = 0..=6,
        },

        register REG0F {
            type Access = RO;
            const ADDRESS = 0x0f;
            const SIZE_BITS = 8;

            sysv: uint as try crate::SystemVoltage = 0..=6,
        },

        register REG10 {
            type Access = RO;
            const ADDRESS = 0x10;
            const SIZE_BITS = 8;

            tspct: uint as try crate::TsVoltagePct = 0..=6,
        },

        register REG11 {
            type Access = RO;
            const ADDRESS = 0x11;
            const SIZE_BITS = 8;

            vbus_gc: uint as try crate::VbusGood = 7..=7,
            vbusv: uint as try crate::VbusVoltage = 0..=6,
        },

        register REG12 {
            type Access = RO;
            const ADDRESS = 0x12;
            const SIZE_BITS = 8;

            ichgr: uint as try crate::ChargeCurrent = 0..=6,
        },

        register REG13 {
            type Access = RO;
            const ADDRESS = 0x13;
            const SIZE_BITS = 8;

            vdpm_stat: uint as try crate::VinDpm = 7..=7,
            idpm_stat: uint as try crate::IinDpm = 6..=6,
            idpm_lim: uint as try crate::IcoInputCurrentLimit = 0..=5,
        },

        register REG14 {
            type Access = RO;
            const ADDRESS = 0x14;
            const SIZE_BITS = 8;

            ico_optimized: uint as try crate::InputCurrentOptimizerStatus = 6..=6,
            pn: uint = 3..=5,
            ts_profile: uint = 2..=2,
            dev_rev: uint = 0..=1,
        },
    }
);
