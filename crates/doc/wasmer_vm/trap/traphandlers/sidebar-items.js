initSidebarItems({"constant":[["TRAP_HANDLER",""],["YIELDER",""]],"enum":[["UnwindReason",""]],"fn":[["catch_traps","Catches any wasm traps that happen within the execution of `closure`, returning them as a `Result`."],["get_pc_sp",""],["init_traps","This function is required to be called before any WebAssembly is entered. This will configure global state such as signal handlers to prepare the process to receive wasm traps."],["lazy_per_thread_init","A module for registering a custom alternate signal stack (sigaltstack)."],["on_host_stack","When executing on the Wasm stack, temporarily switch back to the host stack to perform an operation that should not be constrainted by the Wasm stack limits."],["on_wasm_stack","Runs the given function on a separate stack so that its stack usage can be bounded. Stack overflows and other traps can be caught and execution returned to the root of the stack."],["platform_init",""],["raise_lib_trap","Raises a trap from inside library code immediately."],["raise_user_trap","Raises a user-defined trap immediately."],["resume_panic","Carries a Rust panic across wasm code and resumes the panic on the other side."],["trap_handler",""],["unwind_with",""],["update_context",""],["wasmer_call_trampoline","Call the wasm function pointed to by `callee`."]],"static":[["PREV_SIGBUS",""],["PREV_SIGFPE",""],["PREV_SIGILL",""],["PREV_SIGSEGV",""]],"struct":[["TrapHandlerContext","Read-only information that is used by signal handlers to handle and recover from traps."],["TrapHandlerContextInner",""]],"trait":[["TrapHandler","A package of functionality needed by `catch_traps` to figure out what to do when handling a trap."]],"type":[["TrapHandlerFn","Function which may handle custom signals while processing traps."]]});