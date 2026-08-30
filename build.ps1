# 1. Compile the kernel
cargo build

# 2. Copy the binary
Copy-Item "target/x86_64-unknown-none/debug/glint_os" "disk_image/glint_os" -Force

wsl xorriso -as mkisofs -b limine-bios-cd.bin -no-emul-boot -boot-load-size 4 -boot-info-table -o glint_os.iso disk_image

