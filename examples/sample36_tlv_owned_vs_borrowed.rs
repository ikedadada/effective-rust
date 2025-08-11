#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

#[allow(unused)]
fn main1() {
    // A type-length-value (TLV) from a data stream.
    #[derive(Clone, Debug)]
    struct Tlv<'a> {
        type_code: u8,
        value: &'a [u8],
    }

    type Error = &'static str; // Some local error type.

    /// Extract the next TLV from the 'input', also returning the remaining unprocessed data.
    #[allow(mismatched_lifetime_syntaxes)]
    fn get_next_tlv(input: &[u8]) -> Result<(Tlv, &[u8]), Error> {
        if input.len() < 2 {
            return Err("Input too short for TLV");
        }
        // The TL parts of the TLV are one byte each.
        let type_code = input[0];
        let len = input[1] as usize;
        if 2 + len > input.len() {
            return Err("Input too short for TLV value");
        }

        let tlv = Tlv {
            type_code,
            // Reference the relevant chunk of input data.
            value: &input[2..2 + len],
        };
        Ok((tlv, &input[2 + len..]))
    }

    // 特定の場合にはうまくいくがこのデータ構造のインスタンスを保持し続けようとすると
    // 面倒なことになる

    struct NetworkServer<'a> {
        // ...
        // Most recent max-size message.
        max_size: Option<Tlv<'a>>,
    }

    const SET_MAX_SIZE: u8 = 0x01; // Type code for max-size TLV.

    // このコードはコンパイルできるが、事実上使用できない
    impl<'a> NetworkServer<'a> {
        fn process(&mut self, mut data: &'a [u8]) -> Result<(), Error> {
            while !data.is_empty() {
                let (tlv, rest) = get_next_tlv(data)?;
                match tlv.type_code {
                    SET_MAX_SIZE => {
                        // Save off the most recent 'SET_MAX_SIZE' message.
                        self.max_size = Some(tlv);
                    }
                    // (Deal with other message types)
                    _ => return Err("Unknown message type"),
                }
                data = rest;
            }
            Ok(())
        }
    }

    // ✖ 長く生きるべきサーバーに短命なデータの生存期間が紐づけられるためコンパイルできない
    // let mut server = NetworkServer { max_size: None };
    // while !server.done() {
    //     // Read data into a fresh vector.
    //     let data: Vec<u8> = read_data_from_socket();
    //     if let Err(e) = server.process(&data) {
    //         eprintln!("Error processing data: {}", e);
    //         // Handle error (e.g., log it, send a response, etc.)
    //     }
    // }

    // 長く生存するバッファを再利用するようにコードを書き換えてもうまくいかない
    // 今度はコンパイラは、バッファへの参照を保持したまま、同じバッファへの可変参照を渡そうとしているとして、エラーを出す。
    // let mut perma_buffer = [0u8; 256];
    // // lifetime within `perma_buffer`
    // let mut server = NetworkServer { max_size: None };
    // while !server.done() {
    //     // Reuse the same buffer for the next load of data.
    //     read_data_into_buffer(&mut perma_buffer);
    //     if let Err(e) = server.process(&perma_buffer) {
    //         eprintln!("Error processing data: {}", e);
    //         // Handle error (e.g., log it, send a response, etc.)
    //     }
    // }

    // 問題の本質は、Tlvが一時的なデータへの参照を保持していることである。これは一時的な処理には問題ないが、
    // 状態を後のために取っておくという行為とは本質的に相いれない
}

#[allow(unused)]
fn main2() {
    // A type-length-value (TLV) from a data stream.
    #[derive(Clone, Debug)]
    struct Tlv {
        type_code: u8,
        value: Vec<u8>, // owned heap data
    }

    type Error = &'static str; // Some local error type.

    fn get_next_tlv(input: &[u8]) -> Result<(Tlv, &[u8]), Error> {
        if input.len() < 2 {
            return Err("Input too short for TLV");
        }
        // The TL parts of the TLV are one byte each.
        let type_code = input[0];
        let len = input[1] as usize;
        if 2 + len > input.len() {
            return Err("Input too short for TLV value");
        }

        let tlv = Tlv {
            type_code,
            // Reference the relevant chunk of input data.
            value: input[2..2 + len].to_vec(),
        };
        Ok((tlv, &input[2 + len..]))
    }

    // Now we can use this in a long-lived server.
    struct NetworkServer {
        // ...
        // Most recent max-size message.
        max_size: Option<Tlv>,
    }

    const SET_MAX_SIZE: u8 = 0x01; // Type code for max-size TLV.

    impl NetworkServer {
        fn process(&mut self, mut data: &[u8]) -> Result<(), Error> {
            while !data.is_empty() {
                let (tlv, rest) = get_next_tlv(data)?;
                match tlv.type_code {
                    SET_MAX_SIZE => {
                        // Save off the most recent 'SET_MAX_SIZE' message.
                        self.max_size = Some(tlv);
                    }
                    // (Deal with other message types)
                    _ => return Err("Unknown message type"),
                }
                data = rest;
            }
            Ok(())
        }
        fn done(&self) -> bool {
            // Placeholder for actual logic to determine if the server is done.
            false
        }
    }

    fn read_data_from_socket() -> Vec<u8> {
        // Placeholder for actual socket reading logic.
        vec![0x01, 0x02, 0x03, 0x04] // Example data
    }

    // Now this code compiles and works as expected.
    let mut server = NetworkServer { max_size: None };
    while !server.done() {
        // Read data into a fresh vector.
        let data: Vec<u8> = read_data_from_socket();
        if let Err(e) = server.process(&data) {
            eprintln!("Error processing data: {}", e);
            // Handle error (e.g., log it, send a response, etc.)
        }
    }
}

fn main() {
    // main1() // Uncomment to run the first example
    main2();
}
