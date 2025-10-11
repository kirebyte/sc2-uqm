//Copyright Paul Reiche, Fred Ford. 1992-2002

/*
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA.
 */

#include "genall.h"
#include "../planets.h"
#include "../../gendef.h"
#include "../../globdata.h"
#include "libs/mathlib.h"


static bool GenerateKitsune_generatePlanets (SOLARSYS_STATE *solarSys);
static bool GenerateKitsune_generateMoons (SOLARSYS_STATE *solarSys,
		PLANET_DESC *planet);


const GenerateFunctions generateKitsuneFunctions = {
	/* .initNpcs         = */ GenerateDefault_initNpcs,
	/* .reinitNpcs       = */ GenerateDefault_reinitNpcs,
	/* .uninitNpcs       = */ GenerateDefault_uninitNpcs,
	/* .generatePlanets  = */ GenerateKitsune_generatePlanets,
	/* .generateMoons    = */ GenerateKitsune_generateMoons,
	/* .generateName     = */ GenerateDefault_generateName,
	/* .generateOrbital  = */ GenerateDefault_generateOrbital,
	/* .generateMinerals = */ GenerateDefault_generateMinerals,
	/* .generateEnergy   = */ GenerateDefault_generateEnergy,
	/* .generateLife     = */ GenerateDefault_generateLife,
	/* .pickupMinerals   = */ GenerateDefault_pickupMinerals,
	/* .pickupEnergy     = */ GenerateDefault_pickupEnergy,
	/* .pickupLife       = */ GenerateDefault_pickupLife,
};


