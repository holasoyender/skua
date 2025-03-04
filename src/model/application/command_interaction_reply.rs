use crate::all::{Builder, ButtonStyle, CacheHttp, CreateActionRow, CreateAllowedMentions, CreateAttachment, CreateButton, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage, InteractionId, InteractionResponseFlags};
use crate::model::emoji::Emoji;

#[derive(Clone, Debug, Serialize)]
pub struct CommandInteractionReply {
    #[serde(skip_serializing_if = "Option::is_none")]
    tts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    embeds: Option<Vec<CreateEmbed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_mentions: Option<CreateAllowedMentions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<InteractionResponseFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<Vec<CreateActionRow>>,
    attachments: Vec<CreateAttachment>,
    id: InteractionId,
    token: String,
    pub(crate) deferred: bool
}

impl CommandInteractionReply {
    pub fn emoji(mut self, emoji: Emoji) -> Self {
        self.content = Some(emoji.format());
        self
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(self.content.unwrap_or_default() + &*content.into());
        self
    }

    pub fn tts(mut self, tts: bool) -> Self {
        self.tts = Some(tts);
        self
    }

    pub fn embed(mut self, embed: CreateEmbed) -> Self {
        self.embeds.get_or_insert_with(Vec::new).push(embed.into());
        self
    }

    pub fn embeds(mut self, embeds: Vec<CreateEmbed>) -> Self {
        self.embeds.get_or_insert_with(Vec::new).extend(embeds);
        self
    }

    pub fn allowed_mentions(mut self, allowed_mentions: CreateAllowedMentions) -> Self {
        self.allowed_mentions = Some(allowed_mentions);
        self
    }

    pub fn ephemeral(mut self) -> Self {
        self.flags = Some(InteractionResponseFlags::EPHEMERAL);
        self
    }

    pub fn suppress_embeds(mut self) -> Self {
        self.flags = Some(InteractionResponseFlags::SUPPRESS_EMBEDS);
        self
    }

    pub fn components(mut self, components: Vec<CreateActionRow>) -> Self {
        self.components = Some(components);
        self
    }

    pub fn component(mut self, component: CreateActionRow) -> Self {
        self.components.get_or_insert_with(Vec::new).push(component);
        self
    }

    pub fn link_button(mut self, _label: impl Into<String>, url: impl Into<String>, emoji: Option<Emoji>, disabled: bool) -> Self {
        let empty_vec: Vec<CreateActionRow> = Vec::new();
        let current_action_rows = match &self.components {
            Some(rows) => rows,
            None => {
                &empty_vec
            }
        };

        let label = _label.into();

        if current_action_rows.is_empty() {
            let mut button = CreateButton::new_link(url).disabled(disabled);
            if let Some(emoji) = emoji {
                button = button.emoji(emoji.reaction());
            }

            if label.len() > 0 {
                button = button.label(label);
            }


            self.components = Some(vec![
                CreateActionRow::Buttons(vec![button])
            ]);
        } else {
            let mut rows = current_action_rows.clone();
            let last_action_row = rows.last_mut();

            if let Some(CreateActionRow::Buttons(buttons)) = last_action_row {

                let mut button = CreateButton::new_link(url).disabled(disabled);
                if let Some(emoji) = emoji {
                    button = button.emoji(emoji.reaction());
                }

                if label.len() > 0 {
                    button = button.label(label);
                }

                if buttons.len() >= 5 {
                    self.components.get_or_insert_with(Vec::new).push(CreateActionRow::Buttons(vec![button]));
                } else {
                    buttons.push(button);
                    self.components.get_or_insert_with(Vec::new).pop();
                    self.components.get_or_insert_with(Vec::new).push(CreateActionRow::Buttons(buttons.clone()));
                }
            } else {
                if current_action_rows.len() >= 5 {
                    panic!("Too many action rows, a maximum of 5 is allowed");
                } else {
                    let mut button = CreateButton::new_link(url).disabled(disabled);
                    if let Some(emoji) = emoji {
                        button = button.emoji(emoji.reaction());
                    }

                    if label.len() > 0 {
                        button = button.label(label);
                    }

                    self.components = Some(vec![
                        CreateActionRow::Buttons(vec![button])
                    ]);
                }
            }
        }

        self
    }

