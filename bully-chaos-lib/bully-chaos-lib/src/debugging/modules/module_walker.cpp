#include "module_walker.hpp"

#ifdef MODULE_WALKER

#include <Windows.h>
#include <Psapi.h>
#include <iostream>
#include <fstream>

void module_walker::print_modules( ) {
	// get handle of current process
	HANDLE current_process = GetCurrentProcess( );

	// allocate memory for modules
	HMODULE modules[ 1024 ];
	DWORD needed_bytes = 0ul;

	// clear exports file
	std::ofstream( "exports.txt", std::ios::trunc ).close( );

	// open stream to module exports file
	std::ofstream module_exports_file( "exports.txt", std::ios::app );

	std::cout << "dumping module info...\n";

	// get modules
	if ( K32EnumProcessModules( current_process, modules, sizeof( modules ), &needed_bytes ) ) {
		// get how many modules are currently loaded
		unsigned int module_count = needed_bytes / sizeof( HMODULE );
		std::cout << "currently loaded modules: " << module_count << std::endl;

		// enumerate modules
		for ( unsigned int i = 0; i < module_count; i++ ) {
			// get current module
			HMODULE current_module = modules[ i ];

			// create char array for module name
			char module_name[ MAX_PATH ];

			// get module name
			if ( GetModuleBaseNameA( current_process, current_module, module_name, MAX_PATH ) ) {
				// get base address of module
				unsigned char *module_base_addr = reinterpret_cast< unsigned char * >( current_module );

				// print module name & base address
				std::cout << module_name << " -> 0x" << std::hex << ( void * )module_base_addr << std::endl;

				// open stream to module exports file
				//std::fstream module_exports_file( "exports.txt", std::ios::app );

				// verify we have an open stream to the file
				if ( !module_exports_file ) {
					std::cout << "\tfailed to open exports file.\n";
					continue;
				}

				// write module info to file
				module_exports_file << module_name << " -> 0x" << std::hex << module_base_addr << std::endl;

				// get dos header for module
				IMAGE_DOS_HEADER *module_dos_header = reinterpret_cast< IMAGE_DOS_HEADER * >( module_base_addr );

				// validate dos header
				if ( module_dos_header->e_magic != IMAGE_DOS_SIGNATURE ) {
					std::cout << "\tfailed to get module dos header.\n";
					continue;
				}

				// get nt headers
				IMAGE_NT_HEADERS32 *module_nt_headers = reinterpret_cast< IMAGE_NT_HEADERS32 * >( module_base_addr + module_dos_header->e_lfanew );

				// validate nt headers
				if ( module_nt_headers->Signature != IMAGE_NT_SIGNATURE ) {
					std::cout << "\tfailed to get module nt headers.\n";
					continue;
				}

				// get data directory
				unsigned long &module_data_dir_addr = module_nt_headers->OptionalHeader.DataDirectory[ IMAGE_DIRECTORY_ENTRY_EXPORT ].VirtualAddress;

				// check if the module has exports
				if ( module_data_dir_addr == 0 ) {
					std::cout << "\tmodule has no exports.\n";
					continue;
				}

				// get export directory
				IMAGE_EXPORT_DIRECTORY *module_export_dir = reinterpret_cast< IMAGE_EXPORT_DIRECTORY * >( module_base_addr + module_data_dir_addr );

				// allocate memory for reading export data
				DWORD *module_export_names = reinterpret_cast< DWORD * >( module_base_addr + module_export_dir->AddressOfNames );
				WORD *module_export_ordinals = reinterpret_cast< WORD * >( module_base_addr + module_export_dir->AddressOfNameOrdinals );
				DWORD *module_export_offsets = reinterpret_cast< DWORD * >( module_base_addr + module_export_dir->AddressOfFunctions );

				// enumerate through export names
				for ( unsigned long j = 0; j < module_export_dir->NumberOfNames; j++ ) {
					// get export name & function offset
					const char *export_name = reinterpret_cast< const char * >( module_base_addr + module_export_names[ j ] );
					WORD export_ordinal = module_export_ordinals[ j ];
					DWORD export_offset = module_export_offsets[ export_ordinal ];

					// get function address
					void *export_addr = module_base_addr + export_offset;

					// print export info
					//std::cout << "\t[" << j << "] " << export_name << " -> 0x" << std::hex << export_addr << std::dec << " (offset: 0x" << std::hex << ( void * )export_offset << ")\n";
					module_exports_file << "\t[" << j << "] " << export_name << " -> 0x" << std::hex << export_addr << std::dec << " (offset: 0x" << std::hex << ( void * )export_offset << ")\n";
				}
			} else {
				std::cout << "failed to get module name at index " << i << std::endl;
				if ( module_exports_file ) {
					module_exports_file << "failed to get module name at index " << i << std::endl;
				}
			}
		}

		if ( module_exports_file ) {
			std::cout << "successfully dumped modules.\n";
		}
	}
}

#endif