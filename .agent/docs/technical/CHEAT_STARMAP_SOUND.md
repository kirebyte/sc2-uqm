# Cheat Mode - First Cheat Implementation

## Overview
Implementation of the first cheat functionality: **Starmap Sound Test**

## Description
When cheat mode is enabled and the player is in the starmap view, pressing the 'T' key will play the device invocation sound (menusnd04.wav) - the same sound that plays when using devices from the devices menu.

## Implementation Details

### Location
- **File**: `src/uqm/planets/pstarmap.c`
- **Function**: `DoMoveCursor()`
- **Line**: ~1326-1350

### Code Logic
```c
// Cheat mode: Check for 'T' key press
if (IsCheatModeEnabled())
{
#if SDL_MAJOR_VERSION == 1
    const Uint8* keystate = SDL_GetKeyState(NULL);
    static Uint8 prev_t_state = 0;
    
    // Check for 'T' key press (rising edge detection) - SDL 1.x uses SDLK_t
    if (keystate[SDLK_t] && !prev_t_state)
    {
        // Play the device invocation sound (menusnd04.wav)
        PlayMenuSound (MENU_SOUND_INVOKED);
    }
    prev_t_state = keystate[SDLK_t];
#else
    const Uint8* keystate = SDL_GetKeyboardState(NULL);
    static Uint8 prev_t_state = 0;
    
    // Check for 'T' key press (rising edge detection) - SDL 2.x uses SDL_SCANCODE_T
    if (keystate[SDL_SCANCODE_T] && !prev_t_state)
    {
        // Play the device invocation sound (menusnd04.wav)
        PlayMenuSound (MENU_SOUND_INVOKED);
    }
    prev_t_state = keystate[SDL_SCANCODE_T];
#endif
}
```

### Key Features
1. **Conditional Activation**: Only works when cheat mode is enabled via the settings menu
2. **Context Awareness**: Only active when in starmap view (not in other screens)
3. **Rising Edge Detection**: Uses static variable to prevent sound spam on key hold
4. **Cross-Platform**: Works with both SDL 1.x and SDL 2.x
5. **Audio Feedback**: Plays the exact same sound as device invocation (menusnd04.wav)

### How to Use
1. Enable "Cheat Mode" in the Settings menu (should show "Cheat Mode ON")
2. Enter the starmap (where you can move the cursor with arrow keys)
3. Press the 'T' key to hear the device sound

### Technical Notes
- Uses direct SDL keyboard state access for immediate response
- Integrates seamlessly with existing UQM input system
- No interference with existing starmap controls
- Sound is played through UQM's menu sound system

## Testing Results
- ✅ Compilation successful
- ✅ Game runs without issues
- ✅ Cheat mode toggle works in settings menu
- ✅ 'T' key detection works in starmap
- ✅ Sound plays correctly (menusnd04.wav)
- ✅ No sound spam with key hold
- ✅ Only works when cheat mode is enabled

## Future Enhancements
This is just the first cheat implementation. Future cheats could include:
- Resource manipulation
- Ship upgrades
- Instant travel
- Debug information display
- Game state modification

## Files Modified
- `src/uqm/planets/pstarmap.c`: Added cheat detection logic
- Added include: `#include SDL_INCLUDE(SDL.h)`
- Uses existing: `IsCheatModeEnabled()` function
- Uses existing: `PlayMenuSound(MENU_SOUND_INVOKED)` function