static bool
GenerateKitsune_generatePlanets (SOLARSYS_STATE *solarSys)
{
	COUNT angle;
	
	GenerateDefault_generatePlanets (solarSys);

	// Ajustar cantidad total de planetas y separar mas las orbitas
	// Vamos a usar incrementos mayores de radio y mantener 9 planetas en total
	solarSys->SunDesc[0].NumPlanets = 9;

	// Planeta 0: Big Water World con vida y una luna
	solarSys->PlanetDesc[0].data_index = WATER_WORLD;
	solarSys->PlanetDesc[0].NumPlanets = 1; // 1 luna
	solarSys->PlanetDesc[0].radius = EARTH_RADIUS * 140L / 100;
	angle = NORMALIZE_ANGLE (LOWORD (RandomContext_Random (SysGenRNG)));
	solarSys->PlanetDesc[0].location.x =
			COSINE (angle, solarSys->PlanetDesc[0].radius);
	solarSys->PlanetDesc[0].location.y =
			SINE (angle, solarSys->PlanetDesc[0].radius);

	// Planeta 1: Vinylogous World
	solarSys->PlanetDesc[1].data_index = VINYLOGOUS_WORLD;
	solarSys->PlanetDesc[1].NumPlanets = 0;
	solarSys->PlanetDesc[1].radius = EARTH_RADIUS * 180L / 100;
	angle = NORMALIZE_ANGLE (LOWORD (RandomContext_Random (SysGenRNG)));
	solarSys->PlanetDesc[1].location.x =
			COSINE (angle, solarSys->PlanetDesc[1].radius);
	solarSys->PlanetDesc[1].location.y =
			SINE (angle, solarSys->PlanetDesc[1].radius);

	// Planeta 2: Shattered World
	solarSys->PlanetDesc[2].data_index = SHATTERED_WORLD;
	solarSys->PlanetDesc[2].NumPlanets = 0;
	solarSys->PlanetDesc[2].radius = EARTH_RADIUS * 230L / 100;
	angle = NORMALIZE_ANGLE (LOWORD (RandomContext_Random (SysGenRNG)));
	solarSys->PlanetDesc[2].location.x =
			COSINE (angle, solarSys->PlanetDesc[2].radius);
	solarSys->PlanetDesc[2].location.y =
			SINE (angle, solarSys->PlanetDesc[2].radius);

	// Planeta 3: Primordial World
	solarSys->PlanetDesc[3].data_index = PRIMORDIAL_WORLD;
	solarSys->PlanetDesc[3].NumPlanets = 0;
	solarSys->PlanetDesc[3].radius = EARTH_RADIUS * 280L / 100;
	angle = NORMALIZE_ANGLE (LOWORD (RandomContext_Random (SysGenRNG)));
	solarSys->PlanetDesc[3].location.x =
			COSINE (angle, solarSys->PlanetDesc[3].radius);
	solarSys->PlanetDesc[3].location.y =
			SINE (angle, solarSys->PlanetDesc[3].radius);

	// Planeta 4: Oolite World
	solarSys->PlanetDesc[4].data_index = OOLITE_WORLD;
	solarSys->PlanetDesc[4].NumPlanets = 0;
	solarSys->PlanetDesc[4].radius = EARTH_RADIUS * 330L / 100;
	angle = NORMALIZE_ANGLE (LOWORD (RandomContext_Random (SysGenRNG)));
	solarSys->PlanetDesc[4].location.x =
			COSINE (angle, solarSys->PlanetDesc[4].radius);
	solarSys->PlanetDesc[4].location.y =
			SINE (angle, solarSys->PlanetDesc[4].radius);

	// Planeta 5: Violet Gas Giant
	solarSys->PlanetDesc[5].data_index = VIO_GAS_GIANT;
	solarSys->PlanetDesc[5].NumPlanets = 0;
	solarSys->PlanetDesc[5].radius = EARTH_RADIUS * 380L / 100;
	angle = NORMALIZE_ANGLE (LOWORD (RandomContext_Random (SysGenRNG)));
	solarSys->PlanetDesc[5].location.x =
			COSINE (angle, solarSys->PlanetDesc[5].radius);
	solarSys->PlanetDesc[5].location.y =
			SINE (angle, solarSys->PlanetDesc[5].radius);

	// Planeta 6: Purple World
	solarSys->PlanetDesc[6].data_index = PURPLE_WORLD;
	solarSys->PlanetDesc[6].NumPlanets = 0;
	solarSys->PlanetDesc[6].radius = EARTH_RADIUS * 430L / 100;
	angle = NORMALIZE_ANGLE (LOWORD (RandomContext_Random (SysGenRNG)));
	solarSys->PlanetDesc[6].location.x =
			COSINE (angle, solarSys->PlanetDesc[6].radius);
	solarSys->PlanetDesc[6].location.y =
			SINE (angle, solarSys->PlanetDesc[6].radius);

	// Planeta 7: Treasure World
	solarSys->PlanetDesc[7].data_index = TREASURE_WORLD;
	solarSys->PlanetDesc[7].NumPlanets = 0;
	solarSys->PlanetDesc[7].radius = EARTH_RADIUS * 480L / 100;
	angle = NORMALIZE_ANGLE (LOWORD (RandomContext_Random (SysGenRNG)));
	solarSys->PlanetDesc[7].location.x =
			COSINE (angle, solarSys->PlanetDesc[7].radius);
	solarSys->PlanetDesc[7].location.y =
			SINE (angle, solarSys->PlanetDesc[7].radius);

    // Planeta 8: Rainbow World
	solarSys->PlanetDesc[8].data_index = RAINBOW_WORLD;
	solarSys->PlanetDesc[8].NumPlanets = 0;
    solarSys->PlanetDesc[8].radius = EARTH_RADIUS * 530L / 100;
	angle = NORMALIZE_ANGLE (LOWORD (RandomContext_Random (SysGenRNG)));
	solarSys->PlanetDesc[8].location.x =
			COSINE (angle, solarSys->PlanetDesc[8].radius);
	solarSys->PlanetDesc[8].location.y =
			SINE (angle, solarSys->PlanetDesc[8].radius);

    // Quedan 9 planetas en total; los ultimos dos son Treasure (7) y Rainbow (8)

	return true;
}

static bool
GenerateKitsune_generateMoons (SOLARSYS_STATE *solarSys, PLANET_DESC *planet)
{
	GenerateDefault_generateMoons (solarSys, planet);

	// Luna del planeta 0: Small slave-shielded moon
	if (matchWorld (solarSys, planet, 0, MATCH_PLANET))
	{
		COUNT angle;
		DWORD rand_val;

		solarSys->MoonDesc[0].data_index = SELENIC_WORLD;
		solarSys->MoonDesc[0].radius = MIN_MOON_RADIUS;
		rand_val = RandomContext_Random (SysGenRNG);
		angle = NORMALIZE_ANGLE (LOWORD (rand_val));
		solarSys->MoonDesc[0].location.x =
				COSINE (angle, solarSys->MoonDesc[0].radius);
		solarSys->MoonDesc[0].location.y =
				SINE (angle, solarSys->MoonDesc[0].radius);

		// Make it shielded
		solarSys->MoonDesc[0].data_index = PLANET_SHIELDED;
	}

	return true;
}
