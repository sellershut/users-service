insert into "user" (
    id, ap_id, username, display_name, avatar_url, email, inbox, public_key, private_key, local, followers, outbox
) values
    ('12345', 'https://example.com/users/johndoe', 'johndoe', 'John Doe', 'https://example.com/avatar1.png', 'john.doe@example.com', 'https://example.com/users/johndoe/inbox', 'public_key_12345', 'private_key_12345', true, 
     '{"https://example.com/users/alice_w", "https://example.com/users/annasmith"}', 'https://example.com/users/johndoe/outbox'),
    ('12346', 'https://example.com/users/janedoe', 'janedoe', 'Jane Doe', 'https://example.com/avatar2.png', 'jane.doe@example.com', 'https://example.com/users/janedoe/inbox', 'public_key_12346', 'private_key_12346', true, 
     '{"https://example.com/users/user_1", "https://example.com/users/user_3"}', 'https://example.com/users/janedoe/outbox'),
    ('12347', 'https://example.com/users/mike23', 'mike23', 'Mike', 'https://example.com/avatar3.png', 'mike23@example.com', 'https://example.com/users/mike23/inbox', 'public_key_12347', NULL, false, 
     '{"https://example.com/users/user_1"}', 'https://example.com/users/mike23/outbox'),
    ('12348', 'https://example.com/users/annasmith', 'annasmith', 'Anna Smith', 'https://example.com/avatar4.png', 'anna.smith@example.com', 'https://example.com/users/annasmith/inbox', 'public_key_12348', NULL, true, 
     '{"https://example.com/users/user_2"}', 'https://example.com/users/annasmith/outbox'),
    ('12349', 'https://example.com/users/alice_w', 'alice_w', NULL, 'https://example.com/avatar5.png', 'alice.w@example.com', 'https://example.com/users/alice_w/inbox', 'public_key_12349', 'private_key_12349', true, 
     '{"https://example.com/users/user_3", "https://example.com/users/user_4"}', 'https://example.com/users/alice_w/outbox');

-- Insert user with NULL values (edge case)
insert into "user" (
    id, ap_id, username, display_name, avatar_url, email, inbox, public_key, private_key, local, followers, outbox
) values
    ('12350', 'https://example.com/users/testuser', 'testuser', NULL, NULL, 'test.user@example.com', 'https://example.com/users/testuser/inbox', 'public_key_12350', NULL, true, '{}', 'https://example.com/users/testuser/outbox');

-- Insert a user with only the required fields
insert into "user" (
    id, ap_id, username, inbox, public_key, local, followers, outbox
) values
    ('12351', 'https://example.com/users/bob1987', 'bob1987', 'https://example.com/users/bob1987/inbox', 'public_key_12351', false, '{}', 'https://example.com/users/bob1987/outbox');
