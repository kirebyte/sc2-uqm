"""
UQM Golden Reference Implementation

This module provides a Python reference implementation of core UQM game mechanics.
The Rust implementation must match the behavior of these reference functions exactly.

This serves as the "source of truth" for validating the Rust migration.
"""

import math
from dataclasses import dataclass
from enum import Enum
from typing import List, Optional, Tuple


# ============================================================================
# Core Types
# ============================================================================

@dataclass
class Point:
    """2D point with integer coordinates"""
    x: int
    y: int
    
    def distance_to(self, other: 'Point') -> float:
        """Calculate Euclidean distance to another point"""
        dx = self.x - other.x
        dy = self.y - other.y
        return math.sqrt(dx * dx + dy * dy)
    
    def manhattan_distance(self, other: 'Point') -> int:
        """Calculate Manhattan distance to another point"""
        return abs(self.x - other.x) + abs(self.y - other.y)


@dataclass
class Vec2:
    """2D vector with floating-point coordinates"""
    x: float
    y: float
    
    def magnitude(self) -> float:
        """Calculate vector magnitude"""
        return math.sqrt(self.x * self.x + self.y * self.y)
    
    def normalize(self) -> 'Vec2':
        """Return normalized vector"""
        mag = self.magnitude()
        if mag > 1e-6:
            return Vec2(self.x / mag, self.y / mag)
        return Vec2(0.0, 0.0)
    
    def dot(self, other: 'Vec2') -> float:
        """Dot product with another vector"""
        return self.x * other.x + self.y * other.y
    
    def __add__(self, other: 'Vec2') -> 'Vec2':
        return Vec2(self.x + other.x, self.y + other.y)
    
    def __sub__(self, other: 'Vec2') -> 'Vec2':
        return Vec2(self.x - other.x, self.y - other.y)
    
    def __mul__(self, scalar: float) -> 'Vec2':
        return Vec2(self.x * scalar, self.y * scalar)


class StarType(Enum):
    """Star classification"""
    DWARF = 0
    GIANT = 1
    SUPER_GIANT = 2


class StarColor(Enum):
    """Star color"""
    RED = 0
    ORANGE = 1
    YELLOW = 2
    GREEN = 3
    BLUE = 4
    WHITE = 5


# ============================================================================
# Game Time
# ============================================================================

@dataclass
class GameTime:
    """In-game time representation"""
    day: int  # 0-29
    month: int  # 0-11
    year: int
    
    DAYS_PER_MONTH = 30
    MONTHS_PER_YEAR = 12
    
    @classmethod
    def new(cls) -> 'GameTime':
        """Create new game time at canonical start date"""
        return cls(day=17, month=2, year=2155)
    
    def advance_days(self, days: int) -> None:
        """Advance time by a number of days"""
        remaining_days = days + self.day
        
        while remaining_days >= self.DAYS_PER_MONTH:
            remaining_days -= self.DAYS_PER_MONTH
            self.month += 1
            
            if self.month >= self.MONTHS_PER_YEAR:
                self.month = 0
                self.year += 1
        
        self.day = remaining_days
    
    def format(self) -> str:
        """Format as YYYY-MM-DD"""
        return f"{self.year:04d}-{self.month+1:02d}-{self.day+1:02d}"


# ============================================================================
# Game State
# ============================================================================

class GlobalFlag(Enum):
    """Global game progression flags"""
    GAME_STARTED = 0
    INTRO_VIEWED = 1
    VISITED_STARBASE = 2
    MET_COMMANDER = 3
    MET_URQUAN = 10
    MET_ARILOU = 11
    MET_SPATHI = 12
    FOUND_KITSUNE = 100
    CHEAT_MODE = 999


@dataclass
class GameState:
    """Main game state"""
    location: Point
    current_system: Optional[int]
    fuel: int
    crew: int
    resource_units: int
    time: GameTime
    flags: set
    visited_systems: List[int]
    
    @classmethod
    def new(cls) -> 'GameState':
        """Create new game with default starting conditions"""
        return cls(
            location=Point(1752, 1450),  # Sol coordinates
            current_system=0,  # Sol system
            fuel=1000,
            crew=50,
            resource_units=0,
            time=GameTime.new(),
            flags=set(),
            visited_systems=[0]
        )
    
    def consume_fuel(self, amount: int) -> bool:
        """Consume fuel, return True if successful"""
        if self.fuel >= amount:
            self.fuel -= amount
            return True
        return False
    
    def add_fuel(self, amount: int) -> None:
        """Add fuel"""
        self.fuel = min(self.fuel + amount, 2**32 - 1)  # Saturating add
    
    def lose_crew(self, amount: int) -> bool:
        """Lose crew members, return True if successful"""
        if self.crew >= amount:
            self.crew -= amount
            return True
        return False
    
    def add_crew(self, amount: int) -> None:
        """Add crew members"""
        self.crew = min(self.crew + amount, 2**32 - 1)
    
    def spend_resources(self, amount: int) -> bool:
        """Spend resource units, return True if successful"""
        if self.resource_units >= amount:
            self.resource_units -= amount
            return True
        return False
    
    def add_resources(self, amount: int) -> None:
        """Add resource units"""
        self.resource_units = min(self.resource_units + amount, 2**32 - 1)
    
    def visit_system(self, system_id: int) -> None:
        """Mark a system as visited"""
        if system_id not in self.visited_systems:
            self.visited_systems.append(system_id)
    
    def has_visited(self, system_id: int) -> bool:
        """Check if system has been visited"""
        return system_id in self.visited_systems
    
    def is_game_over(self) -> bool:
        """Check if game is over (no crew)"""
        return self.crew == 0


