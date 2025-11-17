# Authentic System Logs

This directory contains real system logs that make the installation simulator authentic and entertaining.

## Provided Logs

These logs are **included with the application** - no setup needed!

- **`kernel.log`** - Real Linux kernel boot messages (657KB of authentic output)
- **`build.log`** - Real kernel compilation output (1234 lines)
- **`gcc.log`** - Additional compilation logs

The application automatically uses these logs to create a realistic installation experience.

## Just Run It!

No configuration needed. Just build and run:

```bash
cargo run --release
```

The simulator will use these authentic logs to create an endless, nostalgic installation loop with:
- Real kernel boot sequences
- Authentic compilation output
- Package installations
- System configuration
- And more!

## Want to Add Your Own Logs?

Feel free to replace these files with your own system logs to customize the experience. The application will automatically detect and use whatever logs are present in this directory.
