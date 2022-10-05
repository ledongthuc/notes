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
 - Pointer to array
 - Pointer to slice
 - Null-terminated pointer

# Struct
 - Padding strategy code
 - Packed struct
 - How tupes design

# Enum
 - Enum in function scoped can't define methods with self same type

# Union
 - How data is stored and handle in memory (bare & packed

# Stuffs
 - What is callconv and how callconv works
 - How @setCold do in compile step/runtime step

# Links:
 - https://rischmann.fr/blog/virtual-tables-with-zig-sqlite
 - https://zig.news/kprotty/building-a-tiny-mutex-537k
 - https://www.scattered-thoughts.net/writing/how-safe-is-zig/

# Videos
 - https://www.youtube.com/watch?v=Gv2I7qTux7g&feature=youtu.be&ab_channel=ChariotSolutions
 - https://vimeo.com/showcase/7818787/video/481466766