    pub fn button(mut self, _label: impl Into<String>, _id: impl Into<String>, emoji: Option<Emoji>, disabled: bool, button_style: ButtonStyle) -> Self {
        let empty_vec: Vec<CreateActionRow> = Vec::new();
        let current_action_rows = match &self.components {
            Some(rows) => rows,
            None => {
                &empty_vec
            }
        };

        let id = _id.into();
        let label = _label.into();

        if current_action_rows.is_empty() {
            let mut button = CreateButton::new(id)
                .style(button_style)
                .disabled(disabled);
            if let Some(emoji) = emoji {
                button = button.emoji(emoji.reaction());
            }

            if label.len() > 0 {
                button = button.label(label);
            }

            self.components = Some(vec![
                CreateActionRow::Buttons(vec![button])
            ]);
        } else {
            let mut rows = current_action_rows.clone();
            let last_action_row = rows.last_mut();

            if let Some(CreateActionRow::Buttons(buttons)) = last_action_row {

                let mut button = CreateButton::new(id)
                    .style(button_style)
                    .disabled(disabled);
                if let Some(emoji) = emoji {
                    button = button.emoji(emoji.reaction());
                }

                if label.len() > 0 {
                    button = button.label(label);
                }

                if buttons.len() >= 5 {
                    self.components.get_or_insert_with(Vec::new).push(CreateActionRow::Buttons(vec![button]));
                } else {
                    buttons.push(button);
                    self.components.get_or_insert_with(Vec::new).pop();
                    self.components.get_or_insert_with(Vec::new).push(CreateActionRow::Buttons(buttons.clone()));
                }
            } else {
                if current_action_rows.len() >= 5 {
                    panic!("Too many action rows, a maximum of 5 is allowed");
                } else {
                    let mut button = CreateButton::new(id)
                        .style(button_style)
                        .disabled(disabled);
                    if let Some(emoji) = emoji {
                        button = button.emoji(emoji.reaction());
                    }

                    if label.len() > 0 {
                        button = button.label(label);
                    }

                    self.components = Some(vec![
                        CreateActionRow::Buttons(vec![button])
                    ]);
                }
            }
        }

        self
    }

    pub fn primary_button(mut self, _label: impl Into<String>, _id: impl Into<String>, emoji: Option<Emoji>, disabled: bool) -> Self {
        self.button(_label, _id, emoji, disabled, ButtonStyle::Primary)
    }

    pub fn secondary_button(mut self, _label: impl Into<String>, _id: impl Into<String>, emoji: Option<Emoji>, disabled: bool) -> Self {
        self.button(_label, _id, emoji, disabled, ButtonStyle::Secondary)
    }

    pub fn success_button(mut self, _label: impl Into<String>, _id: impl Into<String>, emoji: Option<Emoji>, disabled: bool) -> Self {
        self.button(_label, _id, emoji, disabled, ButtonStyle::Success)
    }

    pub fn danger_button(mut self, _label: impl Into<String>, _id: impl Into<String>, emoji: Option<Emoji>, disabled: bool) -> Self {
        self.button(_label, _id, emoji, disabled, ButtonStyle::Danger)
    }

    /*pub fn attachments(mut self, attachments: EditAttachments) -> Self {
        self.attachments = attachments;
        self
    }

    pub fn attachment(mut self, attachment: EditAttachments) -> Self {
        self.attachments = attachment;
        self
    }*/

    pub fn add_file(mut self, file: CreateAttachment) -> Self {
        self.attachments = self.attachments
            .into_iter()
            .chain(std::iter::once(file))
            .collect();
        self
    }

    pub fn add_files(mut self, files: impl IntoIterator<Item = CreateAttachment>) -> Self {
        self.attachments = self.attachments
            .into_iter()
            .chain(files)
            .collect();
        self
    }

    pub async fn send(self, http_cache: impl CacheHttp) -> crate::Result<()> {

        if self.content.is_none() && self.embeds.is_none() && self.components.is_none() {
            return Err(crate::Error::Other("No content, embeds, or components provided"));
        }

        if self.deferred {
            let mut builder = CreateInteractionResponseFollowup::default();

            if let Some(content) = self.content {
                builder = builder.content(content);
            }
            if let Some(tts) = self.tts {
                builder = builder.tts(tts);
            }
            if let Some(embeds) = self.embeds {
                builder = builder.embeds(embeds);
            }
            if let Some(allowed_mentions) = self.allowed_mentions {
                builder = builder.allowed_mentions(allowed_mentions);
            }
            if let Some(components) = self.components {
                builder = builder.components(components);
            }

            builder = builder.add_files(self.attachments);

            builder.execute(http_cache, (None, &self.token)).await?;
            Ok(())
        } else {

            let mut message = CreateInteractionResponseMessage::default();
            if let Some(content) = self.content {
                message = message.content(content);
            }
            if let Some(tts) = self.tts {
                message = message.tts(tts);
            }
            if let Some(embeds) = self.embeds {
                message = message.embeds(embeds);
            }
            if let Some(allowed_mentions) = self.allowed_mentions {
                message = message.allowed_mentions(allowed_mentions);
            }
            if let Some(flags) = self.flags {
                message = message.flags(flags);
            }
            if let Some(components) = self.components {
                message = message.components(components);
            }

            message = message.add_files(self.attachments);

            let builder = CreateInteractionResponse::Message(message);
            builder.execute(http_cache, (self.id, &self.token)).await
        }
    }
}