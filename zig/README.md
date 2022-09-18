Need more research:

# Compiler
 - How does the zig run/build find main() func
 - What code in ziglang keep const unchangable
 - Error if have 2 root source files "error: found another zig file 'main_test.zig' after root source file 'main.zig'"

# Values & Types
 - Does const in ziglang is calculated at compile time
 - How optinal type hold value
 - Type of null @Type(.Null)
 - How does undefined coerced to any type

# Test
 - Test function implicit return type `anyerror!void`
 - How a test runner can skip async method
 - How test runner detect memory leak

# Operators
 - orelse implementation
 - +%, +| implementation

# List
 - How array in memory, compiler-known size
 - How vector in memory
 - How slice in memory

# Pointer
 - How a single-pointer store/work
 - How a multiple pointer store/work
 - How a single-pointer that point to array work

# Struct
 - Padding strategy code
 - Packed struct
 - How tupes design

# Enum
 - Enum in function scoped can't define methods with self same type
