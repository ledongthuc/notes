const expectEqual = @import("std").testing.expectEqual;
const expect = @import("std").testing.expect;
const expectEqualStrings = @import("std").testing.expectEqualStrings;
const native_endian = @import("builtin").target.cpu.arch.endian();

const Point = struct {
    x: f32,
    y: f32,
};

const p = Point{
    .x = 0.12,
    .y = undefined,
};

test "basic struct" {
    try expectEqual(Point{ .x = 0.12, .y = undefined }, p);

    const anonymous_struct: Point = .{ .x = 1, .y = 2 };
    try expectEqual(2, anonymous_struct.y);
}

const Vec3 = struct {
    x: f32,
    y: f32,
    z: f32,

    pub fn init(x: f32, y: f32, z: f32) Vec3 {
        return Vec3{
            .x = x,
            .y = y,
            .z = z,
        };
    }

    pub fn dot(self1: Vec3, other: Vec3) f32 {
        return self1.x * other.x + self1.y * other.y + self1.z * other.z;
    }

    pub fn sum(self: Vec3) f32 {
        return self.x + self.y + self.z;
    }
};

test "dot product" {
    const v1 = Vec3.init(1.0, 0.0, 0.0);
    const v2 = Vec3.init(0.0, 1.0, 0.0);

    try expectEqual(@floatCast(f32, 0.0), v1.dot(v2));
    try expectEqual(@floatCast(f32, 0.0), Vec3.dot(v1, v2));
    try expectEqual(v1.sum(), Vec3.sum(v1));
}

const paddingStruct = struct {
    member1: i16,
    // padding here
    member2: i32,
};

const packedStruct = packed struct {
    member1: i16,
    member2: i32,
};

test "padding" {
    try expectEqual(64, @bitSizeOf(paddingStruct)); // Has padding
    try expectEqual(48, @bitSizeOf(packedStruct)); // No padding
}

const Empty = struct {
    pub const PI = 3.14;
};

test "struct namespaced variable" {
    try expectEqual(3.14, Empty.PI);
    try expectEqual(0, @sizeOf(Empty));

    const does_nothing = Empty{};
    _ = does_nothing;
}

test "get struct by field" {
    var s = Point{ .x = 1, .y = 2 };
    var pointer_of_s = @fieldParentPtr(Point, "x", &s.x);

    try expectEqual(&s, pointer_of_s); // Same pointer address value

    pointer_of_s.x = 3;
    try expectEqual(@floatCast(f32, 3), s.x);
}

fn LinkedList(comptime T: type) type {
    return struct {
        pub const Node = struct {
            prev: ?*Node,
            next: ?*Node,
            data: T,
        };

        first: ?*Node,
        last: ?*Node,
        len: usize,
    };
}

test "linked list" {
    try expect(LinkedList(i32) == LinkedList(i32)); // memory caching comp-time

    var list = LinkedList(i32){ // return type + init value
        .first = null,
        .last = null,
        .len = 0,
    };
    try expect(list.len == 0);

    const ListOfInts = LinkedList(i32);
    try expect(ListOfInts == LinkedList(i32));

    var node = ListOfInts.Node{
        .prev = null,
        .next = null,
        .data = 1234,
    };
    var list2 = LinkedList(i32){
        .first = &node,
        .last = &node,
        .len = 1,
    };

    try expectEqual(@intCast(i32, 1234), list2.first.?.data);
    try expectEqual(@intCast(i32, 1234), list2.first.?.*.data);
}

const Full = packed struct {
    number: u16,
};
const Divided = packed struct {
    half1: u8,
    quarter3: u4,
    quarter4: u4,
};

test "@bitCast between packed structs" {
    try doTheTest();
    comptime try doTheTest();
}

fn doTheTest() !void {
    try expect(@sizeOf(Full) == 2);
    try expect(@sizeOf(Divided) == 2);
    var full = Full{ .number = 0x1234 };
    var divided = @bitCast(Divided, full);
    switch (native_endian) {
        .Big => {
            try expect(divided.half1 == 0x12);
            try expect(divided.quarter3 == 0x3);
            try expect(divided.quarter4 == 0x4);
        },
        .Little => {
            try expect(divided.half1 == 0x34);
            try expect(divided.quarter3 == 0x2);
            try expect(divided.quarter4 == 0x1);
        },
    }
}

const BitField = packed struct {
    a: u3,
    b: u3,
    c: u2,
};

var bit_field = BitField{
    .a = 1,
    .b = 2,
    .c = 3,
};

test "pointer to non-bit-aligned field" {
    const pointerOfB = &bit_field.b;
    try expectEqual(@intCast(u3, 2), pointerOfB.*);
    // try expectEqual(@intCast(u3, 2),, bar(&bit_field.b)); // bar() expect function expects an ABI-aligned pointer but &bit_field.b is non-ABI-aligned field

    // Same address value
    try expectEqual(@ptrToInt(&bit_field.a), @ptrToInt(&bit_field.b));
    try expectEqual(@ptrToInt(&bit_field.a), @ptrToInt(&bit_field.c));

    try expectEqual(0, @bitOffsetOf(BitField, "a"));
    try expectEqual(3, @bitOffsetOf(BitField, "b"));
    try expectEqual(6, @bitOffsetOf(BitField, "c"));
}

fn bar(x: *const u3) u3 {
    return x.*;
}

test "struct name" {
    const Foo = struct {};
    try expectEqualStrings("BitField", @typeName(BitField));
    try expectEqualStrings("Foo", @typeName(Foo));
    try expectEqualStrings("struct:201:55", @typeName(struct {}));
    try expectEqualStrings("LinkedList(i32)", @typeName(LinkedList(i32)));
}

test "tuple" {
    const values = .{
        @as(u32, 1234),
        @as(f64, 12.34),
        true,
        "hi",
    } ++ .{false} ** 2;
    try expect(values[0] == 1234);
    try expect(values[4] == false);
    inline for (values) |v, i| {
        if (i != 2) continue;
        try expect(v);
    }
    try expect(values.len == 6);
    try expect(values.@"3"[0] == 'h');
}
