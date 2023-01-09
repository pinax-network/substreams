create table hourly_stats
(
    block_num bigint not null
        constraint stats_pk primary key,
    trx_count integer,
    act_count integer
);

create table cursors
(
    id        text not null
        constraint cursor_pk primary key,
    cursor    text,
    block_num bigint,
    block_id  text
);