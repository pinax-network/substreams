create table if not exists hourly_stats
(
    block_num bigint  not null
        constraint hourly_stats_pk primary key,
    chain     text    not null,
    trx_count integer not null,
    act_count integer not null
);

create table if not exists last_block
(
    chain     text    not null
        constraint last_block_pk primary key,
    block_num bigint  not null,
    trx_count integer not null,
    act_count integer not null
);

create table if not exists max_trx_block
(
    block_num bigint  not null
        constraint max_trx_block_pk primary key,
    chain     text    not null,
    trx_count integer not null
);

create table if not exists max_action_block
(
    block_num bigint  not null
        constraint max_action_block_pk primary key,
    chain     text    not null,
    act_count integer not null
);

create table if not exists cursors
(
    id        text not null
        constraint cursor_pk primary key,
    cursor    text,
    block_num bigint,
    block_id  text
);