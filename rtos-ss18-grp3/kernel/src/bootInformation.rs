#[macro_use]

use VMM;
use multiboot2;

pub fn parse(bootblock: &u64)
{
	println!("Parsing Bootinformation @{:p}", bootblock);
	let boot_info = unsafe{ multiboot2::load(bootblock as *const u64 as usize) };
	
	let elf_sections_tag = boot_info.elf_sections_tag()
    .expect("Elf-sections tag required");

	let mut minKernelAddr : u64 = 0xFFFFFFFFFFFFFFFF;
	let mut maxKernelAddr : u64 = 0 ;
	
	println!("kernel sections:");
	for section in elf_sections_tag.sections() {
		println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
			section.addr, section.size, section.flags);
		if(minKernelAddr > section.addr) 
		{
			minKernelAddr = section.addr;
		}
		if(maxKernelAddr < (section.addr + section.size))
		{
			maxKernelAddr = section.addr + section.size;
		}
	}
	println!("kernel Range: {:x} - {:x}", minKernelAddr, maxKernelAddr);
	
	let memory_map_tag = boot_info.memory_map_tag()
    .expect("Memory map tag required");

	println!("memory areas:");
	for area in memory_map_tag.memory_areas() {
		println!("    start: 0x{:x}, length: 0x{:x}",
			area.base_addr, area.length);
		let mut rangeStart = area.base_addr;
		let mut rangeEnd = area.length;
//		 KKKKKKKKKK
//1   MMM
//2   MMMMMM
//3   MMMMMMMMMMMMMMMM
//4        MMMMMM
//5              MMMMM
//6                 MM
		
		if((rangeEnd <= minKernelAddr) | (rangeStart >= maxKernelAddr))
		{//1+6
			//1:1 übernehmen
		}
		else if (rangeStart <= minKernelAddr && rangeEnd >= minKernelAddr && rangeEnd <= maxKernelAddr)
		{//2
			rangeEnd = minKernelAddr - 1;
		}
		else if (rangeStart <= minKernelAddr && rangeEnd >= maxKernelAddr)
		{//3
			println!("Speicherbereich2 wird hinzugefügt: {:x}-{:x}", maxKernelAddr+1, rangeEnd);
			VMM::InsertMemoryRange(maxKernelAddr+1, rangeEnd);
			rangeEnd = minKernelAddr - 1;
		}
		else if (rangeStart >= minKernelAddr && rangeEnd <= maxKernelAddr)
		{//4
			rangeStart = rangeEnd;
		}
		else
		{//5
			rangeStart = maxKernelAddr +1 ;
		}
		
		if( rangeStart <= 0x2FFF)
		{
			rangeStart = 0x3000;
		}
		
		if rangeStart >= rangeEnd
		{
			println!("Speicherbereich ist zu klein: {:x}-{:x}", rangeStart, rangeEnd);
		}
		else
		{
			println!("Speicherbereich wird hinzugefügt: {:x}-{:x}", rangeStart, rangeEnd);
			VMM::InsertMemoryRange(rangeStart, rangeEnd)
		
		}
		
			
			
	}
	
	
}

