# How to Add Menu Options in The Ur-Quan Masters (UQM)

Este documento describe el proceso completo para agregar nuevas opciones al menú de configuración en UQM. Utilizaremos como ejemplo la implementación de "Cheat Mode" que se agregó al menú de configuraciones.

## Arquitectura del Sistema de Menús

UQM utiliza un sistema de menús basado en:
- **Enums (PM_*)**: Definen los identificadores de cada opción de menú
- **gamestrings.txt**: Contiene el texto que se muestra para cada opción
- **Índices**: Los valores de los enums se usan como índices para acceder a las cadenas de texto

### Flujo de datos:
```
PM_OPTION_NAME (enum) → MAINMENU_STRING_BASE + enum_value → gamestrings.txt[index] → Texto mostrado
```

## Archivos Involucrados

### Archivos principales a modificar:
1. `src/uqm/globdata.h` - Definir flags globales
2. `src/uqm/menustat.h` - Definir constantes de menú (PM_*)
3. `src/uqm/gameopt.c` - Lógica del menú de configuración
4. `src/uqm/menu.c` - Navegación y display de menús
5. `src/uqm/settings.c` - Funciones de utilidad (opcional)
6. `src/uqm/settings.h` - Headers de funciones (opcional)
7. `content/base/gamestrings.txt` - Textos del menú

## Paso a Paso: Agregar una Opción Toggle

### 1. Definir el Flag Global

**Archivo**: `src/uqm/globdata.h`

```c
// Agregar en el enum de flags globales
enum
{
    // ... flags existentes ...
    CHEAT_ENABLED        = (1 << 2),  // Usar el siguiente bit disponible
    // ... otros flags ...
};
```

**⚠️ IMPORTANTE**: Usar el siguiente bit disponible para evitar conflictos.

### 2. Definir la Constante de Menú

**Archivo**: `src/uqm/menustat.h`

```c
enum
{
    // ... opciones existentes ...
    PM_CYBORG_SUPER,
    PM_CHEAT_ON,        // ← Agregar AQUÍ, después de CYBORG_SUPER
    PM_CHANGE_CAPTAIN,  // ← Las opciones siguientes se desplazan
    PM_CHANGE_SHIP,
    PM_EXIT_SETTINGS,
    // ... resto del enum ...
};
```

**⚠️ CRÍTICO**: La posición en el enum determina el índice en gamestrings.txt. Insertar en el medio desplaza todas las opciones posteriores.

### 3. Agregar el Texto en gamestrings.txt

**Archivo**: `content/base/gamestrings.txt`

Buscar la sección que comienza con `#(scan) -- MAINMENU_STRING_BASE` y contar las posiciones:

