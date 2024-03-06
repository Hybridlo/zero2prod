-- Add up migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
    '792f91bb-fb55-4d0e-9e4f-d9d89a14b129',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$47mPWTSpfuzuKbcXjsk8cg$9XEF/rhj7xHwK8dUHxM0heMMup1danuCjXopQVM+cCw'
);