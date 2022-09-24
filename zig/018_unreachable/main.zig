const expectEqual = @import("std").testing.expectEqual;

test "unreachable" {
    const x = 1;
    const y = 2;
    if (x + y != 3) {
        unreachable;
    }
}
