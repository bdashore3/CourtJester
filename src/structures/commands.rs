use serenity::framework::standard::macros::group;
use crate::commands::{
    other::*,
    textmod::*,
    ciphers::*,
    textchannel_send::*,
    config::*,
    support::*,
    starboard::*,
    music::*
};
use crate::helpers::voice_utils::*;

// All command groups
#[group("Master")]
#[sub_groups(General, Text, TextLast, Ciphers, TextChannelSend, Config, Support, Starboard, Voice, Music)]
pub struct Master;

#[group]
#[help_available(false)]
#[commands(ping)]
pub struct General;

#[group("Text Modification")]
#[description = "Commands than modify text. \n
Append l in the command to use the last message \n
Example: `mockl` mocks the last message"]
#[commands(mock, inv, upp, low, space, biggspace)]
pub struct Text;

#[group]
#[help_available(false)]
#[commands(mockl, invl, uppl, lowl, spacel, biggspacel)]
pub struct TextLast;

#[group("Ciphers")]
#[description = "Commands that encode/decode messages"]
#[commands(b64encode, b64decode)]
pub struct Ciphers;

#[group("Jars")]
#[description = "Commands that send certain messages to channels"]
#[commands(nice, bruh, quote)]
pub struct TextChannelSend;

#[group("Bot Configuration")]
#[description = "Admin/Moderator commands that configure the bot"]
#[commands(prefix, command)]
pub struct Config;

#[group("Support")]
#[description = "Support commands for the bot"]
#[commands(help)]
pub struct Support;

#[group("Starboard")]
#[description = "Starboard admin commands"]
#[commands(starboard)]
pub struct Starboard;

#[group("Voice")]
#[description = "Commands used for voice chat"]
#[commands(summon, disconnect)]
pub struct Voice;

#[group("Music")]
#[description = "Commands used to play music"]
#[commands(play, pause, resume, queue, skip, stop, clear, seek)]
pub struct Music;