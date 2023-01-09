create table hourly_stats
(
    block_num bigint not null
        constraint stats_pk primary key,
    chain text not null,
    trx_count integer not null,
    act_count integer not null
);

create table cursors
(
    id        text not null
        constraint cursor_pk primary key,
    cursor    text,
    block_num bigint,
    block_id  text
);