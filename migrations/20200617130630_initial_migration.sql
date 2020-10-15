-- Add migration script here
CREATE TABLE public.guild_info
(
    guild_id bigint NOT NULL,
    prefix text COLLATE pg_catalog."default",
    CONSTRAINT guild_info_pkey PRIMARY KEY (guild_id)
)

TABLESPACE pg_default;

ALTER TABLE public.guild_info
    OWNER to postgres;

CREATE TABLE public.commands
(
    "Id" uuid NOT NULL,
    guild_id bigint NOT NULL,
    name text COLLATE pg_catalog."default",
    content text COLLATE pg_catalog."default",
    CONSTRAINT commands_pkey PRIMARY KEY ("Id"),
    CONSTRAINT "FK_commands_guild_info_guild_id" FOREIGN KEY (guild_id)
        REFERENCES public.guild_info (guild_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID
)

TABLESPACE pg_default;

ALTER TABLE public.commands
    OWNER to postgres;

CREATE TABLE public.text_channels
(
    guild_id bigint NOT NULL,
    nice_id bigint NOT NULL,
    bruh_id bigint NOT NULL,
    quote_id bigint NOT NULL,
    CONSTRAINT text_channels_pkey PRIMARY KEY (guild_id),
    CONSTRAINT "FK_text_channels_guild_info_guild_id" FOREIGN KEY (guild_id)
        REFERENCES public.guild_info (guild_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID
)

TABLESPACE pg_default;

ALTER TABLE public.text_channels
    OWNER to postgres;
