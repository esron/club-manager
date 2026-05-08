# Production Build Testing Checklist

## Pre-Release Testing

### Windows Build Testing

- [ ] **Build Process**
  - [ ] Icons generated successfully
  - [ ] Build completes without errors
  - [ ] Installer (.exe) created
  - [ ] Installer size reasonable (< 50 MB)

- [ ] **Installer Testing**
  - [ ] Installer runs on Windows 10
  - [ ] Installer runs on Windows 11
  - [ ] License agreement displays correctly
  - [ ] Installation directory can be chosen
  - [ ] Shortcuts created (Start Menu)
  - [ ] Application icon displays correctly

- [ ] **Application Testing**
  - [ ] Application launches from Start Menu
  - [ ] Window title is "Gestor do Clube"
  - [ ] Application icon in taskbar
  - [ ] No DevTools visible
  - [ ] No console window
  - [ ] First launch: password creation works
  - [ ] Database created in Documents folder
  - [ ] Can close and reopen with password

- [ ] **Functionality**
  - [ ] Add member works
  - [ ] Add payment works
  - [ ] Dashboard displays correctly
  - [ ] Charts render properly
  - [ ] Export CSV works
  - [ ] Export XLSX works
  - [ ] Password change works
  - [ ] Search works

- [ ] **Uninstall**
  - [ ] Uninstaller appears in Apps & Features
  - [ ] Uninstall completes successfully
  - [ ] Database files remain (not deleted)

### Linux Build Testing

- [ ] **Build Process**
  - [ ] Icons generated successfully
  - [ ] Build completes without errors
  - [ ] AppImage created
  - [ ] DEB package created (if configured)
  - [ ] File sizes reasonable

- [ ] **AppImage Testing**
  - [ ] AppImage runs on Ubuntu 22.04
  - [ ] AppImage runs on Fedora 38+
  - [ ] AppImage runs on Debian 12
  - [ ] Execute permission works
  - [ ] Desktop integration works
  - [ ] Application icon displays

- [ ] **Application Testing**
  - [ ] Application launches
  - [ ] Window title correct
  - [ ] No DevTools visible
  - [ ] No terminal output (warnings/errors)
  - [ ] First launch: password creation
  - [ ] Database created in ~/Documents
  - [ ] Can reopen with password

- [ ] **Functionality**
  - [ ] All core features work
  - [ ] Charts render
  - [ ] Export works
  - [ ] Password change works
  - [ ] Search works

### Cross-Platform Testing

- [ ] **Database Portability**
  - [ ] Copy database from Windows to Linux
  - [ ] Open on Linux with same password → works
  - [ ] Copy database from Linux to Windows
  - [ ] Open on Windows with same password → works

- [ ] **Export Files**
  - [ ] CSV exports open in LibreOffice Calc
  - [ ] CSV exports open in Excel
  - [ ] CSV exports open in Google Sheets
  - [ ] XLSX exports open in LibreOffice Calc
  - [ ] XLSX exports open in Excel
  - [ ] XLSX exports open in Google Sheets
  - [ ] Brazilian currency format preserved
  - [ ] Special characters (ç, ã, etc.) display correctly

### Security Testing

- [ ] **Password Protection**
  - [ ] Wrong password rejected
  - [ ] Correct password allows access
  - [ ] Password change invalidates old password
  - [ ] Export requires password re-entry
  - [ ] Database file encrypted (cannot read with text editor)

- [ ] **Data Privacy**
  - [ ] Anonymous export hides names
  - [ ] Complete export shows names
  - [ ] No sensitive data in logs
  - [ ] No plaintext passwords anywhere

### Performance Testing

- [ ] **Large Dataset**
  - [ ] Add 100 members
  - [ ] Add 1000+ payments
  - [ ] Dashboard loads < 2 seconds
  - [ ] Member list loads < 1 second
  - [ ] Search responds instantly
  - [ ] Export completes < 10 seconds

### Documentation Testing

- [ ] **User Manual**
  - [ ] All sections accurate
  - [ ] Screenshots (if any) match current version
  - [ ] No broken links
  - [ ] File paths correct for OS

- [ ] **Installation Guide**
  - [ ] Instructions accurate
  - [ ] Commands work as written
  - [ ] Troubleshooting tips valid

### Final Checks

- [ ] **Version Numbers**
  - [ ] package.json version = 1.0.0
  - [ ] tauri.conf.json version = 1.0.0
  - [ ] README version = 1.0.0
  - [ ] Manual version = 1.0.0

- [ ] **Branding**
  - [ ] All icons display correctly
  - [ ] Application name consistent everywhere
  - [ ] Copyright year correct
  - [ ] License file present

- [ ] **Build Artifacts**
  - [ ] Windows installer in release folder
  - [ ] Linux AppImage in release folder
  - [ ] File names follow convention
  - [ ] README has correct download links

## Issue Tracking

| Issue | Platform | Severity | Status | Notes |
|-------|----------|----------|--------|-------|
|       |          |          |        |       |

## Sign-off

- [ ] All critical issues resolved
- [ ] All tests passed
- [ ] Documentation reviewed
- [ ] Ready for release

**Tested by:** _______________  
**Date:** _______________  
**Version:** 1.0.0