```plaintext
#(scan) -- MAINMENU_STRING_BASE  ← PM_SCAN = 0
scan

#(starmap)                       ← PM_STARMAP = 1  
starmap

#(devices)                       ← PM_DEVICES = 2
devices

#(cargo)                         ← PM_CARGO = 3
cargo

#(roster)                        ← PM_ROSTER = 4
roster

#(game)                          ← PM_GAME_MENU = 5
game

#(navigate)                      ← PM_NAVIGATE = 6
navigate

#(mineral)                       ← PM_MIN_SCAN = 7
mineral

#(energy)                        ← PM_ENE_SCAN = 8
energy

#(biological)                    ← PM_BIO_SCAN = 9
biological

#(exit menu)                     ← PM_EXIT_SCAN = 10
exit menu

#(autoscan)                      ← PM_AUTO_SCAN = 11
autoscan

#(dispatch)                      ← PM_LAUNCH_LANDER = 12
dispatch

#(save game)                     ← PM_SAVE_GAME = 13
save game

#(load game)                     ← PM_LOAD_GAME = 14
load game

#(quit game)                     ← PM_QUIT_GAME = 15
quit game

#(settings)                      ← PM_CHANGE_SETTINGS = 16
settings

#(exit menu)                     ← PM_EXIT_GAME_MENU = 17
exit menu

#(converse)                      ← PM_CONVERSE = 18
converse

#(attack!)                       ← PM_ATTACK = 19
attack!

#(game)                          ← PM_ENCOUNTER_GAME_MENU = 20
game

#(fuel)                          ← PM_FUEL = 21
fuel

#(module)                        ← PM_MODULE = 22
module

#(game)                          ← PM_OUTFIT_GAME_MENU = 23
game

#(exit menu)                     ← PM_EXIT_OUTFIT = 24
exit menu

#(crew)                          ← PM_CREW = 25
crew

#(game)                          ← PM_SHIPYARD_GAME_MENU = 26
game

#(exit menu)                     ← PM_EXIT_SHIPYARD = 27
exit menu

#(sound)                         ← PM_SOUND_ON = 28
sound

#(music)                         ← PM_MUSIC_ON = 29
music

#(cyborg)                        ← PM_CYBORG_OFF = 30
cyborg

#(cheat on)                      ← PM_CHEAT_ON = 31 ← ¡AGREGAR AQUÍ!
cheats

#(captain)                       ← PM_CHANGE_CAPTAIN = 32
captain

#(flagship)                      ← PM_CHANGE_SHIP = 33
flagship

#(exit menu)                     ← PM_EXIT_SETTINGS = 34
exit menu
```

### 4. Definir el Enum de Configuración

**Archivo**: `src/uqm/gameopt.c`

```c
enum
{
    // ... configuraciones existentes ...
    CYBORG_SUPER_SETTING,
    CHEAT_ON_SETTING,        // ← Agregar aquí
    CHANGE_CAPTAIN_SETTING,
    // ... resto ...
};
```

### 5. Implementar el Feedback de la Configuración

**Archivo**: `src/uqm/gameopt.c` - Función `FeedbackSetting()`

```c
static void
FeedbackSetting (BYTE which_setting)
{
    UNICODE buf[128];
    const char *tmpstr;

    switch (which_setting)
    {
        // ... casos existentes ...
        case CHEAT_ON_SETTING:
            snprintf (buf, sizeof (buf) - 1, "Cheat Mode %s",
                    GLOBAL (glob_flags) & CHEAT_ENABLED
                    ? GAME_STRING (OPTION_STRING_BASE + 4)  // "ON"
                    : GAME_STRING (OPTION_STRING_BASE + 3)); // "OFF"
            break;
        // ... otros casos ...
    }

    DrawStatusMessage (buf);
}
```

### 6. Implementar la Lógica de Toggle

**Archivo**: `src/uqm/gameopt.c` - Función `DoSettings()` en el switch de `InputState & DEVICE_BUTTON1`

```c
switch (pMS->CurState)
{
    // ... casos existentes ...
    case CHEAT_ON_SETTING:
        ToggleCheatMode();  // Función de utilidad
        DrawMenuStateStrings (PM_SOUND_ON, pMS->CurState);
        break;
    // ... otros casos ...
}
```

### 7. Actualizar las Funciones de Navegación

**Archivo**: `src/uqm/menu.c`

#### Función `FixMenuState()`:
```c
static BYTE
FixMenuState (BYTE BadState)
{
    switch (BadState)
    {
        // ... casos existentes ...
        case PM_CHEAT_ON:
            return PM_CHEAT_ON;  // Solo una opción, no toggle visual
        // ... otros casos ...
    }
    return BadState;
}
```

#### Función `NextMenuState()`:
```c
switch (CurState + AdjBase)
{
    // ... casos existentes ...
    case PM_CHEAT_ON:
        NextState = PM_CHANGE_CAPTAIN;
        break;
    // ... otros casos ...
}
```

