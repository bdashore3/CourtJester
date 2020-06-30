-- Add migration script here
CREATE TABLE public.starbot
(
    guild_id bigint NOT NULL,
    reaction_message_id bigint NOT NULL,
    sent_message_id bigint NOT NULL,
    CONSTRAINT starbot_pkey PRIMARY KEY (guild_id, reaction_message_id),
    CONSTRAINT "FK_commands_guild_info_guild_id" FOREIGN KEY (guild_id)
        REFERENCES public.guild_info (guild_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE public.starbot
    OWNER to postgres;

DROP TABLE public.guild_info;

CREATE TABLE public.guild_info
(
    guild_id bigint NOT NULL,
    prefix text COLLATE pg_catalog."default",
    starbot_threshold integer,
    CONSTRAINT guild_info_pkey PRIMARY KEY (guild_id)
)

TABLESPACE pg_default;

ALTER TABLE public.guild_info
    OWNER to postgres;

DROP TABLE public.text_channels;

CREATE TABLE public.text_channels
(
    guild_id bigint NOT NULL,
    nice_id bigint,
    bruh_id bigint,
    quote_id bigint,
    CONSTRAINT text_channels_pkey PRIMARY KEY (guild_id),
    CONSTRAINT "FK_text_channels_guild_info_guild_id" FOREIGN KEY (guild_id)
        REFERENCES public.guild_info (guild_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE public.text_channels
    OWNER to postgres;