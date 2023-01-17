window.SIDEBAR_ITEMS = {"constant":[["VERSION","Version number of this crate."],["WASM_MAX_PAGES","The number of pages we can have before we run out of byte index space."],["WASM_MIN_PAGES","The minimum number of pages allowed."],["WASM_PAGE_SIZE","WebAssembly page sizes are fixed to be 64KiB. Note: large page support may be added in an opt-in manner in the future."]],"enum":[["Aarch64Architecture",""],["Architecture","The “architecture” field, which in some cases also specifies a specific subarchitecture."],["BinaryFormat","The “binary format” field, which is usually omitted, and the binary format is implied by the other fields."],["CallingConvention","The calling convention, which specifies things like which registers are used for passing arguments, which registers are callee-saved, and so on."],["Endianness","The target memory endianness."],["Environment","The “environment” field, which specifies an ABI environment on top of the operating system. In many configurations, this field is omitted, and the environment is implied by the operating system."],["ExportIndex","An entity to export."],["ExternType","A list of all possible types which can be externally referenced from a WebAssembly module."],["GlobalInit","Globals are initialized via the `const` operators or by referring to another import."],["ImportIndex","An entity to import."],["LibCall","The name of a runtime library routine."],["MemoryStyle","Implementation styles for WebAssembly linear memory."],["Mutability","Indicator of whether a global is mutable or not"],["OnCalledAction","After the stack is unwound via asyncify what should the call loop do next"],["OperatingSystem","The “operating system” field, which sometimes implies an environment, and sometimes isn’t an actual operating system."],["PointerWidth","The width of a pointer (in the default address space)."],["TableStyle","Implementation styles for WebAssembly tables."],["TrapCode","A trap code describing the reason for a trap."],["Type","A list of all possible value types in WebAssembly."],["Vendor","The “vendor” field, which in practice is little more than an arbitrary modifier."]],"fn":[["is_wasm","Check if the provided bytes are wasm-like"]],"macro":[["entity_impl","Macro which provides the common implementation of a 32-bit entity reference."]],"mod":[["compilation","Types for compilation."],["entity","The entity module, with common helpers for Rust structures"],["error","The WebAssembly possible errors"],["features",""],["indexes","Helper functions and structures for the translation."],["initializers",""],["lib","The `lib` module defines a `std` module that is identical whether the `core` or the `std` feature is enabled."],["libcalls",""],["memory",""],["module","Data structure for representing WebAssembly modules in a `wasmer::Module`."],["native",""],["serialize",""],["table",""],["trapcode","Trap codes describing the reason for a trap."],["types",""],["units",""],["utils",""],["value",""],["vmoffsets","Offsets and sizes of various structs in wasmer-vm’s vmcontext module."]],"struct":[["Bytes","Units of WebAssembly memory in terms of 8-bit bytes."],["CustomSectionIndex","Index type of a custom section inside a WebAssembly module."],["DataIndex","Index type of a passive data segment inside the WebAssembly module."],["DataInitializer","A data initializer for linear memory."],["DataInitializerLocation","A memory index and offset within that memory where a data initialization should be performed."],["ElemIndex","Index type of a passive element segment inside the WebAssembly module."],["ExportType","A descriptor for an exported WebAssembly value."],["ExportsIterator","This iterator allows us to iterate over the exports and offer nice API ergonomics over it."],["Features","Controls which experimental features will be enabled. Features usually have a corresponding WebAssembly proposal."],["FunctionIndex","Index type of a function (imported or local) inside the WebAssembly module."],["FunctionType","The signature of a function that is either implemented in a Wasm module or exposed to Wasm by the host."],["GlobalIndex","Index type of a global variable (imported or local) inside the WebAssembly module."],["GlobalType","WebAssembly global."],["ImportKey","Hash key of an import"],["ImportType","A descriptor for an imported value into a wasm module."],["ImportsIterator","This iterator allows us to iterate over the imports and offer nice API ergonomics over it."],["LocalFunctionIndex","Index type of a function defined locally inside the WebAssembly module."],["LocalGlobalIndex","Index type of a global defined locally inside the WebAssembly module."],["LocalMemoryIndex","Index type of a memory defined locally inside the WebAssembly module."],["LocalTableIndex","Index type of a table defined locally inside the WebAssembly module."],["Memory32","Marker trait for 32-bit memories."],["Memory64","Marker trait for 64-bit memories."],["MemoryIndex","Index type of a linear memory (imported or local) inside the WebAssembly module."],["MemoryType","A descriptor for a WebAssembly memory type."],["MetadataHeader","Metadata header which holds an ABI version and the length of the remaining metadata."],["ModuleInfo","A translated WebAssembly module, excluding the function bodies and memory initializers."],["OwnedDataInitializer","As `DataInitializer` but owning the data rather than holding a reference to it"],["PageCountOutOfRange","The only error that can happen when converting `Bytes` to `Pages`"],["Pages","Units of WebAssembly pages (as specified to be 65,536 bytes)."],["SerializableCompilation","The compilation related data for a serialized modules"],["SerializableModule","Serializable struct that is able to serialize from and to a `ArtifactInfo`."],["SignatureIndex","Index type of a signature (imported or local) inside the WebAssembly module."],["TableIndex","Index type of a table (imported or local) inside the WebAssembly module."],["TableInitializer","A WebAssembly table initializer."],["TableType","A descriptor for a table in a WebAssembly module."],["TargetSharedSignatureIndex","Target specific type for shared signature index."],["Triple","A target “triple”. Historically such things had three fields, though they’ve added additional fields over time."],["V128","The WebAssembly V128 type"],["VMBuiltinFunctionIndex","An index type for builtin functions."],["VMOffsets","This class computes offsets to fields within VMContext and other related structs that JIT code accesses directly."]],"trait":[["MemorySize","Trait for the `Memory32` and `Memory64` marker types."],["NativeWasmType","`NativeWasmType` represents a Wasm type that has a direct representation on the host (hence the “native” term)."],["ValueType","Trait for a Value type. A Value type is a type that is always valid and may be safely copied."]],"type":[["Addend","Addend to add to the symbol value."],["CodeOffset","Offset in bytes from the beginning of the function."]],"union":[["RawValue","Raw representation of a WebAssembly value."]]};