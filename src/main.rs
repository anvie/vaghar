// Copyright (C) 2022 Neuversity
// All Rights Reserved.
//
// NOTICE: All information contained herein is, and remains
// the property of Neuversity.
// The intellectual and technical concepts contained
// herein are proprietary to Neuversity
// and are protected by trade secret or copyright law.
// Dissemination of this information or reproduction of this material
// is strictly forbidden unless prior written permission is obtained
// from Neuversity.

use permutator::{CartesianProduct, Combination, Permutation};

use bip39::{Language, Mnemonic, MnemonicType, Seed};
use clap::Parser;
use rayon::iter::{ParallelBridge, ParallelIterator};
use serde::{Deserialize, Serialize};

use secp256k1_17::key::SecretKey;
use wagyu_ethereum::format::EthereumFormat;
use wagyu_ethereum::{address::EthereumAddress, private_key::EthereumPrivateKey};
use wagyu_model::{PrivateKey, PublicKey};

use rand::{seq::SliceRandom, thread_rng};
use std::sync::{Mutex, RwLock};
use std::{
    collections::HashMap,
    fs,
    process::exit,
    time::{Duration, Instant},
};

mod bip32;

use bip32::{Bip44DerivationPath, HDPrivKey};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long, default_value = "default.conf")]
    config: String,
}

#[derive(Deserialize, Debug)]
struct Config {
    words1: String,
    words2: String,
    words3: String,
    words1_needed: String,
    words2_needed: String,
    words3_needed: String,
    target: String,
}

struct Tokenizer {
    word_map: HashMap<String, u32>,
}

impl Tokenizer {
    fn new() -> Self {
        Self {
            word_map: HashMap::new(),
        }
    }

    fn tokenize(&mut self, words: Vec<&str>) -> Vec<u32> {
        let mut tokens = Vec::new();
        for word in words {
            let count = self.word_map.len() as u32;
            let token = self.word_map.entry(word.to_string()).or_insert_with(|| {
                let token = count + 1;
                token
            });
            tokens.push(*token);
        }
        tokens
    }

    /// Convert tokens to words
    fn to_words(&self, tokens: &Vec<&u32>) -> Vec<String> {
        let mut words = Vec::new();
        for token in tokens {
            let word = self.word_map.iter().find(|(_, &v)| v == **token).unwrap().0;
            words.push(word.to_string());
        }
        words
    }
}

