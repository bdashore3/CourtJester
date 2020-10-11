-- Add migration script here
DROP TABLE public.commands;

CREATE TABLE public.commands
(
    guild_id bigint NOT NULL,
    name text COLLATE pg_catalog."default" NOT NULL,
    content text COLLATE pg_catalog."default",
    CONSTRAINT commands_pkey PRIMARY KEY (guild_id, name),
    CONSTRAINT "FK_commands_guild_info_guild_id" FOREIGN KEY (guild_id)
        REFERENCES public.guild_info (guild_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE public.commands
    OWNER to postgres;