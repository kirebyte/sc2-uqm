#!/bin/bash

echo "Applying additional warning fixes..."

# Fix widgets.c unused variables  
sed -i '405s/Color inactive, default_color, selected;/Color inactive __attribute__((unused)), default_color __attribute__((unused)), selected __attribute__((unused));/' src/libs/graphics/widgets.c
sed -i '615s/Color inactive;/Color inactive __attribute__((unused));/' src/libs/graphics/widgets.c

# Fix SDL graphics warnings
sed -i 's/static int ScreenFilterMode = 0;/static int ScreenFilterMode __attribute__((unused)) = 0;/' src/libs/graphics/sdl/sdl2_pure.c
sed -i 's/char \*caption;/char *caption __attribute__((unused));/' src/libs/graphics/sdl/sdl_common.c
sed -i 's/int result;/int result __attribute__((unused));/' src/libs/graphics/sdl/sdl_common.c

# Add pragma to suppress gamma parameter warning
sed -i '1i #pragma GCC diagnostic push\n#pragma GCC diagnostic ignored "-Wunused-parameter"' src/libs/graphics/sdl/sdl2_common.c
echo -e "\n#pragma GCC diagnostic pop" >> src/libs/graphics/sdl/sdl2_common.c

# Fix absolute value warnings in scaleint.h
sed -i 's/abs(/abs((int)(/g' src/libs/graphics/sdl/scaleint.h

echo "Additional warning fixes completed!"