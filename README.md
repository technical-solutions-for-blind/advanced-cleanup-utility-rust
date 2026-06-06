# Advanced System Cleanup Utility 1.4 - Rust Edition

**Specifically designed for Visually Impaired Users with Full Accessibility Support**

## Features

### 🧹 Cleaning Tools
- **Windows Temp Cleaner** - Deep cleaning of C:\Windows\Temp
- **User Temp Cleaner** - Local temp files cleanup
- **Prefetch Cache Cleaner** - C:\Windows\Prefetch cleanup
- **Windows Update Cleaner** - SoftwareDistribution folder + DISM cleanup
- **Run History Cleaner** - Registry-based run dialog history
- **Full System Optimization** - One-click complete cleanup

### ⚙️ Advanced System Tools
- **Real-time RAM Booster** - Live system memory optimization
- **Ultra Deep Health Monitor** - Comprehensive system information
- **Disk Cleanup Manager** - Windows disk cleanup integration
- **DNS Cache Flusher** - Network optimization
- **Start Menu Manager** - Toggle between classic and new Start Menu (Registry-based)
- **Process Manager** - Monitor and manage system processes
- **Explorer Restart** - Safe Windows Explorer restart

### 🌐 Community & Support
- **Social Community Center** - WhatsApp, Telegram, YouTube links
- **Support Team Contact** - Multiple contact channels
- **Donation Center** - QR code and UPI integration
- **Legal Center** - About, Privacy Policy, Terms & Conditions

### ♿ Accessibility Features
- Screen reader optimized with proper ARIA labels
- High contrast UI (Dark theme)
- Keyboard navigation support
- Real-time status updates
- Clear error messages

## Architecture

### Modular Structure
```
src/
├── main.rs              # UI Entry Point
├── lib.rs               # Library exports
├── modules/
│   ├── temp_cleaner.rs         # Temp file operations
│   ├── prefetch_cleaner.rs     # Prefetch cleanup
│   ├── windows_update_cleaner.rs # Update cache cleanup
│   ├── run_history_cleaner.rs  # Registry run history
│   ├── ram_booster.rs          # Real-time memory optimization
│   ├── system_monitor.rs       # System health info
│   ├── dns_flusher.rs          # DNS cache operations
│   ├── disk_cleaner.rs         # Disk cleanup utilities
│   ├── start_menu_manager.rs   # Registry-based Start Menu toggle
│   ├── registry_manager.rs     # Registry operations
│   ├── explorer_manager.rs     # Windows Explorer management
│   ├── process_manager.rs      # Process utilities
│   ├── drive_info.rs           # Drive information
│   ├── network_monitor.rs      # Network utilities
│   ├── system_info.rs          # System information
│   ├── accessibility.rs        # Accessibility utilities
│   ├── ui_components.rs        # UI widget helpers
│   ├── resource_loader.rs      # Asset/resource loading
│   ├── logger.rs               # Logging utilities
│   └── error_handler.rs        # Error handling
├── assets/
│   ├── qr.png                  # QR code image
│   ├── bell.wav                # Success sound
│   ├── about.txt               # About content
│   ├── privacy.txt             # Privacy policy
│   └── terms.txt               # Terms & conditions
└── Cargo.toml
```

## Requirements

- Windows 10/11
- Administrator privileges for system cleaning
- .NET Runtime (for some system operations)
- Rust 1.70+

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run --release
```

## Installation

1. Download the latest release
2. Run as Administrator
3. All operations require elevated privileges

## Safety Features

- Protected folder exclusions (AppData, Program Files)
- Safe file deletion with error handling
- Registry backup before modifications
- Confirmation dialogs for critical operations
- Detailed logging of all operations

## Accessibility

This application is built with accessibility as a core feature:
- NVDA and JAWS compatible
- Keyboard-only navigation
- Clear status updates
- Descriptive button labels
- High contrast interface

## Contact & Support

- **WhatsApp**: https://wa.me/917410817798
- **Telegram**: https://t.me/technical_solutions_for_blind
- **Email**: technicalsolutionsforblind@gmail.com
- **Community**: https://chat.whatsapp.com/G9IUpSpxPkm8JDvdnUPtEy

## License

MIT License - See LICENSE file for details

## Mission

Bridging the digital gap for visually impaired users by providing powerful, accessible system utilities.
