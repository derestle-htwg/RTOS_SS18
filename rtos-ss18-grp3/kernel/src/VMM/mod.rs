//Mod VMM

pub mod LLFrameAllocator;
pub use self::LLFrameAllocator::*;
pub mod allocator;
pub use self::allocator::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PagetableEntry
{
    pub data: u64 
}

impl PagetableEntry
{
    pub fn getPA(&self) -> u64 {
        self.data & 0x000FFFFFFFFFF000
    }
    pub fn setPA(&mut self, pa: u64) {
        self.data = (self.data & 0xFFF0000000000FFF) | pa;
    }
    pub fn getBit(&self, Bit: u8) -> bool{
        (self.data & (1 << Bit)) != 0
    }

    pub fn setBit(&mut self, Bit: u8, value: bool){
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

    pub fn getPresent(&self) -> bool {
        self.getBit(0)
    }
    pub fn setPresent(&mut self, present: bool){
        self.setBit(0, present)
    }

    pub fn getWriteable(&self) -> bool {
        self.getBit(1)
    }
    pub fn setWriteable(&mut self, writeable: bool){
        self.setBit(1, writeable)
    }
    pub fn getUser(&self) -> bool {
        self.getBit(2)
    }
    pub fn setUser(&mut self, user: bool){
        self.setBit(2, user)
    }
    pub fn getWriteThrough(&self) -> bool {
        self.getBit(3)
    }
    pub fn setWriteThrough(&mut self, wt: bool){
        self.setBit(3, wt)
    }
    pub fn getCacheDisable(&self) -> bool {
        self.getBit(4)
    }
    pub fn setCacheDisable(&mut self, cd: bool){
        self.setBit(4, cd)
    }
    pub fn getAccessed(&self) -> bool {
        self.getBit(5)
    }
    pub fn setAccessed(&mut self, a: bool){
        self.setBit(5, a)
    }

    pub fn getDirty(&self) -> bool {
        self.getBit(6)
    }
    pub fn setDirty(&mut self, dirty: bool){
        self.setBit(6, dirty)
    }

    pub fn getPAT(&self) -> bool {
        self.getBit(7)
    }
    pub fn setPAT(&mut self, PAT3: bool){
        self.setBit(7, PAT3)
    }

    pub fn getGlobal(&self) -> bool {
        self.getBit(8)
    }
    pub fn setGlobal(&mut self, global: bool){
        self.setBit(8, global)
    }

    pub fn getAvailable(&self) -> u64{
        (self.data & (0x7 << 9)) >> 9
    }

    pub fn setAvailable(&mut self, value: u64){
        let mut mask: u64;
        mask = 7 << 9;
        mask = !mask;
        self.data = (self.data & mask) | ((value & 7) << 9);

    }
    
}
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PML4E
{
    pub entry: PagetableEntry
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PML4 
{
    pub entries: [PML4E; 512]
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PML3E
{
    entry: PagetableEntry
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PML3
{
    entries: [PML3E; 512]
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PML2E
{
    entry: PagetableEntry
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PML2
{
    entries: [PML2E; 512]
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PML1E
{
    entry: PagetableEntry
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PML1
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
impl PML4
{	
	fn getPML3(&self, index: usize) -> Option<&mut PML3> 
	{
		if self.entries[index].entry.getPresent()
		{
			let pa = self.entries[index].entry.getPA(); 
			Some( unsafe { &mut *(pa as *mut PML3) } )
		}
		else
		{
			None
		}
	}
	
	pub fn zero(&mut self)
	{
		for i in 0..512
		{
			self.entries[i].entry.data = 0;
		}
	}
}

impl Default for PML3
{
	fn default() -> PML3 { PML3 {entries: [PML3E { ..Default::default() }; 512] } }
}

impl PML3
{	
	fn getPML2(&self, index: usize) -> Option<&mut PML2> 
	{
		if self.entries[index].entry.getPresent()
		{
			let pa = self.entries[index].entry.getPA(); 
			Some( unsafe { &mut *(pa as *mut PML2) } )
		}
		else
		{
			None
		}
	}
	
	pub fn zero(&mut self)
	{
		for i in 0..512
		{
			self.entries[i].entry.data = 0;
		}
	}
}

impl Default for PML2
{
	fn default() -> PML2 { PML2 {entries: [PML2E { ..Default::default() }; 512] } }
}

impl PML2{
	pub fn getPML1(&self, index: usize) -> Option<&mut PML1> 
	{
		if self.entries[index].entry.getPresent()
		{
			let pa = self.entries[index].entry.getPA(); 
			Some( unsafe { &mut *(pa as *mut PML1) } )
		}
		else
		{
			None
		}
	}
	
	pub fn zero(&mut self)
	{
		for i in 0..512
		{
			self.entries[i].entry.data = 0;
		}
	}
}

impl Default for PML1
{
	fn default() -> PML1 { PML1 {entries: [PML1E { ..Default::default() }; 512] } }
}

impl PML1
{
	pub fn getPML1E(&self, index: usize) -> Option<&mut PML1E> 
	{
		if self.entries[index].entry.getPresent()
		{
			let pa = self.entries[index].entry.getPA(); 
			Some( unsafe { &mut *(pa as *mut PML1E) } )
		}
		else
		{
			None
		}
	}
	
	pub fn zero(&mut self)
	{
		for i in 0..512
		{
			self.entries[i].entry.data = 0;
		}
	}
}

