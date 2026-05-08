# Gestor do Clube - Installation Guide

## System Requirements

### Windows
- Windows 10 or later (64-bit)
- 100 MB free disk space
- No additional dependencies required

### Linux
- Ubuntu 20.04+ / Fedora 35+ / Debian 11+ (or equivalent)
- 100 MB free disk space
- GLIBC 2.31 or later
- GTK3 libraries (usually pre-installed)

## Installation

### Windows Installation

1. **Download Installer**
   - Get `Gestor do Clube_1.0.0_x64-setup.exe` from releases page

2. **Run Installer**
   - Double-click the `.exe` file
   - If Windows SmartScreen appears, click "More info" → "Run anyway"
   - Follow installation wizard

3. **Launch Application**
   - Find "Gestor do Clube" in Start Menu
   - Or use desktop shortcut if created

### Linux Installation (AppImage)

1. **Download AppImage**
   - Get `gestor-do-clube_1.0.0_amd64.AppImage` from releases page

2. **Make Executable**
   ```bash
   chmod +x gestor-do-clube_1.0.0_amd64.AppImage
   ```

3. **Run Application**
   ```bash
   ./gestor-do-clube_1.0.0_amd64.AppImage
   ```

4. **Optional: Install to System**
   ```bash
   # Move to applications directory
   mv gestor-do-clube_1.0.0_amd64.AppImage ~/Applications/

   # Create desktop entry
   cat > ~/.local/share/applications/gestor-do-clube.desktop <<EOF
   [Desktop Entry]
   Name=Gestor do Clube
   Exec=$HOME/Applications/gestor-do-clube_1.0.0_amd64.AppImage
   Icon=gestor-do-clube
   Type=Application
   Categories=Office;Finance;
   EOF
   ```

### Linux Installation (DEB Package)

If `.deb` package is available:

```bash
sudo dpkg -i gestor-do-clube_1.0.0_amd64.deb
sudo apt-get install -f  # Fix dependencies if needed
```

Launch from applications menu or:
```bash
gestor-do-clube
```

## First Launch

1. **Create Password**
   - On first launch, you'll be prompted to create a password
   - Minimum 8 characters
   - **IMPORTANT:** There is no password recovery!
   - Write down your password in a secure location

2. **Data Location**
   - Windows: `C:\Users\{username}\Documents\GestorDoClube\`
   - Linux: `~/Documents/GestorDoClube/`
   
   This directory contains:
   - `clube.db` - Encrypted database
   - `config.json` - Application configuration

3. **Start Using**
   - Add members
   - Record payments
   - View dashboard

## Updating

### Windows
1. Download new installer
2. Run installer (will update existing installation)
3. Your data is preserved

### Linux (AppImage)
1. Download new AppImage
2. Replace old AppImage file
3. Your data is preserved (separate from AppImage)

## Uninstallation

### Windows
1. Go to Settings → Apps
2. Find "Gestor do Clube"
3. Click Uninstall

**Note:** Database is NOT deleted automatically. To remove data:
- Delete `C:\Users\{username}\Documents\GestorDoClube\`

### Linux (AppImage)
1. Delete the AppImage file
2. Delete desktop entry if created:
   ```bash
   rm ~/.local/share/applications/gestor-do-clube.desktop
   ```

**Note:** To remove data:
```bash
rm -rf ~/Documents/GestorDoClube/
```

## Troubleshooting

### Windows: "App can't run on your PC"
- You need 64-bit Windows
- Download the correct version

### Linux: "Permission denied"
```bash
chmod +x gestor-do-clube_1.0.0_amd64.AppImage
```

### Linux: "Error while loading shared libraries"
Install GTK3:
```bash
# Ubuntu/Debian
sudo apt install libgtk-3-0 libwebkit2gtk-4.0-37

# Fedora
sudo dnf install gtk3 webkit2gtk3
```

### "Database corrupted" or "Cannot open database"
- Your database file may be damaged
- Restore from backup if available
- Contact support with error details

### Forgot Password
- There is no password recovery
- You will need to create a new database (lose all data)
- Or restore from backup

## Data Backup

**Recommended: Backup regularly!**

### Manual Backup
1. Close the application
2. Copy the entire folder:
   - Windows: `C:\Users\{username}\Documents\GestorDoClube\`
   - Linux: `~/Documents/GestorDoClube/`
3. Paste to backup location (external drive, cloud, etc.)

### Restore from Backup
1. Close the application
2. Replace the folder with backup copy
3. Launch application
4. Enter your password

## Multi-Computer Setup

You can use the same database on multiple computers:

1. Copy `clube.db` to the same location on second computer
2. Install application on second computer
3. Open and enter password
4. Both computers use same data

**Important:** Don't use simultaneously on both computers. Always close on one before opening on the other.

## Support

- User Manual: See `docs/MANUAL_PT.md`
- GitHub Issues: Report bugs and request features
- Email: support@example.com (replace with actual support contact)

---

**Version:** 1.0.0  
**Last Updated:** May 2026
