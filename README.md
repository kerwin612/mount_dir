# mount_dir

**Mount portable directory as consistent user directory.**
# Usage

**dependency**:
```bash
cargo add mount_dir
```

**import**
```rust
use mount_dir::mount;
```

**call**
```rust
mount(target_dir: String, mount_dir: String, is_force: bool);
```
