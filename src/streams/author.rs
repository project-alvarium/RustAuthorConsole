use anyhow::{Result,anyhow};
use iota_streams::app::transport::{
    TransportOptions,
    tangle::{
        client::{Client, SendTrytesOptions},
        PAYLOAD_BYTES,
        MsgId,
    }
};
use iota_streams::app_channels::api::tangle::{Address, Author, ChannelAddress, MessageContent};
use iota_streams::core::psk::PskIds;
use iota_streams::core_edsig::signature::ed25519::PublicKey;

use std::str::FromStr;
use crate::models::{Reading, Annotation};

pub struct ChannelAuthor {
    author: Author<Client>,
    announcement_id: Address,
    channel_address: ChannelAddress,
}

impl ChannelAuthor {
    pub fn new(seed: &str, mwm: u8, local_pow: bool, node: &str) -> Result<ChannelAuthor> {
        // Prepare send options for client
        let mut send_options = SendTrytesOptions::default();
        send_options.min_weight_magnitude = mwm;
        send_options.local_pow = local_pow;

        // Create Client instance
        let mut client = Client::new_from_url(node);
        client.set_send_options(send_options);

        // Generate a multi branch Author instance and start the channel
        let mut author = Author::new(seed, "utf-8", PAYLOAD_BYTES, true, client);
        let announcement_id = author.send_announce()?;

        Ok(ChannelAuthor {
            author: author,
            announcement_id: announcement_id.clone(),
            channel_address: announcement_id.appinst.clone()
        })
    }

    pub fn get_channel_address(&self) -> Result<String> {
        let channel_address = &self.channel_address.to_string();
        Ok(String::from_str(channel_address).unwrap())
    }

    pub fn get_announcement_id(&self) -> Result<(String, String)> {
        let appinst = &self.announcement_id.appinst.to_string();
        let msgid = &self.announcement_id.msgid.to_string();
        Ok((String::from_str(appinst).unwrap(), String::from_str(msgid).unwrap()))
    }

    pub fn subscribe(&mut self, link: &str, pk: &Vec<u8>) -> Result<Address> {
        match MsgId::from_str(link) {
            Ok(msgid) => {
                self.
                    author.
                    receive_subscribe(
                        &Address {
                            appinst: self.channel_address.clone(),
                            msgid,
                        })?;

                let keyload = self.author.send_keyload(
                    &self.announcement_id,
                    &PskIds::new(),
                    &vec![PublicKey::from_bytes(pk).unwrap()]
                )?;

                // Return the sequence message link
                Ok(keyload.1.unwrap())
            },
            Err(_) => {
                Err(anyhow!("Error getting msgid from provided link: {}", link))
            }
        }
    }

    pub fn get_next_msgs(&mut self) -> Result<Vec<(Option<Reading>, Option<Annotation>)>> {
        let mut found_msgs = Vec::new();

        let response = self.author.fetch_next_msgs();
        for msg in response {
            match msg.body {
                MessageContent::SignedPacket {pk: _, public_payload: _, masked_payload: m} => {
                    let reading: serde_json::Result<Reading> = serde_json::from_slice(&m.0);
                    match reading {
                        Ok(r) => found_msgs.push((Some(r), None)),
                        Err(_) => {
                            let annotation: serde_json::Result<Annotation> = serde_json::from_slice(&m.0);
                            match annotation {
                                Ok(a) => found_msgs.push((None, Some(a))),
                                Err(_) => { println!("Error deserializing message") }
                            };
                        }
                    }
                }
                _ => println!("Message type not supported")
            }
        }

        Ok(found_msgs)
    }
}
