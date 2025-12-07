-- Seed predefined formations
INSERT INTO formations (name, description, positions, is_custom) VALUES
('4-4-2', 'Classic formation with two strikers and balanced midfield',
'[
  {"position": "GK", "x": 50, "y": 5},
  {"position": "DR", "x": 80, "y": 25},
  {"position": "DCR", "x": 65, "y": 20},
  {"position": "DCL", "x": 35, "y": 20},
  {"position": "DL", "x": 20, "y": 25},
  {"position": "MR", "x": 80, "y": 55},
  {"position": "MCR", "x": 60, "y": 50},
  {"position": "MCL", "x": 40, "y": 50},
  {"position": "ML", "x": 20, "y": 55},
  {"position": "STR", "x": 60, "y": 85},
  {"position": "STL", "x": 40, "y": 85}
]', 0),

('4-3-3', 'Attacking formation with wingers and one striker',
'[
  {"position": "GK", "x": 50, "y": 5},
  {"position": "DR", "x": 80, "y": 25},
  {"position": "DCR", "x": 65, "y": 20},
  {"position": "DCL", "x": 35, "y": 20},
  {"position": "DL", "x": 20, "y": 25},
  {"position": "DMC", "x": 50, "y": 40},
  {"position": "MCR", "x": 65, "y": 55},
  {"position": "MCL", "x": 35, "y": 55},
  {"position": "AMR", "x": 80, "y": 75},
  {"position": "AML", "x": 20, "y": 75},
  {"position": "STC", "x": 50, "y": 85}
]', 0),

('4-2-3-1', 'Modern formation with attacking midfielder',
'[
  {"position": "GK", "x": 50, "y": 5},
  {"position": "DR", "x": 80, "y": 25},
  {"position": "DCR", "x": 65, "y": 20},
  {"position": "DCL", "x": 35, "y": 20},
  {"position": "DL", "x": 20, "y": 25},
  {"position": "DMCR", "x": 60, "y": 40},
  {"position": "DMCL", "x": 40, "y": 40},
  {"position": "AMR", "x": 75, "y": 65},
  {"position": "AMC", "x": 50, "y": 70},
  {"position": "AML", "x": 25, "y": 65},
  {"position": "STC", "x": 50, "y": 85}
]', 0),

('3-5-2', 'Formation with wing-backs',
'[
  {"position": "GK", "x": 50, "y": 5},
  {"position": "DCR", "x": 70, "y": 20},
  {"position": "DC", "x": 50, "y": 18},
  {"position": "DCL", "x": 30, "y": 20},
  {"position": "WBR", "x": 85, "y": 50},
  {"position": "MCR", "x": 60, "y": 50},
  {"position": "MC", "x": 50, "y": 45},
  {"position": "MCL", "x": 40, "y": 50},
  {"position": "WBL", "x": 15, "y": 50},
  {"position": "STR", "x": 60, "y": 85},
  {"position": "STL", "x": 40, "y": 85}
]', 0),

('4-1-4-1', 'Defensive formation with holding midfielder',
'[
  {"position": "GK", "x": 50, "y": 5},
  {"position": "DR", "x": 80, "y": 25},
  {"position": "DCR", "x": 65, "y": 20},
  {"position": "DCL", "x": 35, "y": 20},
  {"position": "DL", "x": 20, "y": 25},
  {"position": "DMC", "x": 50, "y": 38},
  {"position": "MR", "x": 80, "y": 60},
  {"position": "MCR", "x": 60, "y": 55},
  {"position": "MCL", "x": 40, "y": 55},
  {"position": "ML", "x": 20, "y": 60},
  {"position": "STC", "x": 50, "y": 85}
]', 0);
