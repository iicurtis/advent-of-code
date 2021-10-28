const std = @import("std");

pub fn main() anyerror!void {
    var timer = try std.time.Timer.start();
    const t0 = timer.lap();
    const os = std.io.getStdOut().writer();

    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();

    const input_file = try std.fs.cwd().openFile(
        "../inputs/day01.txt",
        .{ .read = true },
    );
    defer input_file.close();

    const input = try input_file.reader().readAllAlloc(&arena.allocator, 1024 * 8);
    var count: isize = 0;
    var part2: usize = 0;
    for (input) |c, i| {
        if (c == '(') {
            count += 1;
        } else {
            count -= 1;
        }
        if ((count == -1) and (part2 == 0)) {
            part2 = i;
        }
    }
    const t1 = timer.lap();
    try os.print("Part 1: {}\n", .{count});
    try os.print("Part 2: {}\n", .{part2});
    try os.print("Time:   {}μs\n", .{(t1 - t0) / std.time.ns_per_us});
}
