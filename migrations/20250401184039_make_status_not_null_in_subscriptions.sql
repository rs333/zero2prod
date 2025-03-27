-- Add migration script here
begin;
    -- Change existing entries so they are not null
    update subscriptions
        set status = 'unconfirmed'
        where status is null;

    -- alter table so they must be not null
    alter table subscriptions alter column status set not null;

commit;
