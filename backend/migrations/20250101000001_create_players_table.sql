-- Create players table
CREATE TABLE IF NOT EXISTS players (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    age INTEGER NOT NULL,
    nationality TEXT,
    position TEXT NOT NULL, -- GK, DR, DC, DL, WBR, WBL, DMC, MC, MR, ML, AMR, AML, AMC, STC

    -- Technical attributes
    corners INTEGER CHECK(corners BETWEEN 1 AND 20),
    crossing INTEGER CHECK(crossing BETWEEN 1 AND 20),
    dribbling INTEGER CHECK(dribbling BETWEEN 1 AND 20),
    finishing INTEGER CHECK(finishing BETWEEN 1 AND 20),
    first_touch INTEGER CHECK(first_touch BETWEEN 1 AND 20),
    free_kick_taking INTEGER CHECK(free_kick_taking BETWEEN 1 AND 20),
    heading INTEGER CHECK(heading BETWEEN 1 AND 20),
    long_shots INTEGER CHECK(long_shots BETWEEN 1 AND 20),
    long_throws INTEGER CHECK(long_throws BETWEEN 1 AND 20),
    marking INTEGER CHECK(marking BETWEEN 1 AND 20),
    passing INTEGER CHECK(passing BETWEEN 1 AND 20),
    penalty_taking INTEGER CHECK(penalty_taking BETWEEN 1 AND 20),
    tackling INTEGER CHECK(tackling BETWEEN 1 AND 20),
    technique INTEGER CHECK(technique BETWEEN 1 AND 20),

    -- Mental attributes
    aggression INTEGER CHECK(aggression BETWEEN 1 AND 20),
    anticipation INTEGER CHECK(anticipation BETWEEN 1 AND 20),
    bravery INTEGER CHECK(bravery BETWEEN 1 AND 20),
    composure INTEGER CHECK(composure BETWEEN 1 AND 20),
    concentration INTEGER CHECK(concentration BETWEEN 1 AND 20),
    decisions INTEGER CHECK(decisions BETWEEN 1 AND 20),
    determination INTEGER CHECK(determination BETWEEN 1 AND 20),
    flair INTEGER CHECK(flair BETWEEN 1 AND 20),
    leadership INTEGER CHECK(leadership BETWEEN 1 AND 20),
    off_the_ball INTEGER CHECK(off_the_ball BETWEEN 1 AND 20),
    positioning INTEGER CHECK(positioning BETWEEN 1 AND 20),
    teamwork INTEGER CHECK(teamwork BETWEEN 1 AND 20),
    vision INTEGER CHECK(vision BETWEEN 1 AND 20),
    work_rate INTEGER CHECK(work_rate BETWEEN 1 AND 20),

    -- Physical attributes
    acceleration INTEGER CHECK(acceleration BETWEEN 1 AND 20),
    agility INTEGER CHECK(agility BETWEEN 1 AND 20),
    balance INTEGER CHECK(balance BETWEEN 1 AND 20),
    jumping_reach INTEGER CHECK(jumping_reach BETWEEN 1 AND 20),
    natural_fitness INTEGER CHECK(natural_fitness BETWEEN 1 AND 20),
    pace INTEGER CHECK(pace BETWEEN 1 AND 20),
    stamina INTEGER CHECK(stamina BETWEEN 1 AND 20),
    strength INTEGER CHECK(strength BETWEEN 1 AND 20),

    -- Goalkeeper attributes (nullable for outfield players)
    aerial_reach INTEGER CHECK(aerial_reach IS NULL OR aerial_reach BETWEEN 1 AND 20),
    command_of_area INTEGER CHECK(command_of_area IS NULL OR command_of_area BETWEEN 1 AND 20),
    communication INTEGER CHECK(communication IS NULL OR communication BETWEEN 1 AND 20),
    eccentricity INTEGER CHECK(eccentricity IS NULL OR eccentricity BETWEEN 1 AND 20),
    handling INTEGER CHECK(handling IS NULL OR handling BETWEEN 1 AND 20),
    kicking INTEGER CHECK(kicking IS NULL OR kicking BETWEEN 1 AND 20),
    one_on_ones INTEGER CHECK(one_on_ones IS NULL OR one_on_ones BETWEEN 1 AND 20),
    reflexes INTEGER CHECK(reflexes IS NULL OR reflexes BETWEEN 1 AND 20),
    rushing_out INTEGER CHECK(rushing_out IS NULL OR rushing_out BETWEEN 1 AND 20),
    punching INTEGER CHECK(punching IS NULL OR punching BETWEEN 1 AND 20),
    throwing INTEGER CHECK(throwing IS NULL OR throwing BETWEEN 1 AND 20),

    -- Additional info
    value INTEGER, -- in currency
    wage INTEGER, -- weekly wage
    contract_expiry DATE,

    -- Metadata
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create index on position for faster queries
CREATE INDEX idx_players_position ON players(position);

-- Create index on name for search
CREATE INDEX idx_players_name ON players(name);
