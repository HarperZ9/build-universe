# -*- mode: python ; coding: utf-8 -*-
# Calibrate Pro - PyInstaller Build Specification
# Build: pyinstaller calibrate_pro.spec --clean
# Output: dist/calibrate-pro/calibrate-pro.exe

import os

block_cipher = None
PROJECT_ROOT = os.path.abspath('.')

# Data files to bundle
datas = []

# dwm_lut binaries
dwm_lut_dir = os.path.join(PROJECT_ROOT, 'dwm_lut')
if os.path.isdir(dwm_lut_dir):
    for f in os.listdir(dwm_lut_dir):
        src = os.path.join(dwm_lut_dir, f)
        if os.path.isfile(src):
            datas.append((src, 'dwm_lut'))

# Icon
icon_path = os.path.join(PROJECT_ROOT, 'calibrate_pro', 'resources', 'calibrate_pro.ico')
if not os.path.exists(icon_path):
    icon_path = None

# Hidden imports PyInstaller misses (lazy/conditional imports)
hidden = [
    'scipy.interpolate', 'scipy.ndimage', 'scipy.optimize', 'scipy.spatial',
    'hid', 'tkinter',
    'pystray', 'pystray._win32', 'PIL', 'PIL.Image', 'PIL.ImageDraw',
    'PyQt6', 'PyQt6.QtWidgets', 'PyQt6.QtCore', 'PyQt6.QtGui', 'PyQt6.sip',
    'calibrate_pro.core.color_math', 'calibrate_pro.core.lut_engine',
    'calibrate_pro.core.icc_profile', 'calibrate_pro.core.calibration_engine',
    'calibrate_pro.panels.database', 'calibrate_pro.panels.detection',
    'calibrate_pro.sensorless.neuralux', 'calibrate_pro.sensorless.auto_calibration',
    'calibrate_pro.lut_system.dwm_lut',
    'calibrate_pro.hardware.i1d3_native', 'calibrate_pro.hardware.ddc_ci',
    'calibrate_pro.hardware.argyll_backend', 'calibrate_pro.hardware.measurement',
    'calibrate_pro.hardware.warmup_monitor', 'calibrate_pro.hardware.drift_compensation',
    'calibrate_pro.calibration.native_loop', 'calibrate_pro.calibration.hybrid',
    'calibrate_pro.calibration.ccss_import', 'calibrate_pro.calibration.targets',
    'calibrate_pro.verification.patch_sets', 'calibrate_pro.verification.report_generator',
    'calibrate_pro.display.hdr_detect', 'calibrate_pro.display.oled',
    'calibrate_pro.services.calibration_guard', 'calibrate_pro.services.app_switcher',
    'calibrate_pro.services.gamut_clamp', 'calibrate_pro.services.drift_monitor',
    'calibrate_pro.startup.calibration_loader',
    'calibrate_pro.utils.startup_manager',
    'calibrate_pro.platform.windows', 'calibrate_pro.platform.base',
    'calibrate_pro.integrations.resolve',
    'calibrate_pro.gui.app',
    'calibrate_pro.gui.pages.calibrate', 'calibrate_pro.gui.pages.verify',
    'calibrate_pro.gui.pages.profiles', 'calibrate_pro.gui.pages.ddc_control',
    'calibrate_pro.gui.pages.settings',
    'calibrate_pro.tray.tray_app',
]

a = Analysis(
    ['calibrate_pro/main.py'],
    pathex=[PROJECT_ROOT],
    binaries=[],
    datas=datas,
    hiddenimports=hidden,
    hookspath=[],
    hooksconfig={},
    runtime_hooks=[],
    excludes=[
        'calibrate_pro.platform.macos',
        'calibrate_pro.platform.linux',
    ],
    noarchive=False,
    optimize=0,
)

pyz = PYZ(a.pure, cipher=block_cipher)

exe = EXE(
    pyz,
    a.scripts,
    [],
    exclude_binaries=True,
    name='calibrate-pro',
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,
    console=True,
    disable_windowed_traceback=False,
    argv_emulation=False,
    target_arch=None,
    codesign_identity=None,
    entitlements_file=None,
    icon=icon_path,
)

coll = COLLECT(
    exe,
    a.binaries,
    a.datas,
    strip=False,
    upx=True,
    upx_exclude=[],
    name='calibrate-pro',
)
