-- Add migration script here
ALTER TABLE public.starbot
    RENAME TO starboard;

ALTER TABLE public.starboard
    ADD COLUMN delete_time bigint NOT NULL;

ALTER TABLE public.guild_info
    RENAME starbot_threshold TO starboard_threshold;
