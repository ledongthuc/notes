const std = @import("std");
const print = std.debug.print;

const N: i32 = 4;

pub fn main() !void {
    var a: [N][N]i32 = undefined;
    var b: [N][N]i32 = undefined;

    var row: usize = 5;
    var col: usize = 1;
    while (row >= 0) {
        b[row][col] = 1; // Error because index outbound
        row -= 1;
        col -= 1;
    }

    var i: usize = 0;
    while (i < N) : (i += 1) {
        var j: usize = 0;
        while (j < N) : (j += 1) {
            print("{} ", a[i][j]);
        }
        print("\n");
    }
}
