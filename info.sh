# Gets the current target for Rust
(eval $(rustc --print cfg | grep target); echo "Rust target:" $target_arch-$target_vendor-$target_os-$target_env)