fn main() {
    let args = Args::parse();
    println!("Value for config: {}", args.config);

    let config: Config = match fs::read_to_string(args.config) {
        Ok(config) => toml::from_str(&config).unwrap(),
        Err(e) => panic!("Error reading config file: {}", e),
    };

    println!("Config: {:#?}", config);

    // exit(0);

    let words1 = config.words1.split_whitespace().collect::<Vec<&str>>();
    let words2 = config.words2.split_whitespace().collect::<Vec<&str>>();
    let words3 = config.words3.split_whitespace().collect::<Vec<&str>>();

    let mut tokenizer = Tokenizer::new();
    let tokens1 = tokenizer.tokenize(words1);
    let tokens2 = tokenizer.tokenize(words2);
    let tokens3 = tokenizer.tokenize(words3);

    let mut tokens_all = tokens1
        .clone()
        .into_iter()
        .chain(tokens2.clone().into_iter())
        .collect::<Vec<u32>>();
    tokens_all.sort();
    tokens_all.dedup();
    // tokens_all.shuffle(&mut thread_rng());
    println!("tokens_all: {:?}", tokens_all);
    let target = config.target.to_lowercase();

    // println!("Tokens1: {:#?}", tokens1);
    // println!("Tokens2: {:#?}", tokens2);

    // let tokens1 = to_tokens(words1);

    // println!("Words1: {:#?}", words1);
    // println!("Words2: {:#?}", words2);

    // let mut tokens1 = &[1, 2, 3, 4, 5, 6];

    let tokens1_needed = tokenizer.tokenize(
        config
            .words1_needed
            .split_whitespace()
            .collect::<Vec<&str>>(),
    );
    let tokens2_needed = tokenizer.tokenize(
        config
            .words2_needed
            .split_whitespace()
            .collect::<Vec<&str>>(),
    );
    let tokens3_needed = tokenizer.tokenize(
        config
            .words3_needed
            .split_whitespace()
            .collect::<Vec<&str>>(),
    );

    // let target:EthereumAddress = "0x2EA4400F5C66B7B810D46d2F5649966D64a402dB".parse().unwrap();
    let target: EthereumAddress = target.parse().unwrap();

    let counter = RwLock::new(0);
    let lasttime = Mutex::new(Instant::now());
    let calc_per_second = Mutex::new(0);
    tokens1.combination(4).par_bridge().for_each(|mut c| {
        c.permutation().for_each(|p| {
            // if !p.iter().take(6).all(|item| tokens1.contains(item))
            //     || !p.iter().skip(6).all(|item| tokens2.contains(item))
            // {
            //     // println!(
            //     //     "Not all tokens2 in last 6: {}",
            //     //     tokenizer
            //     //         .to_words(&p.clone().into_iter().skip(6).collect::<Vec<&u32>>())
            //     //         .join(" ")
            //     // );
            //     {
            //         let mut calc_per_second = calc_per_second.lock().unwrap();
            //         *calc_per_second += 1;

            //         let mut lasttime = lasttime.lock().unwrap();
            //         if lasttime.elapsed() > Duration::from_secs(1) {
            //             *lasttime = Instant::now();
            //             let counter = counter.read().unwrap();
            //             let passphrase = tokenizer.to_words(&p).join(" ");
            //             println!("speed: {0:}/s - last: {2:} - processed: {1: <10}", calc_per_second, *counter, passphrase);
            //             *calc_per_second = 0;
            //         }
            //     }
            //     return;
            // }

            // if !p.iter().all(|item| tokens1_needed.contains(item)){
            if tokens1_needed.iter().all(|item| !p.contains(&item)) {
                println!("ignored1: {}", tokenizer.to_words(&p).join(" "));
                return;
            }

            // let passphrase = tokenizer.to_words(&p).join(" ");

            tokens2.combination(4).par_bridge().for_each(|mut c2| {
                c2.permutation().for_each(|p2| {
                    if !p2.iter().any(|item| tokens2_needed.contains(item)) {
                        println!("ignored2: {}", tokenizer.to_words(&p2).join(" "));
                        return;
                    }

                    tokens3.combination(4).par_bridge().for_each(|mut c3| {
                        c3.permutation().for_each(|p3| {
                            if !p3.iter().any(|item| tokens3_needed.contains(item)) {
                                // println!("ignored3: {}", tokenizer.to_words(&p3).join(" "));
                                return;
                            }

                            let passphrase = format!(
                                "{} {} {}",
                                tokenizer.to_words(&p).join(" "),
                                tokenizer.to_words(&p2).join(" "),
                                tokenizer.to_words(&p3).join(" "),
                            );
                            // println!("{}", passphrase);

                            // validate the passphrase
                            match Mnemonic::validate(&passphrase, Language::English) {
                                Ok(()) => {
                                    // println!("VALID {}", passphrase);

                                    let mnemonic =
                                        Mnemonic::from_phrase(&passphrase, Language::English)
                                            .unwrap();
                                    let seed = Seed::new(&mnemonic, "");

                                    let seed = HDPrivKey::new(&seed.as_bytes()).unwrap();
                                    let derived = seed
                                        .derive(Bip44DerivationPath {
                                            coin_type: 60,
                                            account: 0,
                                            change: Some(0),
                                            address_index: Some(0),
                                        })
                                        .unwrap();
                                    let secp_key =
                                        SecretKey::from_slice(&derived.key_part()).unwrap();

                                    let ethereum_private_key =
                                        EthereumPrivateKey::from_secp256k1_secret_key(secp_key);
                                    let address = ethereum_private_key
                                        .to_address(&EthereumFormat::Standard)
                                        .unwrap();

                                    // println!("{} {}\n   - {}", counter.read().unwrap(), address, passphrase);

                                    if address == target {
                                        let counter = counter.read().unwrap();
                                        println!("FOUND! {} {}", counter, passphrase);
                                        exit(0);
                                    }
                                }
                                Err(err) => (),
                            }

                            // increase counter
                            {
                                let mut counter = counter.write().unwrap();
                                *counter += 1;
                            }

                            {
                                let mut calc_per_second = calc_per_second.lock().unwrap();
                                *calc_per_second += 1;

                                let mut lasttime = lasttime.lock().unwrap();
                                if lasttime.elapsed() > Duration::from_secs(1) {
                                    *lasttime = Instant::now();
                                    let counter = counter.read().unwrap();
                                    println!(
                                        "speed: {0:}/s - last: {2:} - processed: {1: <10}",
                                        calc_per_second, *counter, passphrase
                                    );
                                    *calc_per_second = 0;
                                }
                            }
                        });
                    });
                });
            });

            // counter += 1;
        });
    });
    let counter = counter.read().unwrap();
    println!("Total permutations: {}", *counter - 1);

    // let words1_back = tokenizer.to_words(tokens1);
    // let words2_back = tokenizer.to_words(tokens2);

    // println!("Words1: {:#?}", words1_back);
    // println!("Words2: {:#?}", words2_back);
}
