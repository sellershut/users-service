create table "user" (
    id varchar(21) primary key,
    ap_id varchar unique not null,
    username varchar not null,
    display_name varchar,
    avatar_url varchar,
    email varchar unique,
    inbox varchar unique not null,
    outbox varchar unique not null,
    summary varchar,
    public_key varchar not null,
    private_key varchar unique,
    local boolean not null,
    followers varchar[] not null default '{}',
    created_at timestamptz default current_timestamp not null,
    last_refreshed_at timestamptz default current_timestamp not null,
    updated_at timestamptz default current_timestamp not null,
    CONSTRAINT min_non_empty_length_check CHECK (
        length(trim(both from username)) >= 2
        AND length(trim(both from display_name)) >= 1
    )
);

create index idx_user_ap_id on "user" (ap_id);
create index idx_user_username on "user" (username);
create index idx_user_local on "user" (local);

create or replace function update_updated_at()
returns trigger as $$
begin
    new.updated_at = current_timestamp;
    return new;
end;
$$ language plpgsql;

create trigger set_updated_at
before update on "user"
for each row
execute function update_updated_at();
