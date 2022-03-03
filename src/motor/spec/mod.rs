pub mod c6;
pub mod crazy_i;
pub mod d12;
pub mod e12;
pub mod e6;
pub mod e6_rtc;
pub mod f10;
pub mod f15;
pub mod g11;
pub mod g12_rtc;
pub mod g12_st;
pub mod g8_st;
pub mod h13;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MotorSpec {
    C6,
    CrazyI,
    D12,
    E6Rtc,
    E6,
    E12,
    F10,
    F15,
    G8St,
    G11,
    G12Rtc,
    G12St,
    H13,
}

impl MotorSpec {
    pub fn into_raw_thrust(self) -> Vec<[f32; 2]> {
        use MotorSpec::*;
        match self {
            C6 => c6::C6_THRUST.to_vec(),
            CrazyI => crazy_i::CRAZY_I_THRUST.to_vec(),
            D12 => d12::D12_THRUST.to_vec(),
            E6Rtc => e6_rtc::E6_RTC_THRUST.to_vec(),
            E6 => e6::E6_THRUST.to_vec(),
            E12 => e12::E12_THRUST.to_vec(),
            F10 => f10::F10_THRUST.to_vec(),
            F15 => f15::F15_THRUST.to_vec(),
            G8St => g8_st::G8ST_THRUST.to_vec(),
            G11 => g11::G11_THRUST.to_vec(),
            G12Rtc => g12_rtc::G12_RCT_THRUST.to_vec(),
            G12St => g12_st::G12ST_THRUST.to_vec(),
            H13 => h13::H13ST_THRUST.to_vec(),
        }
    }

    pub fn into_interpolated_thrust(self, time_step: f32) -> Vec<f32> {
        super::utils::interpolate_thrust(&self.into_raw_thrust(), time_step)
    }
}
