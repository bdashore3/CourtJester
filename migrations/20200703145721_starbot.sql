-- Add migration script here
CREATE TABLE public.starbot
(
    guild_id bigint NOT NULL,
    reaction_message_id bigint NOT NULL,
    sent_message_id bigint NOT NULL,
    CONSTRAINT starbot_pkey PRIMARY KEY (guild_id, reaction_message_id),
    CONSTRAINT "FK_starboard_guild_info_guild_id" FOREIGN KEY (guild_id)
        REFERENCES public.guild_info (guild_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE public.starbot
    OWNER to postgres;

ALTER TABLE public.guild_info
    ADD COLUMN starbot_threshold integer;

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
