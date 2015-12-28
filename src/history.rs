// Copyright (c) 2010-2013, Susumu Yata
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// - Redistributions of source code must retain the above copyright notice, this
//   list of conditions and the following disclaimer.
// - Redistributions in binary form must reproduce the above copyright notice,
//   this list of conditions and the following disclaimer in the documentation
//   and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

use std;
use base::*;

struct History {
    node_id_: u32,
    louds_pos_: u32,
    key_pos_: u32,
    link_id_: u32,
    key_id_: u32,
}

impl History {
    fn new() -> History {
        History { node_id_: 0, louds_pos_: 0, key_pos_: 0,
                  link_id_: INVALID_LINK_ID, key_id_: INVALID_KEY_ID }
    }
    fn set_node_id(&mut self, node_id: usize) {
        assert!(node_id <= std::u32::MAX as usize, "MARISA_SIZE_ERROR");
        self.node_id_ = node_id as u32;
    }
    fn set_louds_pos(&mut self, louds_pos: usize) {
        assert!(louds_pos <= std::u32::MAX as usize, "MARISA_SIZE_ERROR");
        self.louds_pos_ = louds_pos as u32;
    }
    fn set_key_pos(&mut self, key_pos: usize) {
        assert!(key_pos <= std::u32::MAX as usize, "MARISA_SIZE_ERROR");
        self.key_pos_ = key_pos as u32;
    }
    fn set_link_id(&mut self, link_id: usize) {
        assert!(link_id <= std::u32::MAX as usize, "MARISA_SIZE_ERROR");
        self.link_id_ = link_id as u32;
    }
    fn set_key_id(&mut self, key_id: usize) {
        assert!(key_id <= std::u32::MAX as usize, "MARISA_SIZE_ERROR");
        self.key_id_ = key_id as u32;
    }
    fn node_id(&self) -> usize {
        self.node_id_ as usize
    }
    fn louds_pos(&self) -> usize {
        self.louds_pos_ as usize
    }
    fn key_pos(&self) -> usize {
        self.key_pos_ as usize
    }
    fn link_id(&self) -> usize {
        self.link_id_ as usize
    }
    fn key_id(&self) -> usize {
        self.key_id_ as usize
    }
}

pub struct State {
    key_buf_: Vec<u8>,
    history_: Vec<History>,
    node_id_: u32,
    query_pos_: u32,
    history_pos_: u32,
}

impl State {
    pub fn new() -> State {
        State { key_buf_: Vec::new(), history_: Vec::new(), node_id_: 0,
                query_pos_: 0, history_pos_: 0, }
    }

    fn set_node_id(&mut self, node_id: usize) {
        assert!(node_id <= std::u32::MAX as usize, "MARISA_SIZE_ERROR");
        self.node_id_ = node_id as u32;
    }
    fn set_query_pos(&mut self, query_pos: usize) {
        assert!(query_pos <= std::u32::MAX as usize, "MARISA_SIZE_ERROR");
        self.query_pos_ = query_pos as u32;
    }
    fn set_history_pos(&mut self, history_pos: usize) {
        assert!(history_pos <= std::u32::MAX as usize, "MARISA_SIZE_ERROR");
        self.history_pos_ = history_pos as u32;
    }

    fn get_node_id(&self) -> usize {
        self.node_id_ as usize
    }
    fn get_query_pos(&self) -> usize {
        self.query_pos_ as usize
    }
    fn get_history_pos(&self) -> usize {
        self.history_pos_ as usize
    }

    fn reset(&mut self) {
        *self = State::new();
    }

/*
    fn lookup_init(&mut self) {
        self.node_id_ = 0;
        self.query_pos_ = 0;
    }
    fn reverse_lookup_init(&mut self) {
        self.key_buf_ = Vec::with_capacity(32);
    }
    fn common_prefix_search_init(&mut self) {
        self.node_id_ = 0;
        self.query_pos_ = 0;
    }
    fn predictive_search_init(&mut self) {
        self.key_buf_ = Vec::with_capacity(64);
        self.history_ = Vec::with_capacity(4);
        self.node_id_ = 0;
        self.query_pos_ = 0;
        self.history_pos_ = 0;
    }
*/
}

