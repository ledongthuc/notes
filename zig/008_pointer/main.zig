const expectEqual = @import("std").testing.expectEqual;
const expect = @import("std").testing.expect;
const print = @import("std").debug.print;
const builtin = @import("builtin");

test "const single pointer" {
    const x: i32 = 1234;
    const addr_x = &x;

    try expectEqual(1234, addr_x.*);
    try expectEqual(*const i32, @TypeOf(addr_x));
}

test "variable single pointer" {
    var x: i32 = 1234;
    const addr_x = &x;

    try expectEqual(@intCast(i32, 1234), addr_x.*);
    try expectEqual(*i32, @TypeOf(addr_x));

    addr_x.* += 1;
    try expectEqual(@intCast(i32, 1235), addr_x.*);
}

test "many-item pointer" {
    const array = [_]i32{ 1, 3, 5, 7 };
    var ptr_array: [*]const i32 = &array;
    try expectEqual(@intCast(i32, 1), ptr_array[0]);

    ptr_array += 1;
    try expectEqual(@intCast(i32, 3), ptr_array[0]);
    // try expectEqual(@intCast(i32, 0), ptr_array[100]); // outbound access - unexpected behaviour runtime error

    ptr_array += 1;
    try expectEqual(@intCast(i32, 5), ptr_array[0]);

    ptr_array += 1;
    try expectEqual(@intCast(i32, 7), ptr_array[0]);

    ptr_array += 1;
    // try expectEqual(@intCast(i32, 0), ptr_array[0]); // outbound access - unexpected behaviour runtime error
}

test "slice" {
    const array = [_]i32{ 1, 3, 5, 7 };
    var length = @intCast(usize, 4);
    var slice = array[0..length];
    try expectEqual(@intCast(i32, 1), array[0]);

    try expectEqual(@intCast(i32, 3), slice[1]);
    // try expectEqual(@intCast(i32, 0), slice[100]); // outbound access
    try expectEqual(@intCast(i32, 5), slice[2]);
    try expectEqual(@intCast(i32, 7), slice[3]);
    // try expectEqual(@intCast(i32, 0), slice[4]); // outbound access
}

test "comptime pointers" {
    comptime {
        var x: i32 = 1;
        const ptr = &x;
        ptr.* += 1;
        x += 1;
        try expectEqual(@intCast(i32, 3), ptr.*);
    }
}

test "ptr convert" {
    const ptr = @intToPtr(*i32, 0xdeadbee0);
    const addr = @ptrToInt(ptr);
    try expectEqual(usize, @TypeOf(addr));
    try expectEqual(0xdeadbee0, addr);
    // try expectEqual(@intCast(i32, 0), ptr.*); // Segmentation fault
}

test "pointer alignment" {
    var x: i32 = 1234;
    const align_of_i32 = @alignOf(@TypeOf(x));
    try expect(@TypeOf(&x) == *i32);
    try expect(*i32 == *align(align_of_i32) i32);
    if (builtin.target.cpu.arch == .x86_64) {
        try expect(@typeInfo(*i32).Pointer.alignment == 4);
    }
}

test "allowzero pointer" {
    var zero: usize = 0;
    var ptr = @intToPtr(*allowzero i32, zero); // accept 0x00 pointer
    try expectEqual(@intCast(usize, 0), @ptrToInt(ptr));
}

test "optinal pointer" {
    var ptr: ?*i32 = null;
    try expect(ptr == null);
}
