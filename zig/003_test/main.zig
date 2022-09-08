const testing = @import("std").testing;
const builtin = @import("builtin");
const print = @import("std").debug.print;

pub fn main() !void {
    print("{}", .{isATest()});
}

fn addOne(number: i32) i32 {
    return number + 1;
}

fn isATest() bool {
    return builtin.is_test;
}

test "expect addOne adds one to 41" {
    try testing.expect(addOne(41) == 42);
}

test "this will be skipped" {
    return error.SkipZigTest;
}

test "isATest" {
    try testing.expect(isATest());
}
