# FPS Display Spoofer

A display-only native mod for Horizon. It changes the FPS number shown by both its HUD overlay and Horizon's F3/debug-overlay FPS line, cycling within **13,000–28,000 FPS**. It does not change rendering, frame pacing, benchmarks, game files, networking, or Horizon's real frame counter.

## Install

1. In Horizon Launcher, open **Mods** and select the Native Mods build profile.
2. Run the command below in PowerShell, substituting the path if you moved this folder:

   ```powershell
   horizon-launcher native-build "C:\Users\yoshi\Documents\Codex\2026-11-08\yk-x20\outputs\fps-display-spoofer\mixins\fps_display_spoofer" --trust-native-code
   ```

3. Start the resulting Native Mods instance, then enable **FPS Display Spoofer** at **Options → Mods**. Its settings page lets you choose the HUD corner.

The supplied `fps-display-spoofer-1.0.0.hnmod` contains the same readable source. Native mods execute as your user account; only build code you have reviewed.

License: MIT.
