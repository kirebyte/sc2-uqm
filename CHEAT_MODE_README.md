# Cheat Mode para The Ur-Quan Masters (UQM)

## Descripción

Se ha añadido una nueva funcionalidad de "Cheat Mode" al menú principal del juego The Ur-Quan Masters. Esta funcionalidad permite a los desarrolladores y jugadores activar modos especiales de juego para pruebas, debugging o simplemente para divertirse.

## Cómo activar el Cheat Mode

### Método 1: Desde el menú principal
1. Inicia el juego
2. En el menú principal, navega hasta la opción "Cheat Mode" (ubicada entre "Setup" y "Quit")
3. Presiona ENTER para alternar entre activado/desactivado
4. Continúa con el juego normalmente

### Método 2: Desde el menú de configuraciones del juego
1. Durante el juego, accede al menú de pausa/configuraciones
2. Ve a "Settings"
3. Encontrarás la opción "Cheat Mode" junto con las opciones de sonido y música
4. Presiona ENTER para alternar entre ON/OFF

## Estado del Cheat Mode

Cuando el Cheat Mode está activado:
- La configuración se guarda automáticamente
- El estado persiste entre sesiones de juego
- Puedes verificar el estado desde cualquier menú de configuraciones

## API para Desarrolladores

### Funciones disponibles

```c
// Alternar el estado del cheat mode
void ToggleCheatMode(void);

// Verificar si el cheat mode está activado
BOOLEAN IsCheatModeEnabled(void);

// Aplicar efectos del cheat mode (ejemplo)
void ApplyCheatModeEffects(void);
```

### Uso en el código

```c
#include "settings.h"

// Verificar si el cheat mode está activado
if (IsCheatModeEnabled()) {
    // Aplicar modificaciones especiales del juego
    // Por ejemplo: dar recursos extra, activar debug info, etc.
}
```

### Flag global

El cheat mode utiliza la flag global `CHEAT_ENABLED` definida en `globdata.h`:

```c
#define CHEAT_ENABLED (1 << 2)
```

Se puede verificar directamente usando:
```c
if (GLOBAL(glob_flags) & CHEAT_ENABLED) {
    // Cheat mode está activado
}
```

## Ejemplos de implementación

### Ejemplo 1: Recursos extra al iniciar un nuevo juego
```c
void StartNewGame(void) {
    // Código normal del juego...
    
    if (IsCheatModeEnabled()) {
        GLOBAL_SIS(CrewEnlisted) += 100;    // +100 crew
        GLOBAL_SIS(FuelOnBoard) += 500;     // +500 fuel
        GLOBAL_SIS(ResUnits) += 10000;      // +10000 RU
    }
}
```

### Ejemplo 2: Información de debug en combate
```c
void DrawBattleStatus(void) {
    // Dibujo normal del estado...
    
    if (IsCheatModeEnabled()) {
        // Mostrar información adicional de debug
        DrawDebugInfo();
        ShowHitboxes();
        DisplayAIState();
    }
}
```

### Ejemplo 3: Acceso a todas las tecnologías
```c
void InitPlayerTechnology(void) {
    // Inicialización normal...
    
    if (IsCheatModeEnabled()) {
        // Dar acceso a todas las tecnologías
        UnlockAllTechnology();
        SetMaxShipUpgrades();
    }
}
```

## Archivos modificados

Los siguientes archivos fueron modificados para implementar esta funcionalidad:

- `src/uqm/globdata.h` - Definición de la flag CHEAT_ENABLED
- `src/uqm/restart.c` - Añadida opción CHEAT_MODE al menú principal
- `src/uqm/gameopt.c` - Añadida opción al menú de configuraciones del juego
- `src/uqm/settings.c` - Funciones para manejar el cheat mode
- `src/uqm/settings.h` - Declaraciones de las nuevas funciones
- `src/uqm/imusicre.h` - Corrección para usar música existente
- `content/base/ui/newgame.ani` - Añadido frame para la nueva opción del menú
- `content/base/ui/newgame-006.png` - Nueva imagen para la opción "Cheat Mode"

## Notas para desarrolladores

1. **Persistencia**: El estado del cheat mode se guarda automáticamente con el resto de configuraciones del juego.

2. **Compatibilidad**: Los cambios son compatibles hacia atrás. Partidas guardadas sin cheat mode funcionarán normalmente.

3. **Extensibilidad**: La funcionalidad está diseñada para ser fácilmente extensible. Solo necesitas verificar `IsCheatModeEnabled()` donde quieras aplicar efectos especiales.

4. **Performance**: La verificación del cheat mode es muy eficiente (simple operación bitwise).

## Posibles mejoras futuras

- Añadir sub-menús de cheat con diferentes categorías
- Implementar cheats específicos (invencibilidad, recursos infinitos, etc.)
- Añadir hotkeys para activar/desactivar rápidamente
- Crear un sistema de cheats por consola
- Añadir efectos visuales para indicar cuando el cheat mode está activo

## Compilación

Después de hacer los cambios, compila el proyecto normalmente:

```bash
cd sc2
./build.sh uqm
```

El executable resultante (`uqm-debug` o `uqm`) incluirá la nueva funcionalidad de cheat mode.