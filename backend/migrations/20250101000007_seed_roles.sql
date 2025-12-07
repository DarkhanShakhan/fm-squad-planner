-- Seed predefined roles with attribute weights
-- Weights range from 0.0 (not important) to 1.0 (very important)

-- Goalkeeper roles
INSERT INTO roles (name, position, duty, description, attribute_weights, is_custom) VALUES
('Goalkeeper', 'GK', 'Defend', 'Traditional goalkeeper focused on shot-stopping',
'{"aerial_reach": 0.8, "command_of_area": 0.7, "communication": 0.6, "handling": 0.9, "kicking": 0.5, "one_on_ones": 0.8, "reflexes": 1.0, "rushing_out": 0.4, "punching": 0.6, "throwing": 0.5, "anticipation": 0.7, "concentration": 0.8, "decisions": 0.7, "positioning": 0.9, "agility": 0.7, "jumping_reach": 0.6}',
0),

('Sweeper Keeper', 'GK', 'Support', 'Modern goalkeeper who acts as extra defender',
'{"aerial_reach": 0.7, "command_of_area": 0.8, "communication": 0.7, "handling": 0.8, "kicking": 0.9, "one_on_ones": 0.9, "reflexes": 0.9, "rushing_out": 1.0, "punching": 0.5, "throwing": 0.6, "anticipation": 0.9, "concentration": 0.8, "decisions": 0.9, "positioning": 0.8, "agility": 0.8, "pace": 0.7, "acceleration": 0.6}',
0);

-- Defender roles
INSERT INTO roles (name, position, duty, description, attribute_weights, is_custom) VALUES
('Full Back', 'DR', 'Support', 'Balanced full-back who supports attacks',
'{"crossing": 0.6, "dribbling": 0.5, "marking": 0.8, "passing": 0.6, "tackling": 0.9, "technique": 0.5, "anticipation": 0.7, "concentration": 0.7, "decisions": 0.7, "positioning": 0.8, "teamwork": 0.7, "work_rate": 0.8, "acceleration": 0.7, "pace": 0.8, "stamina": 0.8, "strength": 0.6}',
0),

('Wing Back', 'WBR', 'Attack', 'Attack-minded wing-back providing width',
'{"crossing": 0.9, "dribbling": 0.8, "marking": 0.6, "passing": 0.7, "tackling": 0.7, "technique": 0.7, "anticipation": 0.6, "decisions": 0.7, "flair": 0.6, "off_the_ball": 0.8, "teamwork": 0.7, "work_rate": 1.0, "acceleration": 0.9, "pace": 0.9, "stamina": 1.0, "agility": 0.7}',
0),

('Ball Playing Defender', 'DC', 'Defend', 'Centre-back comfortable on the ball',
'{"heading": 0.8, "marking": 0.9, "passing": 0.9, "tackling": 0.9, "technique": 0.8, "anticipation": 0.8, "composure": 0.9, "concentration": 0.8, "decisions": 0.8, "positioning": 0.9, "vision": 0.7, "acceleration": 0.5, "jumping_reach": 0.7, "strength": 0.8}',
0),

('Central Defender', 'DC', 'Defend', 'Traditional no-nonsense centre-back',
'{"heading": 1.0, "marking": 1.0, "tackling": 0.9, "anticipation": 0.8, "bravery": 0.8, "concentration": 0.9, "positioning": 0.9, "aggression": 0.6, "jumping_reach": 0.9, "strength": 0.9, "pace": 0.5}',
0);

-- Midfielder roles
INSERT INTO roles (name, position, duty, description, attribute_weights, is_custom) VALUES
('Ball Winning Midfielder', 'DMC', 'Defend', 'Defensive midfielder focused on winning the ball',
'{"marking": 0.9, "tackling": 1.0, "passing": 0.6, "anticipation": 0.9, "bravery": 0.8, "concentration": 0.8, "decisions": 0.8, "positioning": 0.9, "teamwork": 0.8, "work_rate": 1.0, "aggression": 0.8, "stamina": 0.9, "strength": 0.8}',
0),

('Deep Lying Playmaker', 'DMC', 'Support', 'Playmaker operating from deep',
'{"first_touch": 0.8, "passing": 1.0, "tackling": 0.5, "technique": 0.9, "anticipation": 0.8, "composure": 0.9, "decisions": 0.9, "flair": 0.7, "teamwork": 0.7, "vision": 1.0, "agility": 0.6, "balance": 0.6}',
0),

('Box to Box Midfielder', 'MC', 'Support', 'All-action midfielder covering the pitch',
'{"passing": 0.7, "tackling": 0.7, "technique": 0.7, "anticipation": 0.7, "decisions": 0.8, "determination": 0.8, "off_the_ball": 0.8, "positioning": 0.7, "teamwork": 0.8, "work_rate": 1.0, "acceleration": 0.7, "pace": 0.7, "stamina": 1.0}',
0),

('Advanced Playmaker', 'AMC', 'Support', 'Creative playmaker in advanced position',
'{"dribbling": 0.8, "first_touch": 0.9, "passing": 1.0, "technique": 0.9, "anticipation": 0.8, "composure": 0.8, "decisions": 0.9, "flair": 0.9, "off_the_ball": 0.7, "vision": 1.0, "agility": 0.7, "balance": 0.7}',
0),

('Winger', 'AMR', 'Support', 'Traditional winger providing width and crosses',
'{"crossing": 1.0, "dribbling": 0.9, "technique": 0.7, "anticipation": 0.6, "decisions": 0.7, "flair": 0.8, "off_the_ball": 0.7, "teamwork": 0.6, "work_rate": 0.7, "acceleration": 0.9, "agility": 0.8, "pace": 1.0}',
0);

-- Forward roles
INSERT INTO roles (name, position, duty, description, attribute_weights, is_custom) VALUES
('Advanced Forward', 'STC', 'Attack', 'Forward focused on getting in behind',
'{"dribbling": 0.7, "finishing": 1.0, "first_touch": 0.7, "heading": 0.6, "technique": 0.7, "anticipation": 0.9, "composure": 0.8, "decisions": 0.8, "off_the_ball": 1.0, "acceleration": 0.9, "agility": 0.7, "pace": 0.9}',
0),

('Target Man', 'STC', 'Support', 'Physical forward who holds up play',
'{"finishing": 0.8, "heading": 1.0, "passing": 0.6, "technique": 0.6, "anticipation": 0.7, "bravery": 0.8, "composure": 0.7, "decisions": 0.7, "off_the_ball": 0.7, "teamwork": 0.8, "work_rate": 0.7, "balance": 0.7, "jumping_reach": 1.0, "strength": 1.0}',
0),

('Complete Forward', 'STC', 'Support', 'Versatile forward capable of everything',
'{"dribbling": 0.8, "finishing": 0.9, "first_touch": 0.8, "heading": 0.7, "long_shots": 0.7, "passing": 0.7, "technique": 0.8, "anticipation": 0.8, "composure": 0.8, "decisions": 0.9, "flair": 0.7, "off_the_ball": 0.8, "teamwork": 0.7, "work_rate": 0.7, "acceleration": 0.7, "agility": 0.7, "balance": 0.7, "jumping_reach": 0.7, "pace": 0.7, "strength": 0.7}',
0);
