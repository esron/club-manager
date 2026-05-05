-- Fix any string values in active column
UPDATE members SET active = 1 WHERE active = 'true' OR active = '1';
UPDATE members SET active = 0 WHERE active = 'false' OR active = '0';
