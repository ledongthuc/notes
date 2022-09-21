const expectEqual = @import("std").testing.expectEqual;
const expect = @import("std").testing.expect;
const std = @import("std");

test "basic union" {
    const Payload = union {
        int: i32,
        float: f64,
        boolean: bool,
    };

    var payload: Payload = .{ .int = 123 };
    try expectEqual(@intCast(i32, 123), payload.int);

    payload = .{ .float = 12.34 };
    try expectEqual(@floatCast(f64, 12.34), payload.float);
}

test "tagged union" {
    const ComplexTypeTag = enum {
        ok,
        not_ok,
    };
    const ComplexType = union(ComplexTypeTag) {
        ok: u8,
        not_ok: void,
    };

    const c = ComplexType{ .ok = 42 };
    try expectEqual(ComplexTypeTag.ok, @as(ComplexTypeTag, c));
    try expect(std.mem.eql(u8, @tagName(ComplexTypeTag.ok), "ok"));

    switch (c) {
        ComplexTypeTag.ok => |value| try expectEqual(42, value),
        ComplexTypeTag.not_ok => unreachable,
    }

    try expectEqual(ComplexTypeTag, std.meta.Tag(ComplexType));

    const c1 = ComplexType{ .ok = 42 };
    const c2 = ComplexType.not_ok;

    try expect(c1 == .ok);
    try expect(c2 == .not_ok);
}

test "modify tagged union in switch" {
    const ComplexTypeTag = enum {
        ok,
        not_ok,
    };
    const ComplexType = union(ComplexTypeTag) {
        ok: u8,
        not_ok: void,
    };

    var c = ComplexType{ .ok = 42 };
    try expect(@as(ComplexTypeTag, c) == ComplexTypeTag.ok);

    switch (c) {
        ComplexTypeTag.ok => |*value| value.* += 1,
        ComplexTypeTag.not_ok => unreachable,
    }

    try expect(c.ok == 43);
}

const Variant = union(enum) {
    int: i32,
    boolean: bool,

    // void can be omitted when inferring enum tag type.
    none,

    fn truthy(self: Variant) bool {
        return switch (self) {
            Variant.int => |x_int| x_int != 0,
            Variant.boolean => |x_bool| x_bool,
            Variant.none => false,
        };
    }
};

test "union method" {
    var v1 = Variant{ .int = 1 };
    var v2 = Variant{ .boolean = false };

    try expect(v1.truthy());
    try expect(!v2.truthy());
}
