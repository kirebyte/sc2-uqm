# Quick Reference: Adding Menu Options to UQM

## Essential Steps Checklist

### 1. Define Global Flag
`src/uqm/globdata.h`:
```c
OPTION_FLAG = (1 << X),  // Use next available bit
```

### 2. Add Menu Constant  
`src/uqm/menustat.h`:
```c
PM_YOUR_OPTION,  // Add in correct position!
```

### 3. Add Text Entry
`content/base/gamestrings.txt`:
```
#(your option)
your option text
```
**Position must match enum index!**

### 4. Add Setting Enum
`src/uqm/gameopt.c`:
```c
YOUR_OPTION_SETTING,
```

### 5. Implement Feedback
`src/uqm/gameopt.c` - `FeedbackSetting()`:
```c
case YOUR_OPTION_SETTING:
    // Show current state
    break;
```

### 6. Implement Logic
`src/uqm/gameopt.c` - `DoSettings()`:
```c
case YOUR_OPTION_SETTING:
    // Toggle/change option
    break;
```

### 7. Update Navigation
`src/uqm/menu.c` - Update these functions:
- `FixMenuState()`
- `NextMenuState()`  
- `PreviousMenuState()`
- `DrawPCMenuFrame()`

## Critical Rules

⚠️ **ENUM ORDER = GAMESTRINGS ORDER**  
⚠️ **Never insert enums in middle without adjusting all subsequent strings**  
⚠️ **Count positions carefully from MAINMENU_STRING_BASE**

## Testing
```bash
./build.sh uqm
./uqm-debug
```

## Common Issues
- **Wrong text displayed**: Check enum vs gamestrings alignment
- **Navigation broken**: Update all menu navigation functions
- **Crashes**: Verify all switch statements include new cases