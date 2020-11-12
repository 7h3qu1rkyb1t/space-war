pub struct Sprite{
    pub data:&'static [u8],
    pub width:u8,
    pub height:u8,
}

// these are raw bytes of sprites
pub static PLAYER_1 : Sprite = Sprite{
    data    :   &[0x04, 0x00, 0x04, 0x00, 0x0e, 0x00, 0x1f, 0x00, 0x3f, 0x80, 0x7f, 0xc0, 0xee, 0xe0, 0x9f, 0x20, 0x35, 0x80, 0x20, 0x80],
    width   :   11,
    height  :   10,
};
pub static PLAYER_2 : Sprite = Sprite{
    data    :   &[0x04, 0x00, 0x04, 0x00, 0x0e, 0x00, 0x1f, 0x00, 0x3f, 0x80, 0x7f, 0xc0, 0xff, 0xe0, 0xff, 0xe0, 0x7f, 0xc0, 0x2e, 0x80],
    width   :   11,
    height  :   10,
};

// pub static BULLET : Sprite = Sprite{
//     data    : &[0x70, 0xf8, 0xf8, 0xf8, 0x70],
//     width   : 5,
//     height  :5
// };

pub static BULLET : Sprite = Sprite{
    data    : &[0x40, 0xe0, 0xe0],
    width   : 3,
    height  :3
};
