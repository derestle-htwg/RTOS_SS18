//Mod VMM

use volatile::Volatile;
pub mod LLFrameAllocator;
pub use self::LLFrameAllocator::*;
pub mod allocator;

#[derive(Clone, Copy)]
#[repr(C)]
struct PagetableEntry
{
    data: u64 
}




impl PagetableEntry
{
    fn getPA(&self) -> u64 {
        self.data & 0x000FFFFFFFFFF000
    }
    fn setPA(&mut self, pa: u64) {
        self.data = (self.data & 0xFFF0000000000FFF) | pa;
    }
    fn getBit(&self, Bit: u8) -> bool{
        (self.data & (1 << Bit)) != 0
    }

    fn setBit(&mut self, Bit: u8, value: bool){
        let mut mask: u64;
        mask = 1 << Bit;
        if(value)
        {
            self.data = self.data | mask;
		}
        else
        {
            mask = ! mask;
            self.data = self.data & mask;
        }
    }

    fn getPresent(&self) -> bool {
        self.getBit(0)
    }
    fn setPresent(&mut self, present: bool){
        self.setBit(0, present)
    }

    fn getWriteable(&self) -> bool {
        self.getBit(1)
    }
    fn setWriteable(&mut self, writeable: bool){
        self.setBit(1, writeable)
    }
    fn getUser(&self) -> bool {
        self.getBit(2)
    }
    fn setUser(&mut self, user: bool){
        self.setBit(2, user)
    }
    fn getWriteThrough(&self) -> bool {
        self.getBit(3)
    }
    fn setWriteThrough(&mut self, wt: bool){
        self.setBit(3, wt)
    }
    fn getCacheDisable(&self) -> bool {
        self.getBit(4)
    }
    fn setCacheDisable(&mut self, cd: bool){
        self.setBit(4, cd)
    }
    fn getAccessed(&self) -> bool {
        self.getBit(5)
    }
    fn setAccessed(&mut self, a: bool){
        self.setBit(5, a)
    }

    fn getDirty(&self) -> bool {
        self.getBit(6)
    }
    fn setDirty(&mut self, dirty: bool){
        self.setBit(6, dirty)
    }

    fn getPAT(&self) -> bool {
        self.getBit(7)
    }
    fn setPAT(&mut self, PAT3: bool){
        self.setBit(7, PAT3)
    }

    fn getGlobal(&self) -> bool {
        self.getBit(8)
    }
    fn setGlobal(&mut self, global: bool){
        self.setBit(8, global)
    }

    fn getAvailable(&self) -> u64{
        (self.data & (0x7 << 9)) >> 9
    }

    fn setAvailable(&mut self, value: u64){
        let mut mask: u64;
        mask = 7 << 9;
        mask = !mask;
        self.data = (self.data & mask) | ((value & 7) << 9);

    }
    
}
#[derive(Clone, Copy)]
#[repr(C)]
struct PML4E
{
    entry: PagetableEntry
}

#[derive(Clone, Copy)]
#[repr(C)]
struct PML4 
{
    entries: [PML4E; 512]
}

#[derive(Clone, Copy)]
#[repr(C)]
struct PML3E
{
    entry: PagetableEntry
}

#[derive(Clone, Copy)]
#[repr(C)]
struct PML3
{
    entries: [PML3E; 512]
}

#[derive(Clone, Copy)]
#[repr(C)]
struct PML2E
{
    entry: PagetableEntry
}

#[derive(Clone, Copy)]
#[repr(C)]
struct PML2
{
    entries: [PML2E; 512]
}

#[derive(Clone, Copy)]
#[repr(C)]
struct PML1E
{
    entry: PagetableEntry
}

#[derive(Clone, Copy)]
#[repr(C)]
struct PML1
{
    entries: [PML1E; 512]
}

impl Default for PML4E
{
	fn default() -> PML4E { PML4E{ entry: PagetableEntry{ data: 0} } }
}

impl Default for PML3E
{
	fn default() -> PML3E { PML3E{ entry: PagetableEntry{ data: 0} } }
}

impl Default for PML2E
{
	fn default() -> PML2E { PML2E{ entry: PagetableEntry{ data: 0} } }
}

impl Default for PML1E
{
	fn default() -> PML1E { PML1E{ entry: PagetableEntry{ data: 0} } }
}

impl Default for PML4
{
	fn default() -> PML4 { PML4 {entries: [PML4E { ..Default::default() }; 512] } }
}

impl Default for PML3
{
	fn default() -> PML3 { PML3 {entries: [PML3E { ..Default::default() }; 512] } }
}

impl Default for PML2
{
	fn default() -> PML2 { PML2 {entries: [PML2E { ..Default::default() }; 512] } }
}

impl Default for PML1
{
	fn default() -> PML1 { PML1 {entries: [PML1E { ..Default::default() }; 512] } }
}

