-- Add down migration script here
DELETE FROM users
WHERE user_id = '792f91bb-fb55-4d0e-9e4f-d9d89a14b129';