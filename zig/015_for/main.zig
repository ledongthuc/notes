const expectEqual = @import("std").testing.expectEqual;
const expect = @import("std").testing.expect;

test "simple for array" {
    const items = [_]i32{ 4, 5, 3, 4, 0 };
    var total: i32 = 0;

    for (items) |item| {
        total += item;
    }
    try expectEqual(@intCast(i32, 16), total);
}

test "simple for slice" {
    const items = [_]i32{ 4, 5, 3, 4, 0 };
    var total: i32 = 0;
    var lastIndex: usize = 0;

    for (items[1..4]) |item, index| {
        total += item;
        lastIndex = index;
    }
    try expectEqual(@intCast(i32, 12), total);
    try expectEqual(@intCast(usize, 2), lastIndex);
}

test "for reference" {
    var items = [_]i32{ 3, 4, 2 };

    for (items) |*value| {
        value.* += 1;
    }

    try expect(items[0] == 4);
    try expect(items[1] == 5);
    try expect(items[2] == 3);
}

test "else for" {
    var items = [_]?i32{ 3, 4, null, 5, null };
    var total: i32 = 0;

    var val: i32 = for (items) |item| {
        if (item != null) {
            total += item orelse 0;
        } else {
            break -1;
        }
    } else blk: {
        break :blk 999;
    };

    try expectEqual(@intCast(i32, 7), total);
    try expectEqual(@intCast(i32, -1), val);
}