# ============================================================================
# Starmap
# ============================================================================

@dataclass
class Star:
    """Star system description"""
    index: int
    position: Point
    star_type: StarType
    color: StarColor
    num_planets: int
    name: str
    visited: bool
    
    def distance_to(self, other: 'Star') -> float:
        """Calculate distance to another star"""
        return self.position.distance_to(other.position)


class StarMap:
    """Galaxy star map"""
    def __init__(self):
        self.stars: List[Star] = []
    
    def add_star(self, star: Star) -> None:
        """Add a star to the map"""
        self.stars.append(star)
    
    def get_star(self, index: int) -> Optional[Star]:
        """Get star by index"""
        for star in self.stars:
            if star.index == index:
                return star
        return None
    
    def find_closest_star(self, position: Point) -> Optional[Star]:
        """Find closest star to a position"""
        if not self.stars:
            return None
        
        closest = self.stars[0]
        min_dist = closest.position.distance_to(position)
        
        for star in self.stars[1:]:
            dist = star.position.distance_to(position)
            if dist < min_dist:
                min_dist = dist
                closest = star
        
        return closest
    
    def calculate_fuel_cost(self, from_pos: Point, to_pos: Point) -> int:
        """Calculate fuel cost for travel"""
        distance = from_pos.distance_to(to_pos)
        return int(distance * 0.5)  # FUEL_CONSUMPTION_RATE = 0.5


# ============================================================================
# Ship System
# ============================================================================

class ShipType(Enum):
    """Ship type classification"""
    EARTHLING = 0
    URQUAN = 1
    ARILOU = 2
    SPATHI = 3


class ShipState(Enum):
    """Ship state"""
    IDLE = 0
    MOVING = 1
    IN_COMBAT = 2
    DEAD = 3


@dataclass
class Ship:
    """Ship instance"""
    ship_type: ShipType
    position: Vec2
    velocity: Vec2
    facing: int  # 0-31
    energy: int
    max_energy: int
    crew: int
    max_crew: int
    state: ShipState
    
    @classmethod
    def new(cls, ship_type: ShipType, position: Vec2) -> 'Ship':
        """Create a new ship"""
        # Stats by ship type
        stats = {
            ShipType.EARTHLING: (18, 50),
            ShipType.URQUAN: (42, 100),
            ShipType.ARILOU: (6, 40),
            ShipType.SPATHI: (20, 60),
        }
        max_crew, max_energy = stats[ship_type]
        
        return cls(
            ship_type=ship_type,
            position=position,
            velocity=Vec2(0.0, 0.0),
            facing=0,
            energy=max_energy,
            max_energy=max_energy,
            crew=max_crew,
            max_crew=max_crew,
            state=ShipState.IDLE
        )
    
    def take_damage(self, damage: int) -> None:
        """Take damage to crew"""
        self.crew = max(0, self.crew - damage)
        if self.crew == 0:
            self.state = ShipState.DEAD
    
    def update(self, dt: float) -> None:
        """Update ship state"""
        # Update position
        self.position = self.position + (self.velocity * dt)
        
        # Regenerate energy (5 per second)
        regen = int(5.0 * dt)
        self.energy = min(self.energy + regen, self.max_energy)
    
    def is_alive(self) -> bool:
        """Check if ship is alive"""
        return self.crew > 0


# ============================================================================
# Test Data
# ============================================================================

def create_test_data():
    """Create test data for validation"""
    # Test game state
    state = GameState.new()
    assert state.fuel == 1000
    assert state.crew == 50
    assert state.resource_units == 0
    
    # Test fuel consumption
    assert state.consume_fuel(200) == True
    assert state.fuel == 800
    assert state.consume_fuel(1000) == False
    assert state.fuel == 800
    
    # Test time advancement
    state.time.advance_days(10)
    assert state.time.day == 27
    
    # Test ship creation
    ship = Ship.new(ShipType.EARTHLING, Vec2(0.0, 0.0))
    assert ship.crew == 18
    assert ship.max_energy == 50
    
    # Test ship damage
    ship.take_damage(5)
    assert ship.crew == 13
    assert ship.is_alive() == True
    
    ship.take_damage(20)
    assert ship.crew == 0
    assert ship.is_alive() == False
    
    print("✅ All golden reference tests passed!")


if __name__ == "__main__":
    create_test_data()
    print("\n📖 Golden Reference Implementation Ready")
    print("   Use this as validation for Rust implementation")
