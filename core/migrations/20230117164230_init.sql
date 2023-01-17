-- Add migration script here

CREATE TABLE IF NOT EXISTS planets (
    updated_at TIMESTAMPTZ NOT NULL,
    location VARCHAR(10) PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS resource (
    updated_at TIMESTAMPTZ NOT NULL,
    location VARCHAR(10) NOT NULL REFERENCES planets(location) PRIMARY KEY,
    metal VARCHAR(42) NOT NULL,
    crystal VARCHAR(42) NOT NULL,
    deuterium VARCHAR(42) NOT NULL,
    energy VARCHAR(42) NOT NULL
);

CREATE TABLE IF NOT EXISTS infrastructure (
    updated_at TIMESTAMPTZ NOT NULL,
    location VARCHAR(10) NOT NULL REFERENCES planets(location) PRIMARY KEY,
    metal_mine NUMERIC NOT NULL,
    crystal_mine NUMERIC NOT NULL,
    deuterium_synthesizer NUMERIC NOT NULL,
    energy_plant NUMERIC NOT NULL,
    fusion_reactor NUMERIC NOT NULL,
    solar_satellite NUMERIC NOT NULL,
    crawler NUMERIC NOT NULL,
    metal_storage NUMERIC NOT NULL,
    crystal_storage NUMERIC NOT NULL,
    deuterium_tank NUMERIC NOT NULL
);

CREATE TABLE IF NOT EXISTS facility (
    updated_at TIMESTAMPTZ NOT NULL,
    location VARCHAR(10) NOT NULL REFERENCES planets(location) PRIMARY KEY,
    robotics_factory NUMERIC NOT NULL,
    shipyard NUMERIC NOT NULL,
    research_lab NUMERIC NOT NULL,
    alliance_depot NUMERIC NOT NULL,
    missile_silo NUMERIC NOT NULL,
    nanite_factory NUMERIC NOT NULL,
    terraformer NUMERIC NOT NULL,
    space_dock NUMERIC NOT NULL
);

CREATE TABLE IF NOT EXISTS technology (
    updated_at TIMESTAMPTZ NOT NULL,
    location VARCHAR(10) NOT NULL REFERENCES planets(location) PRIMARY KEY,
    energy_technology NUMERIC NOT NULL,
    laser_technology NUMERIC NOT NULL,
    ion_technology NUMERIC NOT NULL,
    hyperspace_technology NUMERIC NOT NULL,
    plasma_technology NUMERIC NOT NULL,
    combustion_drive NUMERIC NOT NULL,
    impulse_drive NUMERIC NOT NULL,
    hyperspace_drive NUMERIC NOT NULL,
    espionage_technology NUMERIC NOT NULL,
    computer_technology NUMERIC NOT NULL,
    astrophysics NUMERIC NOT NULL,
    intergalactic_research_network NUMERIC NOT NULL,
    graviton_technology NUMERIC NOT NULL,
    armour_technology NUMERIC NOT NULL,
    weapons_technology NUMERIC NOT NULL,
    shielding_technology NUMERIC NOT NULL
);

CREATE TABLE IF NOT EXISTS defence (
    updated_at TIMESTAMPTZ NOT NULL,
    location VARCHAR(10) NOT NULL REFERENCES planets(location) PRIMARY KEY,
    rocket_launcher NUMERIC NOT NULL,
    light_laser NUMERIC NOT NULL,
    heavy_laser NUMERIC NOT NULL,
    ion_cannon NUMERIC NOT NULL,
    gauss_cannon NUMERIC NOT NULL,
    plasma_turret NUMERIC NOT NULL,
    small_shield_dome NUMERIC NOT NULL,
    large_shield_dome NUMERIC NOT NULL,
    anti_ballistic_missile NUMERIC NOT NULL,
    interplanetary_missile NUMERIC NOT NULL
);

CREATE TABLE IF NOT EXISTS fleet (
    updated_at TIMESTAMPTZ NOT NULL,
    location VARCHAR(10) NOT NULL REFERENCES planets(location) PRIMARY KEY,
    light_fighter NUMERIC NOT NULL,
    heavy_fighter NUMERIC NOT NULL,
    cruiser NUMERIC NOT NULL,
    battleship NUMERIC NOT NULL,
    battlecruiser NUMERIC NOT NULL,
    bomber NUMERIC NOT NULL,
    destroyer NUMERIC NOT NULL,
    deathstar NUMERIC NOT NULL,
    reaper NUMERIC NOT NULL,
    pathfinder NUMERIC NOT NULL,
    small_cargo_ship NUMERIC NOT NULL,
    large_cargo_ship NUMERIC NOT NULL,
    colony_ship NUMERIC NOT NULL,
    recycler NUMERIC NOT NULL,
    espionage_probe NUMERIC NOT NULL
);