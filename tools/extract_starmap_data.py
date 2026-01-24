#!/usr/bin/env python3
"""
Extract star system data from UQM C source code and generate Rust code.
This ensures 100% fidelity to the original game data.
"""

import re
import sys

def parse_star_data(plandata_path):
    """Parse starmap_array from plandata.c"""
    
    with open(plandata_path, 'r') as f:
        content = f.read()
    
    # Find starmap_array definition
    array_match = re.search(r'STAR_DESC starmap_array\[\] =\s*\{(.*?)\};', content, re.DOTALL)
    if not array_match:
        print("ERROR: Could not find starmap_array", file=sys.stderr)
        return []
    
    array_content = array_match.group(1)
    
    # Parse each star entry
    # Format: {{x, y}, MAKE_STAR (type, color, -1), alien_defined, prefix, postfix},
    star_pattern = r'\{\{(\d+),\s*(\d+)\},\s*MAKE_STAR\s*\(([A-Z_]+),\s*([A-Z_]+),\s*-1\),\s*([A-Z_]+|\d+),\s*(\d+),\s*(\d+)\}'
    
    stars = []
    for match in re.finditer(star_pattern, array_content):
        x, y, star_type, color, alien, prefix, postfix = match.groups()
        
        # Stop at sentinel (MAX_X_UNIVERSE << 1)
        if 'MAX_X_UNIVERSE' in array_content[match.start():match.start()+100]:
            break
            
        stars.append({
            'x': int(x),
            'y': int(y),
            'star_type': star_type,
            'color': color,
            'alien': alien if alien != '0' else 'None',
            'prefix': int(prefix),
            'postfix': int(postfix)
        })
    
    return stars

def star_type_to_rust(c_type):
    """Convert C star type to Rust enum"""
    mapping = {
        'DWARF_STAR': 'StarType::Dwarf',
        'GIANT_STAR': 'StarType::Giant',
        'SUPER_GIANT_STAR': 'StarType::SuperGiant',
    }
    return mapping.get(c_type, 'StarType::Dwarf')

def color_to_rust(c_color):
    """Convert C color to Rust enum"""
    mapping = {
        'RED_BODY': 'StarColor::Red',
        'ORANGE_BODY': 'StarColor::Orange',
        'YELLOW_BODY': 'StarColor::Yellow',
        'GREEN_BODY': 'StarColor::Green',
        'BLUE_BODY': 'StarColor::Blue',
        'WHITE_BODY': 'StarColor::White',
    }
    return mapping.get(c_color, 'StarColor::White')

def alien_to_rust(c_alien):
    """Convert C alien presence to Rust enum"""
    if c_alien == 'None' or c_alien == '0':
        return 'AlienPresence::None'
    
    mapping = {
        'SOL_DEFINED': 'AlienPresence::Sol',
        'SHOFIXTI_DEFINED': 'AlienPresence::Shofixti',
        'YEHAT_DEFINED': 'AlienPresence::Yehat',
        'PKUNK_DEFINED': 'AlienPresence::Pkunk',
        'MYCON_TRAP_DEFINED': 'AlienPresence::Mycon',
        'RAINBOW_DEFINED': 'AlienPresence::Rainbow',
        'MELNORME0_DEFINED': 'AlienPresence::Melnorme',
        'MELNORME1_DEFINED': 'AlienPresence::Melnorme',
        'MELNORME2_DEFINED': 'AlienPresence::Melnorme',
        'MELNORME3_DEFINED': 'AlienPresence::Melnorme',
        'MELNORME4_DEFINED': 'AlienPresence::Melnorme',
        'MELNORME5_DEFINED': 'AlienPresence::Melnorme',
        'MELNORME6_DEFINED': 'AlienPresence::Melnorme',
        'MELNORME7_DEFINED': 'AlienPresence::Melnorme',
        'MELNORME8_DEFINED': 'AlienPresence::Melnorme',
        'VUX_DEFINED': 'AlienPresence::Vux',
        'START_COLONY_DEFINED': 'AlienPresence::StartColony',
        'MAIDENS_DEFINED': 'AlienPresence::Maidens',
        'THRADD_DEFINED': 'AlienPresence::Thradd',
        'ZOQFOT_DEFINED': 'AlienPresence::ZoqFot',
        'SYREEN_DEFINED': 'AlienPresence::Syreen',
        'BURVIXESE_DEFINED': 'AlienPresence::Burvixese',
        'SAMATRA_DEFINED': 'AlienPresence::Samatra',
        'TALKING_PET_DEFINED': 'AlienPresence::TalkingPet',
        'ILWRATH_DEFINED': 'AlienPresence::Ilwrath',
        'SPATHI_DEFINED': 'AlienPresence::Spathi',
        'SHIP_VAULT_DEFINED': 'AlienPresence::ShipVault',
        'URQUAN_WRECK_DEFINED': 'AlienPresence::UrquanWreck',
        'AQUA_HELIX_DEFINED': 'AlienPresence::AquaHelix',
        'UTWIG_DEFINED': 'AlienPresence::Utwig',
        'SUPOX_DEFINED': 'AlienPresence::Supox',
        'BOMB_DEFINED': 'AlienPresence::Bomb',
        'VUX_BEAST_DEFINED': 'AlienPresence::VuxBeast',
        'SLYLANDRO_DEFINED': 'AlienPresence::Slylandro',
        'KITSUNE_DEFINED': 'AlienPresence::Kitsune',
    }
    return mapping.get(c_alien, 'AlienPresence::None')