#### Función `PreviousMenuState()`:
```c
switch (CurState + AdjBase)
{
    // ... casos existentes ...
    case PM_CHEAT_ON:
        NextState = PM_CYBORG_OFF;
        break;
    case PM_CHANGE_CAPTAIN:
        NextState = PM_CHEAT_ON;
        break;
    // ... otros casos ...
}
```

#### Función `DrawPCMenuFrame()` - Mapeo de estados a posiciones visuales:
```c
switch (pos)
{
    // ... casos existentes ...
    case PM_CHEAT_ON:
        NewState = 3;  // Posición visual en el menú
        break;
    // ... otros casos ...
}
```

### 8. Funciones de Utilidad (Opcional)

**Archivo**: `src/uqm/settings.c`

```c
void ToggleCheatMode(void)
{
    GLOBAL(glob_flags) ^= CHEAT_ENABLED;
    
    // Aplicar efectos del cheat mode si es necesario
    if (GLOBAL(glob_flags) & CHEAT_ENABLED)
    {
        ApplyCheatModeEffects();
    }
}

BOOLEAN IsCheatModeEnabled(void)
{
    return (GLOBAL(glob_flags) & CHEAT_ENABLED) != 0;
}

void ApplyCheatModeEffects(void)
{
    // Implementar efectos específicos del cheat mode
    // Por ejemplo: recursos ilimitados, invencibilidad, etc.
}
```

**Archivo**: `src/uqm/settings.h`

```c
// Agregar las declaraciones de las funciones
void ToggleCheatMode(void);
BOOLEAN IsCheatModeEnabled(void);
void ApplyCheatModeEffects(void);
```

## Consideraciones Importantes

### 1. Orden de los Enums
- **NUNCA** insertar enums en el medio de la lista sin ajustar gamestrings.txt
- Siempre agregar al final de cada sección o ajustar TODOS los textos posteriores
- Los valores de enum se usan como índices directos a gamestrings.txt

### 2. Sincronización de Archivos
- `menustat.h` enum order ↔ `gamestrings.txt` string order
- Cada cambio en un archivo requiere verificación del otro

### 3. Compilación y Testing
```bash
cd /path/to/uqm
./build.sh uqm
./uqm-debug
```

### 4. Debugging
Si las opciones no aparecen correctamente:
1. Verificar que los índices de enum coincidan con gamestrings.txt
2. Comprobar que todas las funciones de navegación estén actualizadas
3. Verificar que el texto esté en la posición correcta

## Tipos de Opciones de Menú

### Toggle Simple (ON/OFF)
Como en el ejemplo del Cheat Mode - una sola opción que alterna estado.

### Multi-estado (Como Cyborg)
Múltiples valores que ciclan: OFF → NORMAL → DOUBLE → SUPER

### Acción (Como Captain/Ship)
Opciones que ejecutan una acción específica en lugar de cambiar estado.

## Ejemplo Completo: Cheat Mode

Esta implementación agrega una opción "Cheat Mode" que:
- Se muestra como "cheats" en el menú
- Funciona como toggle ON/OFF
- Se coloca entre "cyborg" y "captain"
- Usa el flag global CHEAT_ENABLED
- Proporciona feedback visual del estado actual

## Solución de Problemas Comunes

### Problema: Opciones desplazadas o incorrectas
**Causa**: Desincronización entre enum y gamestrings.txt
**Solución**: Verificar que cada PM_* tenga su correspondiente entrada en gamestrings.txt en la posición correcta

### Problema: Texto no se muestra
**Causa**: gamestrings.txt no actualizado o en posición incorrecta
**Solución**: Contar cuidadosamente las posiciones desde MAINMENU_STRING_BASE

### Problema: Navegación rota
**Causa**: Funciones de navegación no actualizadas
**Solución**: Actualizar NextMenuState(), PreviousMenuState(), y DrawPCMenuFrame()

---

**Autor**: Implementación de Cheat Mode en UQM  
**Fecha**: Octubre 2025  
**Versión**: 1.0