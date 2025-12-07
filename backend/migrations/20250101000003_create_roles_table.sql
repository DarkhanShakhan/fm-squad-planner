-- Create roles table
CREATE TABLE IF NOT EXISTS roles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE, -- e.g., "Advanced Playmaker", "Ball-Winning Midfielder"
    position TEXT NOT NULL, -- GK, DR, DC, DL, WBR, WBL, DMC, MC, MR, ML, AMR, AML, AMC, STC
    duty TEXT NOT NULL, -- Support, Attack, Defend, Automatic (for GK)
    description TEXT,

    -- Attribute weights as JSON
    -- Structure: {attribute_name: weight} where weight is 0-1
    -- Higher weight = more important for this role
    attribute_weights TEXT NOT NULL, -- JSON object

    -- Metadata
    is_custom BOOLEAN DEFAULT 0, -- 0 for predefined, 1 for user-created
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create index on position for filtering roles by position
CREATE INDEX idx_roles_position ON roles(position);

-- Create index on name for lookups
CREATE INDEX idx_roles_name ON roles(name);

-- Create composite index for position + duty queries
CREATE INDEX idx_roles_position_duty ON roles(position, duty);
