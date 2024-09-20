const CSUM_SIZE: usize = 32;
const LABEL_SIZE: usize = 256;
const FSID_SIZE: usize = 16;
const UUID_SIZE: usize = 16;
const SYSTEM_CHUNK_ARRAY_SIZE: usize = 2048;

pub const FS_TREE_OBJECTID: u64 = 5;

pub const INODE_REF_KEY: u8 = 12;
pub const DIR_ITEM_KEY: u8 = 84;
pub const ROOT_ITEM_KEY: u8 = 132;
pub const CHUNK_ITEM_KEY: u8 = 228;

pub const FT_REG_FILE: u8 = 1;

mod chunk;
mod dev_item;
mod dir_item;
mod header;
mod inode_item;
mod inode_ref;
mod item;
mod key;
mod key_ptr;
mod leaf;
mod node;
mod root_backup;
mod root_item;
mod stripe;
mod superblock;
mod timespec;

pub use chunk::Chunk;
pub use dev_item::DevItem;
pub use dir_item::DirItem;
pub use header::Header;
pub use inode_item::InodeItem;
pub use inode_ref::InodeRef;
pub use item::Item;
pub use key::Key;
pub use key_ptr::KeyPtr;
pub use leaf::Leaf;
pub use node::Node;
pub use root_backup::RootBackup;
pub use root_item::RootItem;
pub use stripe::Stripe;
pub use superblock::Superblock;
pub use timespec::Timespec;