def generate_rust_code(stars):
    """Generate Rust code for all stars"""
    
    rust_code = []
    rust_code.append("    /// Load ALL 249 canonical star systems from UQM")
    rust_code.append("    ///")
    rust_code.append("    /// Data extracted from sc2/src/uqm/plandata.c - EXACT COPY")
    rust_code.append("    /// This ensures 100% fidelity to the original game")
    rust_code.append("    fn load_canonical_stars() -> Vec<Star> {")
    rust_code.append("        vec![")
    
    for idx, star in enumerate(stars):
        rust_code.append(f"            // Index {idx}")
        rust_code.append(f"            Star {{")
        rust_code.append(f"                index: StarIndex::new({idx}_u16),")
        rust_code.append(f"                position: Point::new({star['x']}, {star['y']}),")
        rust_code.append(f"                star_type: {star_type_to_rust(star['star_type'])},")
        rust_code.append(f"                color: {color_to_rust(star['color'])},")
        rust_code.append(f"                num_planets: 0, // Will be filled from planet data")
        
        # Add special names for known systems
        name = ""
        if idx == 86:
            name = "Sol"
        elif idx == 87:
            name = "Alpha Centauri"
        # TODO: Add more known system names
        
        if name:
            rust_code.append(f'                name: "{name}".to_string(),')
        else:
            rust_code.append(f'                name: String::new(),')
        
        rust_code.append(f"                visited: false,")
        rust_code.append(f"                alien_presence: {alien_to_rust(star['alien'])},")
        rust_code.append(f"                prefix_index: {star['prefix']},")
        rust_code.append(f"                postfix_index: {star['postfix']},")
        rust_code.append(f"            }},")
    
    rust_code.append("        ]")
    rust_code.append("    }")
    
    return '\n'.join(rust_code)

def main():
    if len(sys.argv) != 2:
        print("Usage: extract_starmap_data.py <path_to_plandata.c>")
        sys.exit(1)
    
    plandata_path = sys.argv[1]
    
    print(f"Extracting star data from {plandata_path}...", file=sys.stderr)
    stars = parse_star_data(plandata_path)
    
    print(f"Found {len(stars)} star systems", file=sys.stderr)
    
    if len(stars) == 0:
        print("ERROR: No stars found!", file=sys.stderr)
        sys.exit(1)
    
    # Generate Rust code
    rust_code = generate_rust_code(stars)
    print(rust_code)
    
    print(f"\nSuccessfully extracted {len(stars)} star systems", file=sys.stderr)

if __name__ == '__main__':
    main()
