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

#include "settings.h"

#include "globdata.h"
#include "libs/compiler.h"


static MUSIC_REF LastMusicRef;
static BOOLEAN LastContinuous;
static BYTE LastPriority;

void
ToggleMusic (void)
{
	GLOBAL (glob_flags) ^= MUSIC_DISABLED;
	if (LastPriority <= 1)
	{
		if (GLOBAL (glob_flags) & MUSIC_DISABLED)
			PLRStop (LastMusicRef);
		else if (LastMusicRef)
			PLRPlaySong (LastMusicRef, LastContinuous, LastPriority);
	}
}

void
PlayMusic (MUSIC_REF MusicRef, BOOLEAN Continuous, BYTE Priority)
{
	LastMusicRef = MusicRef;
	LastContinuous = Continuous;
	LastPriority = Priority;

	if (
#ifdef NEVER
		Priority > 1
		||
#endif /* NEVER */
		!(GLOBAL (glob_flags) & MUSIC_DISABLED)
		)
	{
		PLRPlaySong (MusicRef, Continuous, Priority);
	}
}

void
StopMusic (void)
{
	 PLRStop (LastMusicRef);
	 LastMusicRef = 0;
}

void
ResumeMusic (void)
{
	PLRResume (LastMusicRef);
}

void
PauseMusic (void)
{
	PLRPause (LastMusicRef);
}

void
ToggleSoundEffect (void)
{
	GLOBAL (glob_flags) ^= SOUND_DISABLED;
}

void
ToggleCheatMode (void)
{
	GLOBAL (glob_flags) ^= CHEAT_ENABLED;
}

BOOLEAN
IsCheatModeEnabled (void)
{
	return (GLOBAL (glob_flags) & CHEAT_ENABLED) != 0;
}

void
ApplyCheatModeEffects (void)
{
	if (IsCheatModeEnabled ())
	{
		// Example cheat effects - you can modify these as needed
		// Add extra resources when cheat mode is enabled
		// Note: These are examples, you should implement based on your needs
		
		// Example: Give extra crew
		// GLOBAL_SIS (CrewEnlisted) += 50;
		
		// Example: Give extra fuel
		// GLOBAL_SIS (FuelOnBoard) += 100;
		
		// Example: Give extra resource units (RU)
		// GLOBAL_SIS (ResUnits) += 1000;
		
		// For now, we'll just log that cheat mode is active
		// (You can add actual cheat effects here)
	}
}

void
PlaySoundEffect (SOUND S, COUNT Channel, SoundPosition Pos,
		void *PositionalObject, BYTE Priority)
{
	if (!(GLOBAL (glob_flags) & SOUND_DISABLED))
	{
		SetChannelVolume (Channel, MAX_VOLUME >> 1, Priority);
		//SetChannelRate (Channel, GetSampleRate (S), Priority);
		PlayChannel (Channel, S, Pos, PositionalObject, Priority);
	}
}

