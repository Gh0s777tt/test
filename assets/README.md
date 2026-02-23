# VANTIS OS Assets

This directory contains visual assets for the VANTIS OS project.

## Files

### Logo
- `vantis-logo.svg` - Main VANTIS OS logo (context-aware)
- `logo-context-aware.css` - CSS for auto-color switching

### Terminal Demos
- `boot-demo.cast` - Asciiinema recording of boot sequence
- `boot-demo.gif` - Animated GIF of boot sequence

### Usage

To use the context-aware logo in README:

```html
<link rel="stylesheet" href="assets/logo-context-aware.css">
<img src="assets/vantis-logo.svg" class="vantis-logo" alt="VANTIS OS Logo">
```

To embed the boot demo:

```html
<script src="https://cdn.jsdelivr.net/npm/asciinema-player@2.6.1/dist/bundle/asciinema-player.min.js"></script>
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/asciinema-player@2.6.1/dist/bundle/asciinema-player.min.css" />

<asciinema-player src="assets/boot-demo.cast" cols="80" rows="24" autoplay></asciinema-player>
```

## Credits

Logo design by VANTIS OS Team
Boot sequence demonstration by SuperNinja AI Agent