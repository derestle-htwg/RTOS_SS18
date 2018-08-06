
use self::LLFrameAllocator::*;
use VMM::*;

pub fn mapAddresspace(pmem: &mut frame, vmem: u64, pgtbl: &mut PML4)
{
	let level1Index : usize = ((vmem >> 12) & 0x01FF) as usize;
	let level2Index : usize = ((vmem >> 21) & 0x01FF) as usize;
	let level3Index : usize = ((vmem >> 30) & 0x01FF) as usize;
	let level4Index : usize = ((vmem >> 39) & 0x01FF) as usize; 
	
	match pgtbl.getPML3(level4Index)
	{
		Some(_) => {},
		None => {
			let newPG: u64 = unsafe { getFrame().expect("Out of memory") as *mut _ as u64 };
			pgtbl.entries[level4Index].entry.data = 0;
			pgtbl.entries[level4Index].entry.setPA(newPG);
			pgtbl.entries[level4Index].entry.setPresent(true);
			pgtbl.entries[level4Index].entry.setWriteable(true);
			pgtbl.getPML3(level4Index).expect("PML3 load failed").zero();
		}
	}
	let pgLvl3 : &mut PML3 = pgtbl.getPML3(level4Index).expect("PML3 load failed");
	
	match pgLvl3.getPML2(level3Index)
	{
		Some(_) => {},
		None => {
			let newPG: u64 = unsafe { getFrame().expect("Out of memory") as *mut _ as u64 };
			pgLvl3.entries[level3Index].entry.data = 0;
			pgLvl3.entries[level3Index].entry.setPA(newPG);
			pgLvl3.entries[level3Index].entry.setPresent(true);
			pgLvl3.entries[level3Index].entry.setWriteable(true);
			pgLvl3.getPML2(level3Index).expect("PML2 load failed").zero();
		}
	}
	let pgLvl2 : &mut PML2 = pgLvl3.getPML2(level3Index).expect("PML2 load failed");
	
	match pgLvl2.getPML1(level2Index)
	{ 
		Some(_) => {},
		None => {
			let newPG: u64 = unsafe { getFrame().expect("Out of memory") as *mut _ as u64 };
			pgLvl2.entries[level2Index].entry.data = 0;
			pgLvl2.entries[level2Index].entry.setPA(newPG);
			pgLvl2.entries[level2Index].entry.setPresent(true);
			pgLvl2.entries[level2Index].entry.setWriteable(true);
			pgLvl2.getPML1(level2Index).expect("PML1 load failed").zero();
		}
	}
	let pgLvl1 : &mut PML1 = pgLvl2.getPML1(level2Index).expect("PML1 load failed");
	
	match pgLvl1.getPML1E(level1Index)
	{ 
		Some(oldPage) => 
		{
			let oldFrame = frame::new(oldPage.entry.getPA());
			appendFrame(oldFrame);
		},
		None => {}
	}
	let newPG: u64 = unsafe { pmem as *mut frame as u64 };
	pgLvl1.entries[level1Index].entry.data = 0;
	pgLvl1.entries[level1Index].entry.setPA(newPG);
	pgLvl1.entries[level1Index].entry.setPresent(true);
	pgLvl1.entries[level1Index].entry.setWriteable(true);
	
	
	
}
