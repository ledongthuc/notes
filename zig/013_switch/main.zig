const expectEqual = @import("std").testing.expectEqual;
const builtin = @import("builtin");

test "switch simple" {
    const a: u64 = 10;
    const zz: u64 = 103;

    const b = switch (a) {
        1, 2, 3 => 0,
        5...100 => 1,
        101 => blk: {
            const c: u64 = 5;
            break :blk c * 2 + 1;
        },

        zz => zz,
        blk: {
            const d: u32 = 5;
            const e: u32 = 100;
            break :blk d + e;
        } => 107,

        else => 9,
    };
    try expectEqual(1, b);
}

test "switch expressions" {
    const a: u64 = 10;
    const b = switch (a) {
        blk: {
            const d: u32 = 6;
            const e: u32 = 4;
            break :blk d + e;
        } => 5,
        else => 4,
    };
    try expectEqual(5, b);
}

const os_msg = switch (builtin.target.os.tag) {
    .linux => "we found a linux user",
    else => "not a linux user",
};
