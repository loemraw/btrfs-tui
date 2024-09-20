use std::fmt::Debug;
use std::fmt::Display;

use crate::structs::CSUM_SIZE;
use crate::structs::FSID_SIZE;
use crate::structs::LABEL_SIZE;
use crate::structs::SYSTEM_CHUNK_ARRAY_SIZE;

use crate::structs::{DevItem, RootBackup};

use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Scrollbar;
use ratatui::widgets::ScrollbarOrientation;
use ratatui::widgets::ScrollbarState;
use ratatui::{buffer::Buffer, prelude::Rect, widgets::Widget};

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct Superblock {
    pub csum: [u8; CSUM_SIZE],
    pub fsid: [u8; FSID_SIZE],
    /// Physical address of this block
    pub bytenr: u64,
    pub flags: u64,
    pub magic: [u8; 0x8],
    pub generation: u64,
    /// Logical address of the root tree root
    pub root: u64,
    /// Logical address of the chunk tree root
    pub chunk_root: u64,
    /// Logical address of the log tree root
    pub log_root: u64,
    pub log_root_transid: u64,
    pub total_bytes: u64,
    pub bytes_used: u64,
    pub root_dir_objectid: u64,
    pub num_devices: u64,
    pub sector_size: u32,
    pub node_size: u32,
    /// Unused and must be equal to `nodesize`
    pub leafsize: u32,
    pub stripesize: u32,
    pub sys_chunk_array_size: u32,
    pub chunk_root_generation: u64,
    pub compat_flags: u64,
    pub compat_ro_flags: u64,
    pub incompat_flags: u64,
    pub csum_type: u16,
    pub root_level: u8,
    pub chunk_root_level: u8,
    pub log_root_level: u8,
    pub dev_item: DevItem,
    pub label: [u8; LABEL_SIZE],
    pub cache_generation: u64,
    pub uuid_tree_generation: u64,
    pub metadata_uuid: [u8; FSID_SIZE],
    /// Future expansion
    pub _reserved: [u64; 28],
    pub sys_chunk_array: [u8; SYSTEM_CHUNK_ARRAY_SIZE],
    pub root_backups: [RootBackup; 4],
}

impl Widget for Superblock {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
    }
}
