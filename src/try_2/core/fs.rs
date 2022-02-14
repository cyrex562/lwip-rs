pub const FS_READ_EOF: i32 = -1;
pub const FS_READ_DELAYED: u32 = -2;

#[derive(Clone,Debug,Default)]
pub struct FsDataChecksum {
    pub offset: usize,
    pub chksum: u16,
    pub len: usize,
}

impl FsDataChecksum {
    pub fn new() -> FsDataChecksum {
        FsDataChecksum{
            ..Default::default()
        }
    }
}

// pub const FS_FILE_FLAGS_HEADER_INCLUDED: u32 = 0x01;
// pub const FS_FILE_FLAGS_HEADER_PERSISTENT: u32 = 0x02;
// pub const FS_FILE_FLAGS_HEADER_HTTPVER_1_1: u32 = 0x04;
// pub const FS_FILE_FLAGS_SSI: u32 = 0x08;

#[derive(Clone,Debug,Default)]
pub struct FsFile {
    data: Vec<u8>,
    len: usize,
    index: usize,
    /* pextension is free for implementations to hold private (extensional)
    arbitrary data, e.g. holding some file state or file system handle */
    pextension: fs_file_extension,
    chksum: FsDataChecksum,
    chksum_count: u16,
    flags: FsFileFlags,
    is_custom_file: bool,
    state: Vec<u8>,
}

impl FsFile {
    pub fn new() -> FsFile {
        FsFile {
            ..Default::default()
        }
    }
}

type FsWaitCb = fn(arg: &mut Vec<u8>);


pub struct FsFileFlags {
    pub header_included: bool,
    pub header_persistent: bool,
    pub header_http_ver_1_1: bool,
    pub flags_ssi: bool
}

#[derive(Clone, Debug, Default)]
pub struct FsDataFile {
    pub name: String,
    pub data: Vec<u8>,
    pub len: usize,
    pub flags: FsFileFlags,
    pub checksum_count: usize,
    pub checksum: FsDataChecksum,
}

impl FsDataFile {
    pub fn new() -> FsDataFile {
        FsDataFile {
            ..Default::default()
        }
    }
}
