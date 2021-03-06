#[allow(non_snake_case)]
pub struct Offsets {
    pub m_vecViewOffset: isize,
    pub m_iTeamNum: isize,
    pub m_vecVelocity: isize,
    pub m_lifeState: isize,
    pub m_fFlags: isize,
    pub m_iHealth: isize,
    pub m_nTickBase: isize,
    pub m_nPlayerCond: isize,
    pub m_nPlayerCondEx: isize,
}
unsafe impl Sync for Offsets {}
unsafe impl Send for Offsets {} 


impl Offsets {
    pub fn load(&mut self) {
        // nop
    }
}

pub fn ptr_offset<T, Res>(x: *mut T, offset: isize) -> *mut Res {
    (((x as isize) + offset) as *mut Res)
}

pub static mut OFFSETS: Offsets = Offsets {
    m_vecViewOffset: 0xFC,
    m_iTeamNum: 0x0B0,
    m_vecVelocity: 0x120,
    m_lifeState: 0x0A5,
    m_fFlags: 0x37C,
    m_iHealth: 0x0A8,
    m_nTickBase: 0x1140,
    m_nPlayerCond: 0x17E0 + 0x5B0,
    m_nPlayerCondEx: 0x5B4,
};
