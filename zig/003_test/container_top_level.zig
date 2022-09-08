const std = @import("std");
const expect = std.testing.expect;

const imported_file = @import("main.zig");

test {
    std.testing.refAllDecls(@This());
    // Same with following line
    // _ = S;
    // _ = U;
    // _ = @import("main.zig");
}

const S = struct {
    test "S demo test" {
        try expect(true);
    }

    const SE = enum {
        V,

        // Won't run, if want it run try _ = S.SE
        test "This Test Won't Run" {
            try expect(false);
        }
    };
};

const U = union {
    s: US,

    const US = struct {
        test "U.US demo test" {
            try expect(true);
        }
    };

    test "U demo test" {
        try expect(true);
    }
};
