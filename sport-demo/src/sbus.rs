#[derive(Default)]
pub struct SbusFrame {
    pub channels: [u16; 16],
    pub channel17: bool,
    pub channel18: bool,
    pub frame_lost: bool,
    pub failsafe: bool
}

#[derive(Default)]
pub struct SbusReadState {
    pub bytei: u16,
    pub frame: SbusFrame,
}

pub fn process_idle(state: &mut SbusReadState) {
    *state = SbusReadState::default();
}

pub fn process_char(state: &mut SbusReadState, c: u8) -> bool {
    if state.bytei == 0 {
        if c == 0x0f {
             state.bytei = state.bytei + 1;
             state.frame = SbusFrame::default();
        }
        false
    } else if state.bytei < 23 {
        let c16: u16 = c.into();
        let biti = (state.bytei - 1) * 8;
        let x = c16 << (biti % 11);
        let y = x >> 11;
        let ci: usize = (biti / 11).into();
        state.frame.channels[ci] = (state.frame.channels[ci] | x) & 0b11111111111;
        if y != 0 {
            state.frame.channels[ci+1] |= y;
        }
        state.bytei = state.bytei + 1;
        false
    } else if state.bytei == 23 {
        state.frame.channel17 = c & 1 != 0;
        state.frame.channel18 = c & 2 != 0;
        state.frame.frame_lost = c & 4 != 0;
        state.frame.failsafe = c & 8 != 0;
        state.bytei = state.bytei + 1;
        false
    } else {
        state.bytei = 0;
        // success if the last byte was 0
        c == 0
    }
}