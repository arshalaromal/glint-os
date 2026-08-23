# 1. Compile the kernel for bare-metal x86_64
cargo build

# 2. Automatically copy the compiled binary into the disk_image root
Copy-Item "target/x86_64-unknown-none/debug/glint_os" "disk_image/glint_os" -Force

Write-Host "Glint OS built and copied to disk_image successfully!" -ForegroundColor Green