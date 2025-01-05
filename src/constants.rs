pub const SINGLE_PACKET: i32 = -1;
pub const MULTI_PACKET: i32 = -2;

// Offsets
pub const OFS_HEADER: usize = 0;
pub const OFS_SP_PAYLOAD: usize = 4;
pub const OFS_MP_ID: usize = 4;
pub const OFS_MP_SS_TOTAL: usize = 8;
pub const OFS_MP_SS_NUMBER: usize = 9;
pub const OFS_MP_SS_SIZE: usize = 10;
pub const OFS_MP_SS_BZ2_SIZE: usize = 12;
pub const OFS_MP_SS_BZ2_CRC: usize = 16;
pub const OFS_MP_SS_PAYLOAD: usize = OFS_MP_SS_BZ2_SIZE;
pub const OFS_MP_SS_PAYLOAD_BZ2: usize = OFS_MP_SS_BZ2_CRC + 4;

#[derive(Debug)]
pub struct PacketFragment {
    pub number: u8,
    pub payload: Vec<u8>,
}
