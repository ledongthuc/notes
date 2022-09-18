const expectEqual = @import("std").testing.expectEqual;
const expect = @import("std").testing.expect;

test "basic struct" {
    const Type = enum {
        ok,
        not_ok,
    };

    const c = Type.ok;
    try expectEqual(Type.ok, c);
}

test "enum ordinal value" {
    const Type = enum(u2) {
        zero,
        one,
        two,
    };

    try expectEqual(0, @enumToInt(Type.zero));
    try expectEqual(1, @enumToInt(Type.one));
    try expectEqual(2, @enumToInt(Type.two));

    const Type2 = enum(u32) {
        hundred = 100,
        thousand = 1000,
        million = 1000000,
    };
    try expectEqual(100, @enumToInt(Type2.hundred));
    try expectEqual(1000, @enumToInt(Type2.thousand));
    try expectEqual(1000000, @enumToInt(Type2.million));
}

const EnumType = enum(u2) {
    zero,
    one,
    two,

    pub fn isZero(self: EnumType) bool {
        return self == EnumType.zero;
    }
};

test "enum methods" {
    const t = EnumType.zero;
    try expect(t.isZero());
}

// Not compatible with C-ABI
// const Foo = enum { a, b, c };
// export fn entry(foo: Foo) void {
//     _ = foo;
// }

// Compatible with C-ABI
const Foo2 = enum(c_int) { a, b, c };
export fn entry2(foo: Foo2) void {
    _ = foo;
}

test "enum literals" {
    const Color = enum {
        auto,
        off,
        on,
    };
    const color1: Color = .auto;
    const color2: Color = Color.auto;
    try expectEqual(color1, color2);
}

test "switch & enum" {
    const Color = enum {
        auto,
        off,
        on,
    };
    const color = Color.on;
    const result = switch (color) {
        .auto => false,
        .on => true,
        .off => false,
    };
    try expect(result);
}

test "switch on non-exhaustive enum" {
    const Number = enum(u8) {
        one,
        two,
        three,
        _,
    };

    const number = Number.one;
    const result = switch (number) {
        .one => true,
        .two, .three => false,
        _ => false,
    };
    try expect(result);

    const is_one = switch (number) {
        .one => true,
        else => false,
    };
    try expect(is_one);
}
