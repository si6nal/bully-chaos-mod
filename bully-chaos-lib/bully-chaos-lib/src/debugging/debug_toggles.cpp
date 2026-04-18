#include "debug_toggles.hpp"

#ifdef _DEBUG

#ifdef MODULE_WALKER
#include "modules/module_walker.hpp"
#endif

void handle_toggles( ) {
	#ifdef MODULE_WALKER
	module_walker::print_modules( );
	#endif
}

#endif