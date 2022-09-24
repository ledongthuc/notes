const expectEqual = @import("std").testing.expectEqual;
const std = @import("std");

fn foo(condition: bool, b: u32) void {
    const a = if (condition) b else return;
    _ = a;
    @panic("do something with a");
}

test "noreturn" {
    foo(false, 1);
    // foo(true, 1);
    var x = false;
    foo(x, 1);
}
