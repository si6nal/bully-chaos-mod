#include <Windows.h>
#include <iostream>

#ifdef _DEBUG
#include "debugging/debug_toggles.hpp"
#endif

void create_console( ) {
	// allocate console window, this will create a new console window
	AllocConsole( );

	// create file buffer
	FILE *f;

	// redirect all standard output to the console window, we ignore input
	freopen_s( &f, "CONOUT$", "w", stdout );
	freopen_s( &f, "CONOUT$", "w", stderr );

	// sync c++ streams with old C stdio
	std::ios::sync_with_stdio( true );

	// set console window title
	SetConsoleTitleA( "Bully Chaos Mod" );

	// test message
	std::cout << "successfully allocated console.\n";
}

int __stdcall DllMain( void *instance, unsigned long reason, void *reserved ) {
	// check if reason is process attach
	if ( reason == DLL_PROCESS_ATTACH ) {
		// create console window
		create_console( );

		// handle debug toggles
		#ifdef _DEBUG
		handle_toggles( );
		#endif

		// debugging
		MessageBoxA( nullptr, "Test", "Test", MB_OK );
	}

	// return success
	return EXIT_SUCCESS;
}